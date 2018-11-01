mod lz_err;
mod lz_list;
mod get;
mod env;
mod flag;

pub use get::{Getable,GetHolder,GetMode};
use lz_err::LzErr;
use flag::FlagGetter;
pub use lz_list::{LzList,Lz};
use env::EnvGetter;

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

pub fn config<I,S>(c_loc_flag:&str,c_locs:I)->GetHolder
    where I:IntoIterator<Item=S>,
          S:AsRef<Path>,
{
    let mut res = GetHolder::new();
    let fg = FlagGetter();

    if let Some(s) = fg.get(c_loc_flag){
        if let Ok(l) = LzList::load(s){
            res.add(GetMode::Lz,l);
        }
    }

    //after load, as consumed but before rest of things as flags often come first
    res.add(GetMode::Flag,fg);
    res.add(GetMode::Env,EnvGetter());

    for fname in c_locs{
        if let Ok(l) = LzList::load(fname){
            res.add(GetMode::Lz,l);
        }
    }
    res
}


