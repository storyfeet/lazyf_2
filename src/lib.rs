//! The Primary purpose of this module is to provide a super quick
//! way of accessing and prioritizing the three main configuration options:
//! * Arguments as Flags
//! * Environment Variables
//! * Configuration Files
//!
//! For configuration files I use the lazyf format, which is
//! straight forward to use:
//!
//! For example, the contents of "test_data/powers.lz":
//!
//! ```ignore
//! Superman:
//!     power:fly
//!     age:30
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
//! ## Intended Use
//! 
//! The intended use of the program is as follows Assuming the previous
//! file is in one of the supplied locations.
//!
//! ```rust
//!
//! use lazy_conf::config;
//!
//! // note: this has to be "mut" so help messages can be added
//! // note: this will load the "test_data/powers" file described above
//! let mut cfg =
//! config("-c",&["test_data/powers.lz","{HOME}/.config/myprogram.lz"])
//!                 .unwrap();
//!                 //only fails if -c was provided, but badly formed
//!
//!
//! // Use grab builder to get the info out.
//! // note: The help method makes a uses all of the previous cf,fg and env 
//! //  calls to build a useful message to the user.
//! let spower = cfg.grab()
//!                 .cf("Superman.power") 
//!                 .fg("-sppower")
//!                 .env("SUPERMAN_POWER")
//!                 .help("What power") 
//!                 .s();
//!
//! assert_eq!(spower,Some("fly".to_string()));
//!
//!
//! // You can search multiple elements of the same kind.
//! // If the first is not available the second will be chosen:
//! let home = cfg.grab()
//!                 .cf("Superman.home")
//!                 .cf("Batman.home").s();
//!
//! assert_eq!(home,Some("Gotham".to_string()));
//!
//! // s() closes returning an Option<String>, 
//! // t() closes returning a Result<T> where T:FromStr
//!
//! let age = cfg.grab().cf("Superman.age").t::<i32>();
//!
//! assert_eq!(age,Ok(30));
//!
//! // The "help" method, checks for the presence of a "--help" in the
//! // arguments, if found: it prints the compiled help info.
//!
//! if cfg.help("A Program to collect Superpowers") {
//!     // if flag --help existed, cfg prints collated help messages,
//!     // and returns true
//!     return;
//! }
//!
//! // to see the message we can call 
//! let hs = cfg.help_string("My Program");
//! assert_eq!(&hs,"\
//! My Program
//! Config file location flag: \"-c\"
//! default locations : [\"test_data/powers.lz\", \"{HOME}/.config/myprogram.lz\"]
//! What power:
//! \tConf:Superman.power,\tFlag:-sppower,\tEnv:SUPERMAN_POWER,\n\n"); 
//! ```
//! ## Loading Files
//!
//! ```rust,ignore
//! let cfg = config("-c",[loc1,loc2,loc3,etc]);
//! ```
//! The given flag ("-c") allows the user to supply a different path for the main config file.
//!
//! ```ignore
//! myprogram -c localconf.lz
//! ```
//!
//! Whether or not this flag is supplied, the program will attempt to
//! load all files.
//!
//! It is not considered an error even if none are found.
//!
//! When an lz request is made, it will search them
//! all (preloaded) in order "-c", "loc1" ,"loc2", ...
//!

pub mod get;
pub mod env;
mod lz_err;
mod lz_list;
mod flag;
mod loader;

pub use crate::get::{Getable,GetHolder,GetMode};
pub use crate::lz_err::LzErr;
pub use crate::flag::FlagGetter;
pub use crate::lz_list::{LzList,Lz};
pub use crate::env::EnvGetter;
pub use crate::loader::{load_local,Loader};

use std::fmt::Debug;


/// create a config loader object, so each individual config item can be built up and help items
/// can be added too.
/// fails if user provided flag for c_loc_flag, but it was badly formed.
pub fn config<I,S>(c_loc_flag:&str,c_locs:I)->Result<GetHolder,LzErr>
    where I:IntoIterator<Item=S>+Debug,
          S:AsRef<str>,
{
    let mut res = GetHolder::new();
    let fg = FlagGetter();

    res.add_help(&format!("Config file location flag: \"{}\"\
                       \ndefault locations : {:?}",c_loc_flag,c_locs));

    if let Some(s) = fg.get(c_loc_flag){
        let s = env::replace_env(&s)?;
        if let Ok(l) = load_local::<LzList,_>(s){
            res.add(GetMode::Conf,l);
        }
    }

    //after load, as consumed but before rest of things as flags often come first
    res.add(GetMode::Flag,fg);
    res.add(GetMode::Env,EnvGetter());


    for fname in c_locs{
        if let Ok(fname) = env::replace_env(fname.as_ref()){
            if let Ok(l) = load_local::<LzList,_>(fname){
                res.add(GetMode::Conf,l);
            }
        }
    }
    Ok(res)
}


