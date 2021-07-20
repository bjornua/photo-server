use rand::thread_rng;
use rand::Rng;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum IdDecodeError {
    Bs58DecodeError(bs58::decode::Error),
    WrongLength,
}

impl fmt::Display for IdDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IdDecodeError::Bs58DecodeError(_) => {
                write!(f, "Could not decode base 58 decode string")
            }

            IdDecodeError::WrongLength => {
                write!(f, "Decoded data doesn\'t have length 20")
            }
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Id([u8; 20]);

impl Id {
    pub fn new() -> Id {
        Id(thread_rng().gen())
    }
}

impl FromStr for Id {
    type Err = IdDecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buffer: [u8; 20] = [0; 20];
        let size = bs58::decode(s)
            .into(&mut buffer)
            .map_err(IdDecodeError::Bs58DecodeError)?;

        if size != 20 {
            return Err(IdDecodeError::WrongLength);
        }

        Ok(Id(buffer))
    }
}

impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.to_string(), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = String::deserialize(deserializer)?;

        match s.parse::<Id>() {
            Ok(id) => Ok(id),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", bs58::encode(&self.0).into_string())
    }
}
