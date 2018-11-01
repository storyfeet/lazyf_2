
#[derive(Debug)]
pub enum LzErr{
    ParseErr(usize),
    LoadErr,
    NotFound,
    EnvVarErr,
}

use self::LzErr::*;

impl From<std::io::Error> for LzErr{
    fn from(_:std::io::Error)->Self{
        LoadErr
    }
}

impl From<std::env::VarError> for LzErr{
    fn from(_:std::env::VarError) ->Self{EnvVarErr}
}



