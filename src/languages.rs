pub struct LanguageSettings<'a> {
    pub watch_path: &'a str
}

pub fn settings_for_language(language: &str) -> Option<LanguageSettings> {
    match language {
        "rust" => Some(LanguageSettings{ watch_path: "src" }),
        _ => None
    }
}
