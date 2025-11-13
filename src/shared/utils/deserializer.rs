use serde::{Deserialize, Deserializer};

pub fn de_u64_str_ok<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumStr {
        N(u64),
        S(String),
    }

    match NumStr::deserialize(d)? {
        NumStr::N(n) => Ok(n),
        NumStr::S(s) => s.parse().map_err(serde::de::Error::custom),
    }
}
