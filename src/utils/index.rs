pub fn create_meta_regexp(attr: &str) -> regex::Regex {
  regex::Regex::new(&format!("{}: (.*)", attr)).unwrap()
}

pub fn capture_val_by_regexp(regexp: &regex::Regex, data: &str) -> String {
  regexp
    .captures(&data)
    .unwrap()
    .get(1)
    .unwrap()
    .as_str()
    .to_string()
}
