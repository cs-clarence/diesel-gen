#![allow(dead_code)]

use core::fmt::Debug;
use std::{collections::HashMap, fmt::Display, hash::Hash, path::PathBuf};

use merge::Merge;
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
  pub models: Option<ModelsConfig>,
  pub async_graphql: Option<AyncGraphqlConfig>,

  pub table_imports_root: Option<String>,
  pub type_overrides: Option<HashMap<String, String>>,
  pub ref_type_overrides: Option<HashMap<String, String>>,
  pub type_uses: Option<HashMap<String, String>>,
  pub tables: Option<HashMap<String, TableConfig>>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FieldConfig {
  pub rename: Option<String>,
  #[merge(strategy = merge_option)]
  pub attributes: Option<ListConfig<String>>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
  pub skip: Option<bool>,
  #[merge(strategy = merge_option)]
  pub attributes: Option<ListConfig<String>>,

  #[merge(strategy = merge_option)]
  pub updater_attributes: Option<ListConfig<String>>,

  #[merge(strategy = merge_option)]
  pub inserter_attributes: Option<ListConfig<String>>,

  #[merge(strategy = merge_option)]
  pub derives: Option<ListConfig<String>>,

  #[merge(strategy = merge_option)]
  pub updater_derives: Option<ListConfig<String>>,

  #[merge(strategy = merge_option)]
  pub inserter_derives: Option<ListConfig<String>>,

  #[merge(strategy = merge_option)]
  pub columns: Option<MapConfig<String, FieldConfig>>,

  pub model_struct_name_prefix: Option<String>,

  pub model_struct_name_suffix: Option<String>,

  pub inserter_struct: Option<bool>,

  #[merge(strategy = merge_option)]
  pub inserter_struct_omit_columns: Option<ListConfig<String>>,

  pub inserter_struct_name_prefix: Option<String>,

  pub inserter_struct_name_suffix: Option<String>,

  pub updater_struct: Option<bool>,

  #[merge(strategy = merge_option)]
  pub updater_struct_omit_columns: Option<ListConfig<String>>,

  pub updater_struct_name_prefix: Option<String>,

  pub updater_struct_name_suffix: Option<String>,

  pub updater_fields_optional: Option<bool>,

  #[merge(strategy = merge_option)]
  pub operations: Option<OperationsConfig>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct ModelsConfig {
  pub backend: SqlBackend,
  pub mods: Option<Vec<String>>,
  pub pub_mods: Option<Vec<String>>,
  pub uses: Option<Vec<String>>,
  pub pub_uses: Option<Vec<String>>,
  pub output: Option<String>,
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SqlBackend {
  #[default]
  Postgres,
  MySql,
  Sqlite,
}

impl Merge for SqlBackend {
  fn merge(&mut self, other: Self) {
    *self = other;
  }
}

impl Display for SqlBackend {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SqlBackend::Postgres => write!(f, "postgres"),
      SqlBackend::MySql => write!(f, "mysql"),
      SqlBackend::Sqlite => write!(f, "sqlite"),
    }
  }
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

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct OperationsConfig {
  #[serde(rename = "async")]
  pub r#async: Option<bool>,
  pub enable: Option<bool>,
  #[merge(strategy = merge_option)]
  pub delete: Option<DeleteOperationConfig>,
  #[merge(strategy = merge_option)]
  pub soft_delete: Option<SoftDeleteOperationConfig>,
  #[merge(strategy = merge_option)]
  pub insert: Option<InsertOperationConfig>,
  #[merge(strategy = merge_option)]
  pub update: Option<UpdateOperationConfig>,
  #[merge(strategy = merge_option)]
  pub simple_paginate: Option<SimplePaginateOperationConfig>,
  #[merge(strategy = merge_option)]
  pub cursor_paginate: Option<CursorPaginateOperationConfig>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct SoftDeleteOperationConfig {
  pub enable: Option<bool>,
  pub soft_delete_column: Option<String>,
  pub returning: Option<bool>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct DeleteOperationConfig {
  pub enable: Option<bool>,
  pub returning: Option<bool>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct InsertOperationConfig {
  pub enable: Option<bool>,
  #[merge(strategy = merge_option)]
  pub omit_columns: Option<ListConfig<String>>,
  pub returning: Option<bool>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct UpdateOperationConfig {
  pub enable: Option<bool>,
  pub returning: Option<bool>,
  pub column_wise_update: Option<bool>,
  pub column_wise_update_returning: Option<bool>,
  #[merge(strategy = merge_option)]
  pub omit_column_wise_update: Option<ListConfig<String>>,
  #[merge(strategy = merge_option)]
  pub update_timestamp_columns: Option<ListConfig<String>>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct SimplePaginateOperationConfig {
  pub enable: Option<bool>,
  pub include_soft_deleted: Option<bool>,
}

#[derive(Default, Deserialize, Clone, Debug, Merge)]
#[serde(deny_unknown_fields)]
pub struct CursorPaginateOperationConfig {
  pub enable: Option<bool>,
  pub include_soft_deleted: Option<bool>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum ListConfig<T> {
  Value(Vec<T>),
  Replace { replace: Vec<T> },
  Merge { merge: Vec<T> },
}

impl<T> PartialEq for ListConfig<T>
where
  T: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.vec() == other.vec()
  }
}

impl<T> Default for ListConfig<T> {
  fn default() -> Self {
    ListConfig::Value(Vec::new())
  }
}

impl<T> Merge for ListConfig<T> {
  fn merge(&mut self, other: Self) {
    match other {
      ListConfig::Value(value) => {
        self.vec_mut().extend(value);
      }
      ListConfig::Replace { replace } => {
        *self = ListConfig::Replace { replace };
      }
      ListConfig::Merge { merge } => {
        self.vec_mut().extend(merge);
      }
    }
  }
}

fn merge_option<T>(lhs: &mut Option<T>, rhs: Option<T>)
where
  T: Merge,
{
  if let Some(rhs) = rhs {
    match lhs {
      Some(lhs) => lhs.merge(rhs),
      None => *lhs = Some(rhs),
    }
  }
}

impl<T> ListConfig<T> {
  pub fn into_vec(self) -> Vec<T> {
    match self {
      ListConfig::Value(v) => v,
      ListConfig::Replace { replace } => replace,
      ListConfig::Merge { merge } => merge,
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

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MapConfig<K, V>
where
  K: Eq + Hash,
{
  Value(HashMap<K, V>),
  Replace { replace: HashMap<K, V> },
  Merge { merge: HashMap<K, V> },
}

impl<K, V> Default for MapConfig<K, V>
where
  K: Eq + Hash,
{
  fn default() -> Self {
    MapConfig::Value(HashMap::new())
  }
}

impl<K, V> MapConfig<K, V>
where
  K: Eq + Hash,
{
  pub fn map_mut(&mut self) -> &mut HashMap<K, V> {
    match self {
      MapConfig::Value(v) => v,
      MapConfig::Replace { replace } => replace,
      MapConfig::Merge { merge } => merge,
    }
  }

  pub fn map(&self) -> &HashMap<K, V> {
    match self {
      MapConfig::Value(v) => v,
      MapConfig::Replace { replace } => replace,
      MapConfig::Merge { merge } => merge,
    }
  }

  pub fn get<Q>(&self, key: &Q) -> Option<&V>
  where
    Q: ?Sized,
    K: std::borrow::Borrow<Q>,
    Q: Eq + Hash,
  {
    self.map().get(key)
  }
}

impl<K, V> Merge for MapConfig<K, V>
where
  K: Eq + Hash,
{
  fn merge(&mut self, other: Self) {
    match other {
      MapConfig::Value(value) => {
        self.map_mut().extend(value);
      }
      MapConfig::Replace { replace } => {
        *self = MapConfig::Replace { replace };
      }
      MapConfig::Merge { merge } => {
        self.map_mut().extend(merge);
      }
    }
  }
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct AyncGraphqlConfig {
  pub output: Option<String>,
  pub model_imports_root: Option<String>,

  #[serde(default)]
  pub output_types: HashMap<String, OutputTypeConfig>,

  pub mods: Option<Vec<String>>,
  pub pub_mods: Option<Vec<String>>,
  pub uses: Option<Vec<String>>,
  pub pub_uses: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug, Default, Merge)]
#[serde(deny_unknown_fields)]
pub struct OutputTypeConfig {
  #[merge(skip)]
  pub table: String,
  pub impl_from: Option<bool>,
  pub complex_object: Option<bool>,
  pub derives: Option<ListConfig<String>>,
  pub attributes: Option<ListConfig<String>>,
  pub fields: Option<MapConfig<String, FieldConfig>>,

  #[serde(default)]
  #[merge(skip)]
  pub inherits: Vec<String>,
}
