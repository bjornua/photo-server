use bs58;
use rand::{thread_rng, Rng};

pub enum TokenDecodeError {
    BS58DecodeError(bs58::decode::Error),
    WrongLength,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Token([u8; 32]);

impl Token {
    pub fn new() -> Token {
        Token(thread_rng().gen())
    }

    pub fn decode(s: String) -> Result<Token, TokenDecodeError> {
        let mut buffer: [u8; 32] = [0; 32];
        let size = bs58::decode(s)
            .into(&mut buffer)
            .map_err(TokenDecodeError::BS58DecodeError)?;

        if size != 32 {
            return Err(TokenDecodeError::WrongLength);
        }

        return Ok(Token(buffer));
    }

    pub fn encode(&self) -> String {
        bs58::encode(&self.0).into_string()
    }
}
