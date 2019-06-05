use failure_derive::*;

#[derive(Debug, PartialEq, Fail)]
pub enum LzErr {
    #[fail(display = "Parse Error")]
    ParseErr,
    #[fail(display = "Parse Error at {}", 0)]
    ParseErrAt(usize),
    #[fail(display = "Load Error")]
    LoadErr,
    #[fail(display = "Not Found Error")]
    NotFound,
    #[fail(display = "Env Var Error")]
    EnvVarErr,
}

use self::LzErr::*;

impl From<std::io::Error> for LzErr {
    fn from(_: std::io::Error) -> Self {
        LoadErr
    }
}

impl From<std::env::VarError> for LzErr {
    fn from(_: std::env::VarError) -> Self {
        EnvVarErr
    }
}
