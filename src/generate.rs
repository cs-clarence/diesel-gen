use std::{
  collections::HashMap, io::Write, path::PathBuf, str::FromStr, sync::OnceLock,
};

use inflector::Inflector;

use crate::{
  config::{DieselConfig, ListConfig, ModelsConfig},
  parse::{type_name, Column, File, Table, Type, TypeName},
};

fn write_rust_file_headers<W: Write>(mut writer: W) -> std::io::Result<()> {
  writeln!(writer, "// @generated automatically by diesel-gen\n")?;

  writeln!(writer, "#![allow(unused)]")?;
  writeln!(writer, "#![allow(clippy::all)]\n")?;

  Ok(())
}

fn to_singular_pascal_case(text: &str) -> String {
  text.to_pascal_case().to_singular()
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

fn get_field_name(
  model: &ModelsConfig,
  table_name: &str,
  column_name: &str,
) -> String {
  fn imp(model: &ModelsConfig, table_name: &str, column_name: &str) -> String {
    if model.tables.is_none() {
      return column_name.to_string();
    }

    let tables = model.tables.as_ref().unwrap();

    let table = tables.get(table_name);
    let wildcard = tables.get("*");

    if let Some(table) = table {
      if let Some(columns) = table.columns.as_ref() {
        if let Some(column) = columns.get(column_name) {
          return column.rename.as_deref().unwrap_or(column_name).to_string();
        }
      }
    }

    if let Some(table) = wildcard {
      if let Some(columns) = table.columns.as_ref() {
        if let Some(column) = columns.get(column_name) {
          return column.rename.as_deref().unwrap_or(column_name).to_string();
        }
      }
    }

    column_name.to_string()
  }

  let field_name = imp(model, table_name, column_name);

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

  if let Some(t) = tp_map.get(&ty.name.to_string().as_str()) {
    let params = ty
      .params
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

  if ty.name.is_string_type() {
    return Some(format!("&{}str", lifetime));
  }

  if ty.name == TypeName::Nullable {
    let params = ty
      .params
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

  if let Some(t) = tp_map.get(&ty.name.to_string().as_str()) {
    let params = ty
      .params
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

pub fn generate_models<W: Write>(
  file: &File,
  config: &DieselConfig,
  model: &ModelsConfig,
  mut w: W,
) -> anyhow::Result<()> {
  const DIESEL_DEFAULT_DERIVE: &str =
    "#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable)]";

  const DIESEL_DEFAULT_WITH_CHANGESET_DERIVE: &str =
    "#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]";

  const DIESEL_INSERTER_DERIVE: &str = "#[derive(diesel::Insertable)]";

  const DIESEL_UPDATER_DERIVE: &str = "#[derive(diesel::AsChangeset)]";

  let struct_prefix = model
    .struct_names_prefix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let struct_suffix = model
    .struct_names_suffix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let inserter_prefix = model
    .inserter_struct_names_prefix
    .as_ref()
    .cloned()
    .unwrap_or("New".to_string());

  let inserter_suffix = model
    .inserter_struct_name_suffix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let updater_prefix = model
    .updater_structs_name_prefix
    .as_ref()
    .cloned()
    .unwrap_or("".to_string());

  let updater_suffix = model
    .updater_structs_name_suffix
    .as_ref()
    .cloned()
    .unwrap_or("Update".to_string());

  let table_configs = model.tables.as_ref().cloned().unwrap_or_default();

  let wildcard_table_config =
    table_configs.get("*").cloned().unwrap_or_default();

  let wildcard_derives = wildcard_table_config.derives.unwrap_or_default();

  let wildcard_inserter_derives =
    wildcard_table_config.inserter_derives.unwrap_or_default();

  let wildcard_updater_derives =
    wildcard_table_config.updater_derives.unwrap_or_default();

  let wildcard_attributes =
    wildcard_table_config.attributes.unwrap_or_default();

  let wildcard_inserter_attributes = wildcard_table_config
    .inserter_attributes
    .unwrap_or_default();

  let wildcard_updater_attributes =
    wildcard_table_config.updater_attributes.unwrap_or_default();

  // remove spaces from type overrides to make it easier to match
  let type_overrides = model
    .type_overrides
    .as_ref()
    .cloned()
    .unwrap_or_default()
    .iter()
    .map(|(x, y)| {
      (
        x.replace(' ', "").to_string(),
        y.replace(' ', "").to_string(),
      )
    })
    .collect::<HashMap<String, String>>();

  let backend = model.backend.as_ref().map(|b| b.path());

  // remove spaces from type overrides to make it easier to match
  let ref_type_overrides = model
    .ref_type_overrides
    .as_ref()
    .cloned()
    .unwrap_or_default()
    .iter()
    .map(|(x, y)| {
      (
        x.replace(' ', "").to_string(),
        y.replace(' ', "").to_string(),
      )
    })
    .collect::<HashMap<String, String>>();

  let optinal_updater_fields = model.updater_fields_optional.unwrap_or(true);

  let import_root = model.table_imports_root.as_ref().cloned().unwrap_or(
    config
      .print_schema
      .as_ref()
      .cloned()
      .unwrap_or_default()
      .file
      .unwrap_or(PathBuf::from_str("./schema.rs").unwrap())
      .to_str()
      .unwrap()
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
      .join("::"),
  );

  write_rust_file_headers(&mut w)?;

  let operations = wildcard_table_config
    .operations
    .as_ref()
    .cloned()
    .unwrap_or_default();

  let enable_operations = operations.enable.unwrap_or(false);

  let delete_config = operations.delete.unwrap_or_default();
  let enable_delete = delete_config.enable.unwrap_or(enable_operations);
  let soft_delete = delete_config.soft_delete.unwrap_or(true);
  let hard_delete = delete_config.hard_delete.unwrap_or(true);
  let soft_delete_column = delete_config.soft_delete_column;

  let update_config = operations.update.unwrap_or_default();
  let enable_update = update_config.enable.unwrap_or(enable_operations);
  let per_column = update_config.per_column.unwrap_or(true);
  let whole_table = update_config.whole_table.unwrap_or(true);
  let update_timestamps = update_config.update_timestamp_columns;

  let insert_config = operations.insert.unwrap_or_default();
  let enable_insert = insert_config.enable.unwrap_or(enable_operations);

  let use_async = operations.r#async.unwrap_or(true);

  if let Some(module) = file.module.as_deref() {
    writeln!(w, "use {}::{}::{{", import_root, module)?;
  } else {
    writeln!(w, "use {}::{{", import_root)?;
  }

  for t in &file.tables {
    if let Some(config) = table_configs.get(&t.name) {
      if config.skip.unwrap_or(false) {
        continue;
      }
    }

    writeln!(w, "  {},", &t.name)?;
  }

  writeln!(w, "}};\n")?;

  if let Some(ref imports) = model.uses {
    for i in imports {
      writeln!(w, "use {};", i)?;
    }
  }

  if let Some(ref forward_imports) = model.pub_uses {
    for i in forward_imports {
      writeln!(w, "pub use {};", i)?;
    }
  }

  if let Some(ref mods) = model.mods {
    for m in mods {
      writeln!(w, "mod {};", m)?;
    }
  }

  if let Some(ref forward_mods) = model.pub_mods {
    for m in forward_mods {
      writeln!(w, "pub mod {};", m)?;
    }
  }

  for t in &file.tables {
    let table_config = table_configs.get(&t.name).cloned().unwrap_or_default();

    if table_config.skip.unwrap_or(false) {
      continue;
    }

    let mut d =
      wildcard_derives.combine(&table_config.derives.unwrap_or_default());
    d.dedup();
    if !d.is_empty() {
      writeln!(w, "#[derive({})]", d.vec().join(", "))?;
    }

    if t.only_primary_key_columns() {
      writeln!(w, "{}", DIESEL_DEFAULT_DERIVE)?;
    } else {
      writeln!(w, "{}", DIESEL_DEFAULT_WITH_CHANGESET_DERIVE)?;
    }

    writeln!(w, "#[diesel(table_name = {})]", t.name)?;
    writeln!(w, "#[diesel(primary_key({}))]", t.primary_key.join(", "))?;

    if let Some(b) = backend {
      writeln!(w, "#[diesel(check_for_backend({}))]", b)?;
    }

    let struct_name = to_singular_pascal_case(&t.name);

    let final_struct_name =
      format!("{}{}{}", struct_prefix, struct_name, struct_suffix);
    let final_inserter_name =
      format!("{}{}{}", inserter_prefix, struct_name, inserter_suffix);
    let final_updater_name =
      format!("{}{}{}", updater_prefix, struct_name, updater_suffix);

    let mut a =
      wildcard_attributes.combine(&table_config.attributes.unwrap_or_default());

    a.dedup();

    if !a.is_empty() {
      for a in a {
        writeln!(w, "#[{}]", a)?;
      }
    }

    writeln!(w, "pub struct {} {{", final_struct_name)?;

    for c in &t.columns {
      let field_name = get_field_name(model, &t.name, &c.name);

      if field_name != c.name {
        writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
      }
      writeln!(
        w,
        "  pub {}: {},",
        field_name,
        get_type(&type_overrides, &c.r#type).ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      )?;
    }

    writeln!(w, "}}\n")?;

    let inserter_structs = model.inserter_structs.unwrap_or(true);

    if inserter_structs {
      let mut d = wildcard_inserter_derives
        .combine(&table_config.inserter_derives.unwrap_or_default());
      d.dedup();
      if !d.is_empty() {
        writeln!(w, "#[derive({})]", d.vec().join(", "))?;
      }

      writeln!(w, "{}", DIESEL_INSERTER_DERIVE)?;
      writeln!(w, "#[diesel(table_name = {})]", t.name)?;
      if let Some(b) = backend {
        writeln!(w, "#[diesel(check_for_backend({}))]", b)?;
      }

      let mut a = wildcard_inserter_attributes
        .combine(&table_config.inserter_attributes.unwrap_or_default());

      a.dedup();

      if !a.is_empty() {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }

      let lifetime = "'a";
      writeln!(w, "pub struct {}<{}>{{", final_inserter_name, lifetime)?;
      for c in &t.columns {
        let field_name = get_field_name(model, &t.name, &c.name);

        if field_name != c.name {
          writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
        }
        writeln!(
          w,
          "  pub {}: {},",
          field_name,
          get_ref_type(&ref_type_overrides, &c.r#type, Some(lifetime))
            .ok_or_else(|| {
              anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
            },)?
        )?;
      }
      writeln!(w, "}}\n")?;
    }

    let non_primary_key_columns = t.non_primary_key_columns();

    let updater_structs = model.updater_structs.unwrap_or(true);

    if updater_structs && !t.only_primary_key_columns() {
      let mut d = wildcard_updater_derives
        .combine(&table_config.updater_derives.unwrap_or_default());

      d.dedup();
      if !d.is_empty() {
        writeln!(w, "#[derive({})]", d.vec().join(", "))?;
      }
      writeln!(w, "{}", DIESEL_UPDATER_DERIVE)?;
      writeln!(w, "#[diesel(table_name = {})]", t.name)?;
      if let Some(b) = backend {
        writeln!(w, "#[diesel(check_for_backend({}))]", b)?;
      }

      let mut a = wildcard_updater_attributes
        .combine(&table_config.updater_attributes.unwrap_or_default());

      a.dedup();

      if !a.is_empty() {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }

      let lifetime = "'a";
      writeln!(w, "pub struct {}<{}>{{", final_updater_name, lifetime)?;
      for c in &non_primary_key_columns {
        let field_name = get_field_name(model, &t.name, &c.name);

        if field_name != c.name {
          writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
        }
        let ty = get_ref_type(&ref_type_overrides, &c.r#type, Some(lifetime))
          .ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?;
        writeln!(
          w,
          "  pub {}: {},",
          field_name,
          if optinal_updater_fields {
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

      let enable_operations = operations.enable.unwrap_or(enable_operations);

      if !enable_operations {
        return Ok(());
      }

      let delete_config = operations.delete.unwrap_or_default();
      let enable_delete = delete_config
        .enable
        .unwrap_or(enable_operations && enable_delete);
      let soft_delete = delete_config.soft_delete.unwrap_or(soft_delete);
      let hard_delete = delete_config.hard_delete.unwrap_or(hard_delete);
      let soft_delete_column = delete_config
        .soft_delete_column
        .or(soft_delete_column.clone());

      let update_config = operations.update.unwrap_or_default();
      let enable_update = update_config
        .enable
        .unwrap_or(enable_operations && enable_update);

      let per_column = update_config.per_column.unwrap_or(per_column);
      let whole_table = update_config.whole_table.unwrap_or(whole_table);
      let ut = if let Some(e) = update_config.update_timestamp_columns {
        Some(e.combine(&update_timestamps.clone().unwrap_or_default()))
      } else {
        update_timestamps.clone()
      };

      let timestamp_columns = get_update_timestamp_columns(t, ut.as_ref());

      if !timestamp_columns.iter().all(|i| {
        i.r#type.is_datetime_type()
          || i.r#type.is_nullable_type(|t| t.is_datetime_type())
      }) {
        return Err(anyhow::anyhow!(
          "Only datetime columns are supported for update_timestamp_columns"
        ));
      }

      let insert_config = operations.insert.unwrap_or_default();
      let enable_insert = insert_config
        .enable
        .unwrap_or(enable_operations && enable_insert);

      let use_async = operations.r#async.unwrap_or(use_async);

      if backend.is_none() {
        return Err(anyhow::anyhow!("Backend not specified"));
      }

      let primary_keys = t.primary_key_columns();

      let backend = backend.unwrap();

      if enable_delete {
        writeln!(w, "impl {} {{", final_struct_name)?;
        if hard_delete {
          write_operation(use_async, "delete", backend, None, &mut w)?;
          write_ref_fn_params(&ref_type_overrides, &primary_keys, &mut w)?;
          write!(w, "mut conn: Conn")?;
          write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;

          write_default_uses(use_async, true, true, &mut w)?;
          writeln!(
            w,
            r#"
            diesel::delete({table}::table)
            "#,
            table = t.name
          )?;

          for i in &primary_keys {
            writeln!(
              w,
              r#"
                .filter({table}::{column}.eq({column}))
              "#,
              table = t.name,
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
            model = final_struct_name
          )?;
          writeln!(w, "}}")?;
        }

        if let Some(c) =
          get_soft_delete_column(t, soft_delete_column.as_deref())
        {
          if !(c.r#type.is_boolean_type()
            || c.r#type.is_nullable_type(|t| t.is_datetime_type())
            || c.r#type.is_integer_type())
          {
            return Err(anyhow::anyhow!(
              "Unsupported soft delete column type '{}' of column '{}' in table '{}'. Supported class of types are boolean, datetime, integer",
              c.r#type,
              c.name,
              t.name
            ));
          }

          if soft_delete {
            write_operation(use_async, "soft_delete", backend, None, &mut w)?;
            write_ref_fn_params(&ref_type_overrides, &primary_keys, &mut w)?;
            write!(w, "mut conn: Conn")?;
            write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;
            write_default_uses(use_async, true, true, &mut w)?;
            writeln!(
              w,
              r#"
                diesel::update({table}::table)
              "#,
              table = t.name
            )?;

            for i in &primary_keys {
              writeln!(
                w,
                r#"
                  .filter({table}::{column}.eq({column}))
                "#,
                table = t.name,
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
                table = t.name,
                column = i.name,
              )?;
            }

            if c.r#type.is_boolean_type() {
              writeln!(
                w,
                r#"
                  {table}::{column}.eq(true),
                "#,
                table = t.name,
                column = c.name,
              )?;
            } else if c.r#type.is_integer_type() {
              writeln!(
                w,
                r#"
                  {table}::{column}.eq(1),
                "#,
                table = t.name,
                column = c.name,
              )?;
            } else if c.r#type.is_nullable() {
              writeln!(
                w,
                r#"
                  {table}::{column}.eq(diesel::dsl::now),
                "#,
                table = t.name,
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
              model = final_struct_name
            )?;
            writeln!(w, "}}")?;
          }
        }

        writeln!(w, "}}\n")?;
      }

      if enable_update
        && !t.only_primary_key_columns()
        && (per_column || whole_table)
      {
        if !updater_structs {
          return Err(anyhow::anyhow!(
            "updater_structs must be enabled to generate update functions"
          ));
        }

        writeln!(w, "impl {} {{", final_struct_name)?;

        if whole_table {
          write_operation(use_async, "update", backend, None, &mut w)?;
          write_ref_fn_params(&ref_type_overrides, &primary_keys, &mut w)?;
          write!(w, "changes: & {}<'_>, ", final_updater_name)?;
          write!(w, "mut conn: Conn")?;
          write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;
          write_default_uses(use_async, true, true, &mut w)?;
          writeln!(
            w,
            r#"
              diesel::update({table}::table)
            "#,
            table = t.name
          )?;

          for i in &primary_keys {
            writeln!(
              w,
              r#"
                .filter({table}::{column}.eq({column}))
              "#,
              table = t.name,
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
              table = t.name,
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
            model = final_struct_name
          )?;
          writeln!(w, "}}")?;
        }

        if per_column {
          for c in &non_primary_key_columns {
            let field_name = get_field_name(model, &t.name, &c.name);

            write_operation(
              use_async,
              &format!(
                "update_{}",
                field_name.strip_prefix("r#").unwrap_or(&field_name)
              ),
              backend,
              None,
              &mut w,
            )?;

            write_ref_fn_params(&ref_type_overrides, &primary_keys, &mut w)?;
            write!(
              w,
              "{}: {}, ",
              &c.name,
              get_ref_type(&ref_type_overrides, &c.r#type, None).ok_or_else(
                || {
                  anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
                }
              )?
            )?;
            write!(w, "mut conn: Conn")?;
            write!(w, "\n  ) -> Result<Self, diesel::result::Error> {{")?;
            write_default_uses(use_async, true, true, &mut w)?;
            writeln!(
              w,
              r#"
                diesel::update({table}::table)
              "#,
              table = t.name
            )?;

            for i in &primary_keys {
              writeln!(
                w,
                r#"
                  .filter({table}::{column}.eq({column}))
                "#,
                table = t.name,
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
                table = t.name,
                column = i.name,
              )?;
            }

            writeln!(
              w,
              r#"
                {table}::{column}.eq({column}),))
              "#,
              table = t.name,
              column = c.name,
            )?;

            writeln!(
              w,
              r#"
                .returning({model}::as_returning())
                .get_result::<{model}>(&mut conn)
                .await
              "#,
              model = final_struct_name
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
        writeln!(w, "impl {} {{", final_struct_name)?;
        write_operation(use_async, "insert", backend, None, &mut w)?;
        write_ref_fn_params(&ref_type_overrides, &primary_keys, &mut w)?;
        write!(w, "data: &{}<'_>, ", final_inserter_name)?;
        write!(w, "mut conn: Conn")?;
        write!(w, "\n) -> Result<Self, diesel::result::Error> {{")?;

        write_default_uses(use_async, true, false, &mut w)?;
        writeln!(
          w,
          r#"
            diesel::insert_into({table}::table)
              .values(data)
              .returning({model}::as_returning())
              .get_result::<{model}>(&mut conn)
              .await
          "#,
          table = t.name,
          model = final_struct_name
        )?;

        writeln!(w, "}}")?;

        writeln!(w, "}}\n")?;
      }
    }
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

fn write_operation<W: Write>(
  use_async: bool,
  name: &str,
  backend: &str,
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

fn write_default_uses<W: Write>(
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
