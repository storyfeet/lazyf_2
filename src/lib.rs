mod lz_err;
mod lz_list;
mod get;
use get::Getable;
mod brace;
use lz_err::LzErr;
use std::io::Read;
use std::str::FromStr;
use std::fs::File;
use std::path::Path;

impl<T,E>Loader for T
    where T:FromStr<Err=E>,
        LzErr:From<E>, 
{
    fn read_into<R:Read>(mut r:R)->Result<Self,LzErr>{
        let mut b = String::new();
        r.read_to_string(&mut b)?;
        Ok(T::from_str(&b)?)
    }
}

pub trait Loader:Sized {
    fn read_into<R:Read>(r:R)->Result<Self,LzErr>;
    fn load<P:AsRef<Path>>(p:P)->Result<Self,LzErr>{
        let f = File::open(p)?;
        Self::read_into(f) 
    }
}

pub fn config(c_loc_flag:&str,)->GetHolder{
    GetHolder{v:Vec::new()}
}

pub enum GetMode{
    Flag,
    Lz,
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




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
