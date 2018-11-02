//! The Primary purpose of this module is to provide a super quick
//! way of accessing and prioritizing the three main configuration options:
//! * Arguments as Flags
//! * Environment Variables
//! * Configuration Files
//!
//! For configuration files I use the lazyf format, which is
//! straight forward to use:
//!
//! ```ignore
//! Superman:
//!     power:fly
//!     home:Kansas
//!
//! #comment
//!
//! Batman:
//!     power:money
//!     home:Gotham
//! ```
//!
//! The structure is only 2 layers deep, and uses any whitespace to
//! indicate that this row is a child or the previous untabbed row.
//! 
//! The intended use of the program is as follows Assuming the previous
//! file is in one of the supplied locations.
//!
//! ```rust,no_run
//!
//! use lazy_conf::config;
//!
//! // note mut so help messages can be added
//! let mut cfg = config("-c",&["conf.lz","{HOME}/.config/myprogram.lz"]);
//! let spower = cfg.grab()
//!                 .lz("Superman.power") 
//!                 .fg("-sppower")
//!                 .env("SUPERMAN_POWER").help("What power").s();
//!
//! assert_eq!(spower,Some("fly".to_string()));
//!
//! if cfg.help("A Program to collect Superpowers") {
//!     // if flag --help existed, cfg prints collated help messages,
//!     // and returns true
//!     return;
//! }
//!
//! ```
//!
//! Items are searched in the order you choose, so if you would
//! rather prioritize the flag result, put ```.fg(...)``` first. 
//! You can also supply two lz places to try.
//! ```rust,ignore
//! let spower = cfg.grab()
//!                 .lz("Superman.power") 
//!                 .lz("Batman.power").s()
//! ```
//!
//! will return Some("money") if supermans is not supplied.
//!
//! ```rust,ignore
//! let cfg = config("-c",[loc1,loc2,loc3,etc]);
//! ```
//! The given flag ("-c") should refer to a path
//!
//! ```ignore
//! myprogram -c localconf.lz
//! ```
//!
//! Whether or not this flag is supplied, the program will attempt to
//! load all files, and when an lz request is made, it will search them
//! all (preloaded) in order "-c", "loc1" ,"loc2", ...
//!
//!
//!
//!

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


