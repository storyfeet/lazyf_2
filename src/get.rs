
use crate::lz_err::LzErr;
use crate::env;
use std::str::FromStr;
use std::path::{PathBuf,Path};

pub trait Getable{
    fn get(&self,_:&str)->Option<String>;
    /// Present exists to make it possible to check if a non key-value flag like "--help" is marked
    fn is_present(&self,s:&str)->bool{
        self.get(s).is_some()
    }

    fn localize(&self,p:&Path)->PathBuf{
        PathBuf::from(p)
    }
}

#[derive(Clone,Copy)]
pub enum Getter{
}

/// The GetHolder does the work of maintaining a list of Getables, and
/// prioritizing user requests.
///
/// Normally you will not instantiate this directly, prefering to use the 
/// ``` config() ``` method to create it.
///
/// However it can be instatiated to use anything that implements Getable
/// (from this module)
///
/// For example
///
///
///
/// ```
/// use lazy_conf::{Getable,GetHolder,GetMode};
/// use std::collections::BTreeMap;
/// 
/// struct GetTester(BTreeMap<String,String>);
///
/// impl Getable for GetTester{
///   fn get(&self,s:&str)->Option<String>{
///       self.0.get(s).map(|s|s.to_string())
///    }
/// }
///
/// let mut g = GetHolder::new();
/// let mut t1 = BTreeMap::new();
/// t1.insert("a".to_string(),"a_t1_res".to_string());
/// let mut t2 = BTreeMap::new();
/// t2.insert("a".to_string(),"a_t2_res".to_string());
/// t2.insert("num".to_string(),"4".to_string());
/// g.add(GetMode::Conf,GetTester(t1));
/// g.add(GetMode::Flag,GetTester(t2));
///
/// assert_eq!(&g.grab().cf("a").fg("a").s().unwrap(),"a_t1_res");
/// assert_eq!(&g.grab().fg("a").cf("a").s().unwrap(),"a_t2_res");
/// assert_eq!(g.grab().cf("num").fg("num").t::<i32>().unwrap(),4);
/// assert!(g.grab().s().is_none());
/// assert!(g.grab().fg("b").cf("b").s().is_none());
///
/// ```
pub struct GetHolder {    
    pub v:Vec<(GetMode,Box<Getable>)>,
    pub help_mess:String,
    pub fails:String,
}


impl GetHolder{
    pub fn new()->Self{
        GetHolder{v:Vec::new(),help_mess:String::new(),fails:String::new()}
    }

    pub fn add<T:'static+Getable>(&mut self,g:GetMode,gt:T){
        self.v.push((g,Box::new(gt)));
    }

    pub fn grab<'a>(&'a mut self)->Grabber<'a>{ 
        Grabber{
            v:Vec::new(),
            h:self,
        }
    }

    pub fn add_fail(&mut self,f_str:&str){
        self.fails.push_str(f_str);
        self.fails.push('\n');
    }

    pub fn get(&self,g:GetMode,s:&str)->Option<String>{
        for (gm,v) in &self.v {
            if *gm == g {
                if let Some(s) =  v.get(s){
                    return Some(s);
                }
            }
        }
        None
    }

    pub fn get_local(&self,g:GetMode,s:&str)->Option<PathBuf>{
        for (gm,v) in &self.v{
            if *gm != g{continue}
            if let Some(s) = v.get(s){
                return match env::replace_env(&s){
                    Ok(rp)=>Some(v.localize(rp.as_ref())),
                    _=>Some(v.localize(s.as_ref())),
                };
            }
        }
        None
    }

    pub fn is_present(&self,g:GetMode,s:&str)->bool{
        for (gm,v) in &self.v{
            if *gm == g{
                if v.is_present(s){
                    return true;
                }
            }
        }
        return false;
    }

    pub fn add_help(&mut self,mess:&str){
        self.help_mess.push_str(mess);
        self.help_mess.push('\n');//CONSIDER windows at some point
    }

    pub fn help_string(&self,mess:&str)->String{
        if self.fails.len() > 0 {
            return format!("{}\nMissing:\n{}\n{}",mess,self.fails,self.help_mess);
        }
        format!("{}\n{}\n",mess,self.help_mess)
    }

    /// Checks for a --help flag, and prints the built up help message to stdout, returns true if
    pub fn help(&self,mess:&str)->bool{
        if self.is_present(GetMode::Flag,"--help"){
            println!("{}",self.help_string(mess));
            return true;
        }
        if self.fails.len() >0 {
            println!("{}",self.help_string(mess));
            return true;
        }
        false
    }
}

pub struct Grabber<'a>{ //get builder
    v:Vec<(GetMode,&'a str)>,
    h:&'a mut GetHolder,
}

impl<'a> Grabber<'a>{
    pub fn env(self,s:&'a str)->Self{
        self.gm(GetMode::Env,s)
    }
    pub fn cf(self,s:&'a str)->Self{
        self.gm(GetMode::Conf,s)
    }
    pub fn fg(self,s:&'a str)->Self{
        self.gm(GetMode::Flag,s)
    }
    fn gm(mut self,g:GetMode,s:&'a str)->Self{
        self.v.push((g,s));
        self
    }

    pub fn is_present(self)->bool{
        for (m,st) in self.v{
            if self.h.is_present(m,st){
                return true;
            }
        }
        false
    }

    fn _s(&self)->Option<String>{
        for (m,st) in &self.v{
            if let Some(v)=  self.h.get(*m,st){
                return Some(v);
            }
        }
        None
    }

    pub fn s(self)->Option<String>{
        self._s()
    }

    pub fn s_local(self)->Option<PathBuf>{
        for (m,st) in &self.v{
            if let Some(v)= self.h.get_local(*m,st){
                return Some(v)
            }
        }
        None
    }

    pub fn s_req(self,mess:&str)->Option<String>{
        let hs = &self.help_str(mess);
        match self._s(){
            Some(s)=>{
                self.h.add_help(&hs);
                Some(s)
            },
            None=>{
                self.h.add_fail(&hs);
                None
            },
        }
    }

    pub fn t<T:FromStr>(self)->Result<T,LzErr>{
        let s = self._s().ok_or(LzErr::NotFound)?;
        s.parse().map_err(|_|LzErr::ParseErr)
    }

    pub fn t_req<T:FromStr>(self,mess:&str)->Result<T,LzErr>{
        let s = self._s()
                .ok_or(LzErr::NotFound)
                .and_then(|x|x.parse().map_err(|_|LzErr::ParseErr));
        let hs = self.help_str(mess);

        match s{
            Ok(s)=>{
                self.h.add_help(&hs);
                Ok(s)
            },
            Err(e)=>{
                self.h.add_fail(&hs);
                Err(e) 
            },
        }
    }

    pub fn help(self,mess:&str)->Self{
        let hs = self.help_str(mess);
        self.h.add_help(&hs);
        self
    }

    pub fn help_str(&self,mess:&str)->String{
        let mut s = format!("{}:\n",mess );
        for (m,v) in &self.v{
            s.push_str(&format!("\t{:?}:{},",m,v));
        }
        s
    }

}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum GetMode{
    Flag,
    Env,
    Conf,
}


#[cfg(test)]
mod test{
    use super::*;
    use std::collections::BTreeMap;
    
    struct GetTester(BTreeMap<String,String>);

    impl Getable for GetTester{
        fn get(&self,s:&str)->Option<String>{
            self.0.get(s).map(|s|s.to_string())
        }
    }


    #[test]
    fn grab_test(){
        let mut g = GetHolder::new();
        let mut t1 = BTreeMap::new();
        t1.insert("a".to_string(),"a_t1_res".to_string());
        let mut t2 = BTreeMap::new();
        t2.insert("a".to_string(),"a_t2_res".to_string());
        t2.insert("num".to_string(),"4".to_string());
        g.add(GetMode::Conf,GetTester(t1));
        g.add(GetMode::Flag,GetTester(t2));

        assert_eq!(&g.grab().cf("a").fg("a").s().unwrap(),"a_t1_res");
        assert_eq!(&g.grab().fg("a").cf("a").s().unwrap(),"a_t2_res");
        assert_eq!(g.grab().cf("num").fg("num").t::<i32>().unwrap(),4);
        assert!(g.grab().s().is_none());
        assert!(g.grab().fg("b").cf("b").s().is_none());
    }
}
