pub fn create_meta_section_regexp() -> regex::Regex {
  regex::Regex::new("---([^.*]*)---").unwrap()
}

pub fn create_meta_regexp(attr: &str) -> regex::Regex {
  regex::Regex::new(&format!("{}: (.*)", attr)).unwrap()
}

pub fn capture_val_by_regexp(regexp: &regex::Regex, data: &str) -> String {
  regexp
    .captures(&data)
    .expect("meta section not found")
    .get(1)
    .expect("can't get meta section")
    .as_str()
    .to_string()
}
