
#[derive(Debug)]
pub enum LzErr{
    ParseErr(usize),
    LoadErr,
    NotFound,
}

use self::LzErr::*;

impl From<std::io::Error> for LzErr{
    fn from(_:std::io::Error)->Self{
        LoadErr
    }
}



