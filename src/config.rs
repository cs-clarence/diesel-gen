#![allow(dead_code)]

use core::fmt::Debug;
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
  pub attributes: Option<ListConfig<String>>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
  pub skip: Option<bool>,
  pub attributes: Option<ListConfig<String>>,
  pub updater_attributes: Option<ListConfig<String>>,
  pub inserter_attributes: Option<ListConfig<String>>,
  pub derives: Option<ListConfig<String>>,
  pub updater_derives: Option<ListConfig<String>>,
  pub inserter_derives: Option<ListConfig<String>>,
  pub columns: Option<HashMap<String, ColumnConfig>>,
  pub model_struct_name_prefix: Option<String>,
  pub model_struct_name_suffix: Option<String>,
  pub inserter_struct: Option<bool>,
  pub inserter_struct_name_prefix: Option<String>,
  pub inserter_struct_name_suffix: Option<String>,
  pub updater_struct: Option<bool>,
  pub updater_structs_name_prefix: Option<String>,
  pub updater_structs_name_suffix: Option<String>,
  pub updater_fields_optional: Option<bool>,
  pub operations: Option<OperationsConfig>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ModelsConfig {
  pub backend: Option<SqlBackend>,
  pub mods: Option<ListConfig<String>>,
  pub pub_mods: Option<ListConfig<String>>,
  pub uses: Option<ListConfig<String>>,
  pub pub_uses: Option<ListConfig<String>>,
  pub tables: Option<HashMap<String, TableConfig>>,
  pub output: Option<String>,
  pub table_imports_root: Option<String>,
  pub type_overrides: Option<HashMap<String, String>>,
  pub ref_type_overrides: Option<HashMap<String, String>>,
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
  pub enable: Option<bool>,
  pub delete: Option<DeleteOperationConfig>,
  pub insert: Option<InsertOperationConfig>,
  pub update: Option<UpdateOperationConfig>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeleteOperationConfig {
  pub enable: Option<bool>,
  pub hard_delete: Option<bool>,
  pub hard_delete_returning: Option<bool>,
  pub soft_delete: Option<bool>,
  pub soft_delete_returning: Option<bool>,
  pub soft_delete_column: Option<String>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct InsertOperationConfig {
  pub enable: Option<bool>,
  pub omit_columns: Option<ListConfig<String>>,
  pub returning: Option<bool>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpdateOperationConfig {
  pub enable: Option<bool>,
  pub per_column: Option<bool>,
  pub per_column_returning: Option<bool>,
  pub omit_columns: Option<ListConfig<String>>,
  pub whole_table: Option<bool>,
  pub whole_table_returning: Option<bool>,
  pub update_timestamp_columns: Option<ListConfig<String>>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum ListConfig<T>
where
  T: Default + Clone + Debug,
{
  Value(Vec<T>),
  Replace { replace: Vec<T> },
  Merge { merge: Vec<T> },
}

impl<T> Default for ListConfig<T>
where
  T: Default + Clone + Debug,
{
  fn default() -> Self {
    ListConfig::Value(Vec::new())
  }
}

impl<T> ListConfig<T>
where
  T: Default + Clone + Debug,
{
  pub fn into_vec(self) -> Vec<T> {
    match self {
      ListConfig::Value(v) => v,
      ListConfig::Replace { replace } => replace,
      ListConfig::Merge { merge } => merge,
    }
  }

  pub fn to_vec(&self) -> Vec<T> {
    match self {
      ListConfig::Value(v) => v.clone(),
      ListConfig::Replace { replace } => replace.clone(),
      ListConfig::Merge { merge } => merge.clone(),
    }
  }

  pub fn vec(&self) -> &Vec<T> {
    match self {
      ListConfig::Value(v) => v,
      ListConfig::Replace { replace } => replace,
      ListConfig::Merge { merge } => merge,
    }
  }

  pub fn vec_mut(&mut self) -> &mut Vec<T> {
    match self {
      ListConfig::Value(v) => v,
      ListConfig::Replace { replace } => replace,
      ListConfig::Merge { merge } => merge,
    }
  }

  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.vec().iter()
  }

  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    self.vec_mut().iter_mut()
  }

  pub fn is_empty(&self) -> bool {
    self.vec().is_empty()
  }

  pub fn combine(&self, other: &ListConfig<T>) -> ListConfig<T> {
    ListConfig::Value(match other {
      ListConfig::Value(v) => {
        self.iter().chain(v.iter()).cloned().collect::<Vec<T>>()
      }

      ListConfig::Replace { replace } => replace.clone(),
      ListConfig::Merge { merge } => {
        self.iter().chain(merge.iter()).cloned().collect::<Vec<T>>()
      }
    })
  }
}

impl<T> ListConfig<T>
where
  T: Default + Clone + Debug + PartialEq,
{
  pub fn dedup(&mut self) {
    let vec = self.vec_mut();
    vec.dedup();
  }
}

impl<T> IntoIterator for ListConfig<T>
where
  T: Default + Clone + Debug,
{
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.into_vec().into_iter()
  }
}

impl<'a, T> IntoIterator for &'a ListConfig<T>
where
  T: Default + Clone + Debug,
{
  type Item = &'a T;

  type IntoIter = std::slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.vec().iter()
  }
}

impl<'a, T> IntoIterator for &'a mut ListConfig<T>
where
  T: Default + Clone + Debug,
{
  type Item = &'a mut T;

  type IntoIter = std::slice::IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.vec_mut().iter_mut()
  }
}
