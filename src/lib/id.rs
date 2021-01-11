use bs58;
use rand::{thread_rng, Rng};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum IDDecodeError {
    BS58DecodeError(bs58::decode::Error),
    WrongLength,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ID([u8; 20]);

impl ID {
    pub fn new() -> ID {
        ID(thread_rng().gen())
    }
}

impl FromStr for ID {
    type Err = IDDecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buffer: [u8; 20] = [0; 20];
        let size = bs58::decode(s)
            .into(&mut buffer)
            .map_err(IDDecodeError::BS58DecodeError)?;

        if size != 20 {
            return Err(IDDecodeError::WrongLength);
        }

        return Ok(ID(buffer));
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}

impl fmt::Debug for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", bs58::encode(&self.0).into_string())
    }
}
