use std::collections::HashMap;

use inflector::Inflector;

use crate::parse::File;

pub fn remove_spaces_from_keys(
  map: &HashMap<String, String>,
) -> HashMap<String, String> {
  map
    .iter()
    .map(|(k, v)| (k.replace(' ', ""), v.clone()))
    .collect::<HashMap<String, String>>()
}

pub fn import_root_from_path(file: &File, path: &str) -> String {
  let root = path
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
    .join("::");

  if let Some(module) = file.module.as_ref() {
    format!("{}::{}", root, module)
  } else {
    root
  }
}

pub fn final_name(
  prefix: Option<&str>,
  model_name: &str,
  suffix: Option<&str>,
) -> String {
  format!(
    "{}{}{}",
    prefix.unwrap_or(""),
    model_name,
    suffix.unwrap_or("")
  )
}

pub fn model_name(text: &str) -> String {
  text.to_pascal_case().to_singular()
}
