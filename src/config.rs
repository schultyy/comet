use rustc_serialize::{json};
use rustc_serialize::json::DecoderError;

#[derive(RustcDecodable)]
pub struct Config {
    pub language: String,
    pub script: Vec<String>
}

pub fn from_json(json: &str) -> Result<Config, DecoderError> {
    json::decode(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_json() {
        let json = "
            {
              \"language\": \"rust\",
              \"script\": [
                \"cargo test\"
              ]
            }";
        let config = from_json(json).unwrap();
        assert_eq!(config.language, "rust");
        assert_eq!(config.script, vec!("cargo test"));
    }
}
