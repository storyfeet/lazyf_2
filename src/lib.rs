mod lz_err;
mod lz_list;
mod get;
use get::Getable;
mod env;
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
    let v = Vec::new();
    //TODO
    GetHolder{v}
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
