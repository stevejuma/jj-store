use serde::{Deserialize, Deserializer, Serializer, de::Error as _};

macro_rules! codec {
    ($name:ident, $size:literal) => {
        pub(super) mod $name {
            use super::*;
            pub fn serialize<S: Serializer>(bytes: &[u8; $size], s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&hex::encode(bytes))
            }
            pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; $size], D::Error> {
                let value = String::deserialize(d)?;
                if value.len() != $size * 2
                    || value
                        .bytes()
                        .any(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
                {
                    return Err(D::Error::custom("invalid private capture digest"));
                }
                hex::decode(&value)
                    .map_err(|_| D::Error::custom("invalid private capture digest"))?
                    .try_into()
                    .map_err(|_| D::Error::custom("invalid private capture digest"))
            }
        }
    };
}
codec!(hex16, 16);
codec!(hex32, 32);
