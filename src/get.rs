use lz_err::LzErr;
use std::str::FromStr;

pub trait Getable{
    fn get(&self,&str)->Option<&str>;
}

