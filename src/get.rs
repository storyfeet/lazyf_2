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

