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
  pub models: Option<ModelsConfig>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ColumnConfig {
  #[serde(default)]
  pub rename: Option<String>,
  #[serde(default)]
  pub attributes: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
  #[serde(default)]
  pub skip: Option<bool>,
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
  #[serde(default)]
  pub operations: Option<OperationsConfig>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ModelsConfig {
  #[serde(default)]
  pub backend: Option<SqlBackend>,
  #[serde(default)]
  pub mods: Option<Vec<String>>,
  #[serde(default)]
  pub pub_mods: Option<Vec<String>>,
  #[serde(default)]
  pub uses: Option<Vec<String>>,
  #[serde(default)]
  pub pub_uses: Option<Vec<String>>,
  #[serde(default)]
  pub tables: Option<HashMap<String, TableConfig>>,
  #[serde(default)]
  pub output: Option<String>,
  #[serde(default)]
  pub table_imports_root: Option<String>,
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

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SqlBackend {
  #[default]
  Postgres,
  MySql,
  Sqlite,
}

impl SqlBackend {
  pub fn path(&self) -> &'static str {
    match self {
      SqlBackend::Postgres => "diesel::pg::Pg",
      SqlBackend::MySql => "diesel::mysql::Mysql",
      SqlBackend::Sqlite => "diesel::sqlite::Sqlite",
    }
  }
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OperationsConfig {
  #[serde(rename = "async")]
  pub r#async: Option<bool>,
  #[serde(default)]
  pub enable: Option<bool>,
  #[serde(default)]
  pub delete: Option<DeleteOperationConfig>,
  #[serde(default)]
  pub insert: Option<InsertOperationConfig>,
  #[serde(default)]
  pub update: Option<UpdateOperationConfig>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeleteOperationConfig {
  pub enable: Option<bool>,
  pub hard_delete: Option<bool>,
  pub soft_delete: Option<bool>,
  pub soft_delete_column: Option<String>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct InsertOperationConfig {
  pub enable: Option<bool>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpdateOperationConfig {
  pub enable: Option<bool>,
  pub per_column: Option<bool>,
  pub whole_table: Option<bool>,
  pub update_timestamp_columns: Option<Vec<String>>,
}
