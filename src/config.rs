use rustc_serialize::{json};
use rustc_serialize::json::DecoderError;

#[derive(RustcDecodable)]
pub struct Config {
    pub script: Vec<String>,
    pub watch: String
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
              \"script\": [
                \"cargo test\"
              ],
              \"watch\": \"src/\"
            }";
        let config = from_json(json).unwrap();
        assert_eq!(config.script, vec!("cargo test"));
        assert_eq!(config.watch, "src/");
    }
}
