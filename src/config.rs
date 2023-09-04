#![allow(dead_code)]

use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Default, Clone, Debug)]
pub struct DieselConfig {
  #[serde(default)]
  pub print_schema: Option<PrintSchema>,
}

#[derive(Default, Deserialize, Clone, Debug)]
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

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DieselGenConfig {
  #[serde(default)]
  pub model: Option<ModelConfig>,
}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct ColumnConfig {
  #[serde(default)]
  pub rename: Option<String>,
  #[serde(default)]
  pub attributes: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct TableConfig {
  #[serde(default)]
  pub attributes: Option<Vec<String>>,
  #[serde(default)]
  pub updater_attributes: Option<Vec<String>>,
  #[serde(default)]
  pub inserter_attributes: Option<Vec<String>>,
  #[serde(default)]
  pub derives: Option<Vec<String>>,
  #[serde(default)]
  pub updater_derives: Option<Vec<String>>,
  #[serde(default)]
  pub inserter_derives: Option<Vec<String>>,
  #[serde(default)]
  pub columns: Option<HashMap<String, ColumnConfig>>,
}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct ModelConfig {
  #[serde(default)]
  pub tables: Option<HashMap<String, TableConfig>>,
  #[serde(default)]
  pub output: Option<String>,
  #[serde(default)]
  pub imports: Option<Vec<String>>,
  #[serde(default)]
  pub table_import_root: Option<String>,
  #[serde(default)]
  pub type_overrides: Option<HashMap<String, String>>,
  #[serde(default)]
  pub ref_type_overrides: Option<HashMap<String, String>>,
  #[serde(default)]
  pub struct_names_prefix: Option<String>,
  #[serde(default)]
  pub struct_names_suffix: Option<String>,
  #[serde(default)]
  pub inserter_structs: Option<bool>,
  #[serde(default)]
  pub inserter_struct_names_prefix: Option<String>,
  #[serde(default)]
  pub inserter_struct_name_suffix: Option<String>,
  #[serde(default)]
  pub updater_structs: Option<bool>,
  #[serde(default)]
  pub updater_structs_name_prefix: Option<String>,
  #[serde(default)]
  pub updater_structs_name_suffix: Option<String>,
  #[serde(default)]
  pub updater_fields_optional: Option<bool>,
}
