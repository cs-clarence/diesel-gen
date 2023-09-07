use std::collections::HashMap;

pub fn remove_spaces_from_keys(
  map: &HashMap<String, String>,
) -> HashMap<String, String> {
  map
    .iter()
    .map(|(k, v)| (k.replace(' ', ""), v.clone()))
    .collect::<HashMap<String, String>>()
}

pub fn import_root_from_path(path: &str) -> String {
  path
    .trim_start_matches("./")
    .split('/')
    .filter_map(|mut e| {
      if e == "mod.rs" {
        return None;
      }

      if e == "src" {
        return Some("crate");
      }

      if e.ends_with(".rs") {
        e = e.trim_end_matches(".rs");

        return Some(e);
      }

      Some(e)
    })
    .collect::<Vec<&str>>()
    .join("::")
}
