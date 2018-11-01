
use lz_err::LzErr;
use std::str::FromStr;

pub trait Getable{
    fn get(&self,&str)->Option<String>;
}

#[derive(Clone,Copy)]
pub enum Getter{
}

pub struct GetHolder {    
    pub v:Vec<(GetMode,Box<Getable>)>,
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
/// g.add(GetMode::Lz,GetTester(t1));
/// g.add(GetMode::Flag,GetTester(t2));
///
/// assert_eq!(&g.grab().lz("a").fg("a").s().unwrap(),"a_t1_res");
/// assert_eq!(&g.grab().fg("a").lz("a").s().unwrap(),"a_t2_res");
/// assert_eq!(g.grab().lz("num").fg("num").t::<i32>().unwrap(),4);
/// assert!(g.grab().s().is_none());
/// assert!(g.grab().fg("b").lz("b").s().is_none());
///
/// ```
impl GetHolder{
    pub fn new()->Self{
        GetHolder{v:Vec::new()}
    }

    pub fn add<T:'static+Getable>(&mut self,g:GetMode,gt:T){
        self.v.push((g,Box::new(gt)));
    }

    pub fn grab<'a>(&'a self)->Grabber<'a>{ 
        Grabber{
            v:Vec::new(),
            h:&self,
        }
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
}

pub struct Grabber<'a>{ //get builder
    v:Vec<(GetMode,&'a str)>,
    h:&'a GetHolder,
}

impl<'a> Grabber<'a>{
    pub fn env(self,s:&'a str)->Self{
        self.gm(GetMode::Env,s)
    }
    pub fn lz(self,s:&'a str)->Self{
        self.gm(GetMode::Lz,s)
    }
    pub fn fg(self,s:&'a str)->Self{
        self.gm(GetMode::Flag,s)
    }
    fn gm(mut self,g:GetMode,s:&'a str)->Self{
        self.v.push((g,s));
        self
    }

    pub fn s(self)->Option<String>{
        for (m,st) in self.v{
            if let Some(v)=  self.h.get(m,st){
                return Some(v);
            }
        }
        None
    }

    pub fn t<T:FromStr>(self)->Result<T,LzErr>{
        let s = self.s().ok_or(LzErr::NotFound)?;
        s.parse().map_err(|_|LzErr::ParseErr)

    }
}

#[derive(PartialEq,Clone,Copy)]
pub enum GetMode{
    Flag,
    Env,
    Lz,
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
        g.add(GetMode::Lz,GetTester(t1));
        g.add(GetMode::Flag,GetTester(t2));

        assert_eq!(&g.grab().lz("a").fg("a").s().unwrap(),"a_t1_res");
        assert_eq!(&g.grab().fg("a").lz("a").s().unwrap(),"a_t2_res");
        assert_eq!(g.grab().lz("num").fg("num").t::<i32>().unwrap(),4);
        assert!(g.grab().s().is_none());
        assert!(g.grab().fg("b").lz("b").s().is_none());
    }
}
