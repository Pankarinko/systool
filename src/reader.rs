use core::fmt;

#[derive(Debug, Clone)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "I need a better message!")
    }
}
