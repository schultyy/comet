use rustc_serialize::json;

#[derive(RustcDecodable)]
pub struct Config {
    language: String,
    script: Vec<String>
}

pub fn from_json(json: &str) -> Config {
    json::decode(json).unwrap()
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
        let config = from_json(json);
        assert_eq!(config.language, "rust");
        assert_eq!(config.script, vec!("cargo test"));
    }
}
