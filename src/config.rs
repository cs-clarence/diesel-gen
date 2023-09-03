#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
  #[serde(default)]
  pub print_schema: PrintSchema,

  #[serde(default)]
  pub generate: Generate,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrintSchema {
  #[serde(default)]
  pub file: Option<PathBuf>,
  // #[serde(default)]
  // pub with_docs: print_schema::DocConfig,
  // #[serde(default)]
  // pub filter: Filtering,
  // #[serde(default)]
  // pub column_sorting: ColumnSorting,
  #[serde(default)]
  pub schema: Option<String>,
  #[serde(default)]
  pub patch_file: Option<PathBuf>,
  #[serde(default)]
  pub import_types: Option<Vec<String>>,
  #[serde(default)]
  pub generate_missing_sql_type_definitions: Option<bool>,
  #[serde(default)]
  pub custom_type_derives: Option<Vec<String>>,
}

#[derive(Default, Deserialize)]
pub struct Generate {
  #[serde(default)]
  pub model: Option<Model>,
}

#[derive(Default, Deserialize)]
pub struct Model {
  #[serde(default)]
  imports: Option<Vec<String>>,
  #[serde(default)]
  type_mapping: Option<HashMap<String, String>>,
}
