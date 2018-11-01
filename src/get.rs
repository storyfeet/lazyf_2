use lz_err::LzErr;
use std::str::FromStr;

pub trait Getable{
    fn get(&self,&str)->Option<&str>;
}

#[derive(Clone,Copy)]
pub enum Getter{
}
pub struct GetHolder {    
    v:Vec<(GetMode,Box<Getable>)>,
}

pub struct Grabber<'a>{ //get builder
    v:Vec<(GetMode,&'a str)>
}

pub enum GetMode{
    Flag,
    Lz,
}

