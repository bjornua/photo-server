#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Type {
    Jpg,
    Png,
}

impl Type {
    fn from_mime_type(mimetype: &str) -> Option<Type> {
        match mimetype {
            "image/jpeg" => Some(Type::Jpg),
            "image/png" => Some(Type::Png),
            _ => None,
        }
    }
}

impl Type {
    fn to_mime_type(self) -> &'static str {
        match self {
            Type::Jpg => "image/jpeg",
            Type::Png => "image/png",
        }
    }
}
