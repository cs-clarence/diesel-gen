#![allow(dead_code)]

use std::{
  collections::{HashMap, HashSet},
  io::Write,
  sync::OnceLock,
};

use inflector::Inflector;
use merge::Merge;

use crate::{
  config::{ListConfig, SqlBackend, TableConfig},
  parse::{self, type_name, Column, File, ParseContext, Table, Type, TypeName},
};

pub fn rust_file_headers<W: Write>(mut writer: W) -> std::io::Result<()> {
  writeln!(writer, "// @generated automatically by diesel-gen\n")?;

  writeln!(writer, "#![allow(unused)]")?;
  writeln!(writer, "#![allow(clippy::all)]\n")?;

  Ok(())
}

fn to_singular_pascal_case(text: &str) -> String {
  text.to_pascal_case().to_singular()
}

pub fn table_uses<W: Write>(
  table_imports_root: &str,
  tables: &Vec<Table>,
  table_configs: &HashMap<String, TableConfig>,
  mut w: W,
) -> anyhow::Result<()> {
  let wildcard_table_config = table_configs.get("*");

  if !tables.is_empty() {
    writeln!(w, "use {}::{{", table_imports_root)?;
    for table in tables {
      let table_config = table_configs.get(&table.name);

      if let Some(conf) = table_config {
        if conf.skip.unwrap_or(false) {
          continue;
        }
      }

      if let Some(conf) = wildcard_table_config {
        if conf.skip.unwrap_or(false) {
          continue;
        }
      }

      writeln!(w, "{},", table.name)?;
    }
    writeln!(w, "}};")?;
  }

  Ok(())
}

pub fn table_type_uses<W: Write>(
  table: &Table,
  types_uses: &HashMap<String, String>,
  type_overrides: &HashMap<String, String>,
  mut w: W,
) -> anyhow::Result<()> {
  let mut uses_set = HashSet::new();

  for c in &table.columns {
    if let Some(str) = get_type(type_overrides, &c.r#type) {
      let tp = parse::r#type(&mut ParseContext::new(&str))?;

      for i in tp.type_names() {
        if let Some(ty) = types_uses.get(&i.to_string().replace(' ', "")) {
          uses_set.insert(ty.clone());
        }
      }
    }
  }

  for u in uses_set {
    writeln!(w, "use {};", u)?;
  }

  Ok(())
}

pub fn tables_type_uses<W: Write>(
  tables: &Vec<Table>,
  types_uses: &HashMap<String, String>,
  type_overrides: &HashMap<String, String>,
  mut w: W,
) -> anyhow::Result<()> {
  let mut uses_set = HashSet::new();

  for table in tables {
    for c in &table.columns {
      if let Some(str) = get_type(type_overrides, &c.r#type) {
        let tp = parse::r#type(&mut ParseContext::new(&str))?;

        for i in tp.type_names() {
          if let Some(ty) = types_uses.get(&i.to_string().replace(' ', "")) {
            uses_set.insert(ty.clone());
          }
        }
      }
    }
  }

  for u in uses_set {
    writeln!(w, "use {};", u)?;
  }

  Ok(())
}

fn is_rust_keyword(str: &str) -> bool {
  matches!(
    str,
    "abstract"
      | "alignof"
      | "as"
      | "become"
      | "box"
      | "break"
      | "const"
      | "continue"
      | "crate"
      | "do"
      | "else"
      | "enum"
      | "extern"
      | "false"
      | "final"
      | "fn"
      | "for"
      | "if"
      | "impl"
      | "in"
      | "let"
      | "loop"
      | "macro"
      | "match"
      | "mod"
      | "move"
      | "mut"
      | "offsetof"
      | "override"
      | "priv"
      | "proc"
      | "pub"
      | "pure"
      | "ref"
      | "return"
      | "Self"
      | "self"
      | "sizeof"
      | "static"
      | "struct"
      | "super"
      | "trait"
      | "true"
      | "type"
      | "typeof"
  )
}

fn get_field_name(table_config: &TableConfig, column_name: &str) -> String {
  fn imp(table: &TableConfig, column_name: &str) -> String {
    if let Some(columns) = table.columns.as_ref() {
      if let Some(column) = columns.get(column_name) {
        return column.rename.as_deref().unwrap_or(column_name).to_string();
      }
    }

    column_name.to_string()
  }

  let field_name = imp(table_config, column_name);

  if is_rust_keyword(&field_name) {
    format!("r#{}", field_name)
  } else {
    field_name
  }
}

macro_rules! hash_map {
  {$($key:expr => $value:expr),* $(,)?} => {{
    let mut map = HashMap::new();
    $(map.insert($key, $value);)*
    map
  }};
}

static RUST_TYPE_MAP: OnceLock<HashMap<&'static str, &'static str>> =
  OnceLock::new();

fn init_type_map() {
  use type_name::*;
  _ = RUST_TYPE_MAP.set(hash_map! {
   ARRAY => "Vec",
   INT2 => "i16",
   SMALLINT=> "i16",
   INT4 => "i32",
   INTEGER => "i32",
   UNSIGNED => "Unsigned",
   INT8 => "i64",
   BIGINT => "i64",
   NUMERIC => "bigdecimal::BigDecimal",
   DECIMAL => "bigdecimal::BigDecimal",
   TEXT => "String",
   DATE => "time::Date",
   TIME => "time::Time",
   DATETIME => "time::OffsetDateTime",
   TIMESTAMP => "time::OffsetDateTime",
   TIMESTAMPTZ => "time::OffsetDateTime",
   FLOAT4 => "f32",
   FLOAT8 => "f64",
   FLOAT => "f32",
   BOOL => "bool",
   JSON => "serde_json::Value",
   JSONB => "serde_json::Value",
   UUID => "uuid::Uuid",
   CHAR => "char",
   VARCHAR =>"String",
   DOUBLE => "f64",
   TINYINT => "i8",
   NULLABLE => "Option",

  // Unsupported types
   BYTEA => "Bytea",
   BINARY =>"Binary",
   VARBINARY => "Varbinary",
   BLOB => "Blob",
   TINYBLOB => "Tinyblob",
   MEDIUMBLOB => "Mediumblob",
   LONGBLOB => "Longblob",
   BIT => "Bit",
   INET => "Inet",
   TINYTEXT => "Tinytext",
   MEDIUMTEXT => "Mediumtext",
   LONGTEXT => "Longtext",
  });
}

fn get_type(
  type_overrides: &HashMap<String, String>,
  ty: &Type,
) -> Option<String> {
  init_type_map();
  let ts = ty.to_string().replace(' ', "");

  if let Some(ty) = type_overrides.get(&ts) {
    return Some(ty.clone());
  }

  let tp_map = RUST_TYPE_MAP.get().expect("RUST_TYPE_MAP not initialized");

  if let Some(ty) = tp_map.get(ts.as_str()) {
    return Some(ty.to_string());
  }

  if let Some(t) = tp_map.get(&ty.name().to_string().as_str()) {
    let params = ty
      .params()
      .iter()
      .map(|i| get_type(type_overrides, i))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("{}<{}>", t, params.join(", ")));
  }

  None
}

fn get_ref_type(
  type_overrides: &HashMap<String, String>,
  ty: &Type,
  lifetime: Option<&str>,
) -> Option<String> {
  init_type_map();
  let ts = ty.to_string().replace(' ', "");

  let lifetime = if let Some(lifetime) = lifetime {
    if lifetime.is_empty() {
      "".to_string()
    } else {
      format!("{} ", lifetime)
    }
  } else {
    "".to_string()
  };

  if let Some(ty) = type_overrides.get(&ts) {
    return Some(ty.clone());
  }

  if ty.name().is_string_type() {
    return Some(format!("&{}str", lifetime));
  }

  if *ty.name() == TypeName::Nullable {
    let params = ty
      .params()
      .iter()
      .map(|i| get_ref_type(type_overrides, i, Some(&lifetime)))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("Option<{}>", params.join(", ")));
  }

  let tp_map = RUST_TYPE_MAP.get().expect("RUST_TYPE_MAP not initialized");

  if let Some(ty) = tp_map.get(ts.as_str()) {
    return Some(format!("&{}{}", lifetime, ty));
  }

  if let Some(t) = tp_map.get(&ty.name().to_string().as_str()) {
    let params = ty
      .params()
      .iter()
      .map(|i| get_type(type_overrides, i))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("&{}{}<{}>", lifetime, t, params.join(", ")));
  }

  None
}

const DIESEL_DEFAULT_DERIVE: &str =
    "#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable)]";

const DIESEL_DEFAULT_WITH_CHANGESET_DERIVE: &str =
    "#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]";

const DIESEL_INSERTER_DERIVE: &str = "#[derive(diesel::Insertable)]";

const DIESEL_UPDATER_DERIVE: &str = "#[derive(diesel::AsChangeset)]";

pub struct ModelsArgs<'a> {
  pub file: &'a File,
  pub backend: &'a SqlBackend,
  pub table_configs: &'a HashMap<String, TableConfig>,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
  pub pub_uses: &'a Vec<&'a str>,
  pub pub_mods: &'a Vec<&'a str>,
  pub uses: &'a Vec<&'a str>,
  pub mods: &'a Vec<&'a str>,
}

pub fn models<W: Write>(
  &ModelsArgs {
    file,
    backend,
    ref_type_overrides,
    type_overrides,
    table_configs,
    mods,
    pub_mods,
    pub_uses,
    uses,
  }: &ModelsArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let wildcard_table_config =
    table_configs.get("*").cloned().unwrap_or_default();

  for i in uses {
    writeln!(w, "use {};", i)?;
  }

  for i in pub_uses {
    writeln!(w, "pub use {};", i)?;
  }

  for m in mods {
    writeln!(w, "mod {};", m)?;
  }

  for m in pub_mods {
    writeln!(w, "pub mod {};", m)?;
  }

  for table in &file.tables {
    let mut table_configs =
      table_configs.get(&table.name).cloned().unwrap_or_default();

    table_configs.merge(wildcard_table_config.clone());

    model(
      &ModelArgs {
        backend,
        ref_type_overrides,
        type_overrides,
        table,
        table_config: &table_configs,
        mods: &vec![],
        pub_mods: &vec![],
        pub_uses: &vec![],
        uses: &vec![],
      },
      &mut w,
    )?;
  }

  Ok(())
}

fn get_soft_delete_column<'a>(
  t: &'a Table,
  col: Option<&str>,
) -> Option<&'a Column> {
  const DEF_COL_NAMES: [&str; 5] = [
    "deleted_at",
    "deleted",
    "is_deleted",
    "datetime_deleted",
    "date_deleted",
  ];

  if let Some(col) = col {
    t.columns.iter().find(|c| c.name == *col)
  } else {
    for col in DEF_COL_NAMES.iter() {
      if let Some(col) = t.columns.iter().find(|c| c.name == *col) {
        return Some(col);
      }
    }

    None
  }
}

fn get_update_timestamp_columns<'a>(
  t: &'a Table,
  cols: Option<&ListConfig<String>>,
) -> Vec<&'a Column> {
  const DEF_COL_NAMES: [&str; 4] = [
    "updated_at",
    "datetime_updated",
    "date_updated",
    "datetime_deleted",
  ];

  if let Some(cols) = cols {
    cols
      .vec()
      .iter()
      .filter_map(|str| t.get_column(str))
      .collect::<Vec<_>>()
  } else {
    DEF_COL_NAMES
      .iter()
      .filter_map(|str| t.get_column(str))
      .collect::<Vec<_>>()
  }
}

fn write_ref_fn_params<W: Write>(
  type_overrides: &HashMap<String, String>,
  cols: &Vec<&Column>,
  mut w: W,
) -> std::io::Result<()> {
  for c in cols {
    write!(
      w,
      "{}: {}, ",
      c.name,
      get_ref_type(type_overrides, &c.r#type, None)
        .expect("Unknown type encountered")
    )?;
  }

  Ok(())
}

fn operation_sig<W: Write>(
  use_async: bool,
  name: &str,
  backend: &SqlBackend,
  lifetimes: Option<Vec<&str>>,
  mut w: W,
) -> std::io::Result<()> {
  if use_async {
    writeln!(w, "  pub async fn {}<", name)?;
  } else {
    writeln!(w, "  pub fn {}<", name)?;
  }

  if let Some(lifetimes) = lifetimes {
    for lf in lifetimes {
      writeln!(w, "{}, ", lf)?;
    }
  }

  if use_async {
    writeln!(
      w,
      "Conn: diesel_async::AsyncConnection<Backend = {}>>(",
      backend
    )?;
  } else {
    writeln!(w, "Conn: diesel::Connection<Backend = {}>>(", backend)?;
  }

  Ok(())
}

fn default_uses<W: Write>(
  use_async: bool,
  use_selectable_helper: bool,
  use_expression_methods: bool,
  mut w: W,
) -> std::io::Result<()> {
  if use_selectable_helper {
    writeln!(w, "use diesel::SelectableHelper;")?;
  }

  if use_expression_methods {
    writeln!(w, "use diesel::ExpressionMethods;")?;
  }

  if use_async {
    writeln!(w, "use diesel_async::RunQueryDsl;")?;
  } else {
    writeln!(w, "use diesel::RunQueryDsl;")?;
  }

  Ok(())
}

pub struct ModelArgs<'a> {
  pub table: &'a Table,
  pub table_config: &'a TableConfig,
  pub backend: &'a SqlBackend,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
  pub pub_uses: &'a Vec<&'a str>,
  pub pub_mods: &'a Vec<&'a str>,
  pub uses: &'a Vec<&'a str>,
  pub mods: &'a Vec<&'a str>,
}

pub fn model<W: Write>(
  &ModelArgs {
    backend,
    ref_type_overrides,
    table,
    table_config,
    type_overrides,
    pub_mods,
    pub_uses,
    mods,
    uses,
  }: &ModelArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let optional_updater_fields =
    table_config.updater_fields_optional.unwrap_or(true);

  for i in uses {
    writeln!(w, "use {};", i)?;
  }

  for i in pub_uses {
    writeln!(w, "pub use {};", i)?;
  }

  for m in mods {
    writeln!(w, "mod {};", m)?;
  }

  for m in pub_mods {
    writeln!(w, "pub mod {};", m)?;
  }

  let mut d = table_config.derives.clone().unwrap_or_default();

  d.dedup();
  if !d.is_empty() {
    writeln!(w, "#[derive({})]", d.vec().join(", "))?;
  }

  if table.only_primary_key_columns() {
    writeln!(w, "{}", DIESEL_DEFAULT_DERIVE)?;
  } else {
    writeln!(w, "{}", DIESEL_DEFAULT_WITH_CHANGESET_DERIVE)?;
  }

  writeln!(w, "#[diesel(table_name = {})]", table.name)?;
  writeln!(
    w,
    "#[diesel(primary_key({}))]",
    table.primary_key.join(", ")
  )?;

  writeln!(w, "#[diesel(check_for_backend({}))]", backend)?;

  let inserter_prefix = table_config
    .inserter_struct_name_prefix
    .clone()
    .unwrap_or_default();

  let inserter_suffix = table_config
    .inserter_struct_name_suffix
    .clone()
    .unwrap_or_else(|| "New".to_string());
  let updater_prefix = table_config
    .updater_struct_name_prefix
    .clone()
    .unwrap_or_default();
  let updater_suffix = table_config
    .updater_struct_name_suffix
    .clone()
    .unwrap_or_else(|| "Update".to_string());
  let model_prefix = table_config
    .model_struct_name_prefix
    .clone()
    .unwrap_or_default();
  let model_suffix = table_config
    .model_struct_name_suffix
    .clone()
    .unwrap_or_default();

  let struct_name = to_singular_pascal_case(&table.name);

  let final_model_name =
    format!("{}{}{}", model_prefix, struct_name, model_suffix);
  let final_inserter_name =
    format!("{}{}{}", inserter_prefix, struct_name, inserter_suffix);
  let final_updater_name =
    format!("{}{}{}", updater_prefix, struct_name, updater_suffix);

  let mut a = table_config.attributes.clone().unwrap_or_default();

  a.dedup();

  if !a.is_empty() {
    for a in a {
      writeln!(w, "#[{}]", a)?;
    }
  }

  writeln!(w, "pub struct {} {{", final_model_name)?;

  for c in &table.columns {
    let field_name = get_field_name(table_config, &c.name);

    if field_name != c.name {
      writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
    }
    writeln!(
      w,
      "  pub {}: {},",
      field_name,
      get_type(type_overrides, &c.r#type).ok_or_else(|| {
        anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
      })?
    )?;
  }

  writeln!(w, "}}\n")?;

  let inserter_structs = table_config.inserter_struct.unwrap_or(true);

  if inserter_structs {
    let mut d = table_config.inserter_derives.clone().unwrap_or_default();

    d.dedup();
    if !d.is_empty() {
      writeln!(w, "#[derive({})]", d.vec().join(", "))?;
    }

    writeln!(w, "{}", DIESEL_INSERTER_DERIVE)?;
    writeln!(w, "#[diesel(table_name = {})]", table.name)?;
    writeln!(w, "#[diesel(check_for_backend({}))]", backend)?;

    let mut a = table_config.inserter_attributes.clone().unwrap_or_default();

    a.dedup();

    if !a.is_empty() {
      for a in a {
        writeln!(w, "#[{}]", a)?;
      }
    }

    let lifetime = "'a";
    writeln!(w, "pub struct {}<{}>{{", final_inserter_name, lifetime)?;
    for c in &table.columns {
      let field_name = get_field_name(table_config, &c.name);

      if field_name != c.name {
        writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
      }
      writeln!(
        w,
        "  pub {}: {},",
        field_name,
        get_ref_type(ref_type_overrides, &c.r#type, Some(lifetime))
          .ok_or_else(|| {
            anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
          },)?
      )?;
    }
    writeln!(w, "}}\n")?;
  }

  let non_primary_key_columns = table.non_primary_key_columns();

  let updater_structs = table_config.updater_struct.unwrap_or(true);

  if updater_structs && !table.only_primary_key_columns() {
    let mut d = table_config.updater_derives.clone().unwrap_or_default();

    d.dedup();
    if !d.is_empty() {
      writeln!(w, "#[derive({})]", d.vec().join(", "))?;
    }
    writeln!(w, "{}", DIESEL_UPDATER_DERIVE)?;
    writeln!(w, "#[diesel(table_name = {})]", table.name)?;
    writeln!(w, "#[diesel(check_for_backend({}))]", backend)?;

    let mut a = table_config.updater_attributes.clone().unwrap_or_default();

    a.dedup();

    if !a.is_empty() {
      for a in a {
        writeln!(w, "#[{}]", a)?;
      }
    }

    let lifetime = "'a";
    writeln!(w, "pub struct {}<{}>{{", final_updater_name, lifetime)?;
    for c in &non_primary_key_columns {
      let field_name = get_field_name(table_config, &c.name);

      if field_name != c.name {
        writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
      }
      let ty = get_ref_type(ref_type_overrides, &c.r#type, Some(lifetime))
        .ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?;
      writeln!(
        w,
        "  pub {}: {},",
        field_name,
        if optional_updater_fields {
          format!("Option<{}>", ty)
        } else {
          ty
        }
      )?;
    }
    writeln!(w, "}}\n")?;

    let operations = table_config
      .operations
      .as_ref()
      .cloned()
      .unwrap_or_default();

    let enable_operations = operations.enable.unwrap_or(false);

    if !enable_operations {
      return Ok(());
    }

    let delete_config = operations.delete.unwrap_or_default();
    let enable_delete = delete_config.enable.unwrap_or(true);
    let soft_delete = delete_config.soft_delete.unwrap_or(true);
    let hard_delete = delete_config.hard_delete.unwrap_or(true);
    let soft_delete_column = delete_config.soft_delete_column;

    let update_config = operations.update.unwrap_or_default();
    let enable_update = update_config.enable.unwrap_or(true);

    let per_column = update_config.per_column.unwrap_or(true);
    let whole_table = update_config.whole_table.unwrap_or(true);

    let ut = update_config.update_timestamp_columns;

    let timestamp_columns = get_update_timestamp_columns(table, ut.as_ref());

    if !timestamp_columns.iter().all(|i| {
      i.r#type.is_datetime_type()
        || i.r#type.is_nullable_type(|t| t.is_datetime_type())
    }) {
      return Err(anyhow::anyhow!(
        "Only datetime columns are supported for update_timestamp_columns"
      ));
    }

    let insert_config = operations.insert.unwrap_or_default();
    let enable_insert = insert_config.enable.unwrap_or(true);

    let use_async = operations.r#async.unwrap_or(true);

    let primary_keys = table.primary_key_columns();

    if enable_delete {
      writeln!(w, "impl {} {{", final_model_name)?;
      if hard_delete {
        operation_sig(use_async, "delete", backend, None, &mut w)?;
        write_ref_fn_params(ref_type_overrides, &primary_keys, &mut w)?;
        write!(w, "mut conn: Conn")?;
        write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;

        default_uses(use_async, true, true, &mut w)?;
        writeln!(
          w,
          r#"
            diesel::delete({table}::table)
            "#,
          table = table.name
        )?;

        for i in &primary_keys {
          writeln!(
            w,
            r#"
                .filter({table}::{column}.eq({column}))
              "#,
            table = table.name,
            column = i.name,
          )?;
        }

        writeln!(
          w,
          r#"
              .returning({model}::as_returning())
              .get_result::<{model}>(&mut conn)
              .await
            "#,
          model = final_model_name
        )?;
        writeln!(w, "}}")?;
      }

      if let Some(c) =
        get_soft_delete_column(table, soft_delete_column.as_deref())
      {
        if !(c.r#type.is_boolean_type()
          || c.r#type.is_nullable_type(|t| t.is_datetime_type())
          || c.r#type.is_integer_type())
        {
          return Err(anyhow::anyhow!(
              "Unsupported soft delete column type '{}' of column '{}' in table '{}'. Supported class of types are boolean, datetime, integer",
              c.r#type,
              c.name,
              table.name
            ));
        }

        if soft_delete {
          operation_sig(use_async, "soft_delete", backend, None, &mut w)?;
          write_ref_fn_params(ref_type_overrides, &primary_keys, &mut w)?;
          write!(w, "mut conn: Conn")?;
          write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;
          default_uses(use_async, true, true, &mut w)?;
          writeln!(
            w,
            r#"
                diesel::update({table}::table)
              "#,
            table = table.name
          )?;

          for i in &primary_keys {
            writeln!(
              w,
              r#"
                  .filter({table}::{column}.eq({column}))
                "#,
              table = table.name,
              column = i.name,
            )?;
          }

          writeln!(w, ".set((")?;

          for i in &timestamp_columns {
            if i.name == c.name {
              continue;
            }
            writeln!(
              w,
              r#"
                  {table}::{column}.eq(diesel::dsl::now),
                "#,
              table = table.name,
              column = i.name,
            )?;
          }

          if c.r#type.is_boolean_type() {
            writeln!(
              w,
              r#"
                  {table}::{column}.eq(true),
                "#,
              table = table.name,
              column = c.name,
            )?;
          } else if c.r#type.is_integer_type() {
            writeln!(
              w,
              r#"
                  {table}::{column}.eq(1),
                "#,
              table = table.name,
              column = c.name,
            )?;
          } else if c.r#type.is_nullable() {
            writeln!(
              w,
              r#"
                  {table}::{column}.eq(diesel::dsl::now),
                "#,
              table = table.name,
              column = c.name,
            )?;
          }
          writeln!(w, "))")?;

          writeln!(
            w,
            r#"
                .returning({model}::as_returning())
                .get_result::<{model}>(&mut conn)
                .await
              "#,
            model = final_model_name
          )?;
          writeln!(w, "}}")?;
        }
      }

      writeln!(w, "}}\n")?;
    }

    if enable_update
      && !table.only_primary_key_columns()
      && (per_column || whole_table)
    {
      if !updater_structs {
        return Err(anyhow::anyhow!(
          "updater_structs must be enabled to generate update functions"
        ));
      }

      writeln!(w, "impl {} {{", final_model_name)?;

      if whole_table {
        operation_sig(use_async, "update", backend, None, &mut w)?;
        write_ref_fn_params(ref_type_overrides, &primary_keys, &mut w)?;
        write!(w, "changes: & {}<'_>, ", final_updater_name)?;
        write!(w, "mut conn: Conn")?;
        write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;
        default_uses(use_async, true, true, &mut w)?;
        writeln!(
          w,
          r#"
              diesel::update({table}::table)
            "#,
          table = table.name
        )?;

        for i in &primary_keys {
          writeln!(
            w,
            r#"
                .filter({table}::{column}.eq({column}))
              "#,
            table = table.name,
            column = i.name,
          )?;
        }

        writeln!(
          w,
          r#"
              .set((
            "#,
        )?;

        for i in &timestamp_columns {
          writeln!(
            w,
            r#"
                {table}::{column}.eq(diesel::dsl::now),
              "#,
            table = table.name,
            column = i.name,
          )?;
        }

        writeln!(
          w,
          r#"
              changes,))
            "#
        )?;

        writeln!(
          w,
          r#"
              .returning({model}::as_returning())
              .get_result::<{model}>(&mut conn)
              .await
            "#,
          model = final_model_name
        )?;
        writeln!(w, "}}")?;
      }

      if per_column {
        for c in &non_primary_key_columns {
          let field_name = get_field_name(table_config, &c.name);

          operation_sig(
            use_async,
            &format!(
              "update_{}",
              field_name.strip_prefix("r#").unwrap_or(&field_name)
            ),
            backend,
            None,
            &mut w,
          )?;

          write_ref_fn_params(ref_type_overrides, &primary_keys, &mut w)?;
          write!(
            w,
            "{}: {}, ",
            &c.name,
            get_ref_type(ref_type_overrides, &c.r#type, None).ok_or_else(
              || { anyhow::anyhow!("Unknown type: {}", c.r#type.to_string()) }
            )?
          )?;
          write!(w, "mut conn: Conn")?;
          write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;
          default_uses(use_async, true, true, &mut w)?;
          writeln!(
            w,
            r#"
                diesel::update({table}::table)
              "#,
            table = table.name
          )?;

          for i in &primary_keys {
            writeln!(
              w,
              r#"
                  .filter({table}::{column}.eq({column}))
                "#,
              table = table.name,
              column = i.name,
            )?;
          }

          writeln!(w, ".set((")?;

          for i in &timestamp_columns {
            if i.name == c.name {
              continue;
            }
            writeln!(
              w,
              r#"
                  {table}::{column}.eq(diesel::dsl::now),
                "#,
              table = table.name,
              column = i.name,
            )?;
          }

          writeln!(
            w,
            r#"
                {table}::{column}.eq({column}),))
              "#,
            table = table.name,
            column = c.name,
          )?;

          writeln!(
            w,
            r#"
                .returning({model}::as_returning())
                .get_result::<{model}>(&mut conn)
                .await
              "#,
            model = final_model_name
          )?;
          writeln!(w, "}}")?;
        }
      }

      writeln!(w, "}}\n")?;
    }

    if enable_insert {
      if !inserter_structs {
        return Err(anyhow::anyhow!(
          "inserter_structs must be enabled to generate insert functions"
        ));
      }
      writeln!(w, "impl {} {{", final_model_name)?;
      operation_sig(use_async, "insert", backend, None, &mut w)?;
      write_ref_fn_params(ref_type_overrides, &primary_keys, &mut w)?;
      write!(w, "data: &{}<'_>, ", final_inserter_name)?;
      write!(w, "mut conn: Conn")?;
      write!(w, "\n) -> Result<Self, diesel::result::Error> {{")?;

      default_uses(use_async, true, false, &mut w)?;
      writeln!(
        w,
        r#"
            diesel::insert_into({table}::table)
              .values(data)
              .returning({model}::as_returning())
              .get_result::<{model}>(&mut conn)
              .await
          "#,
        table = table.name,
        model = final_model_name
      )?;

      writeln!(w, "}}")?;

      writeln!(w, "}}\n")?;
    }
  }
  Ok(())
}
