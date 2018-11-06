use lz_err::LzErr;

use std::fs::File;
use std::str::FromStr;
use std::io::Read;
use std::path::{Path,PathBuf};
use get::Getable;

pub trait Loader:Sized {
    fn read_into<R:Read>(r:R)->Result<Self,LzErr>;
    fn load<P:AsRef<Path>>(p:P)->Result<Self,LzErr>{
        let f = File::open(p)?;
        Self::read_into(f) 
    }
}

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

pub struct Local<T>{
    root:PathBuf,
    item:T,
}


pub fn load_local<T:Loader,P:AsRef<Path>>(p:P)->Result<Local<T>,LzErr>{
    Ok(Local{
        root:PathBuf::from(p.as_ref().parent().unwrap_or(&PathBuf::from("./"))),
        item:T::load(p)?,
    })
}

impl<T:Getable> Getable for Local<T>{
    fn get(&self,s:&str)->Option<String>{
        self.item.get(s)
    }
    fn localize(&self,p:&Path)->PathBuf{
        if p.is_absolute() {
            return PathBuf::from(p);
        }
        let mut res = self.root.clone();
        res.push(p);
        res
    }
}

#[cfg(test)]
mod test{
    use super::*;
    struct GTest {
    }
    impl Getable for GTest{
        fn get(&self,s:&str)->Option<String>{
            let mut res =  String::from(s);
            res.push_str("bye");
            Some(res)
        }
    }

    #[test]
    fn test_pathness(){
        let lc = Local{
            root:PathBuf::from("hello"),
            item:GTest{},
        };

        let np = lc.localize(&PathBuf::from("goodbye"));
        assert_eq!(np, PathBuf::from("hello/goodbye"));

        let np = lc.localize(&PathBuf::from("/goodbye"));
        assert_eq!(np, PathBuf::from("/goodbye"));
    }
}

