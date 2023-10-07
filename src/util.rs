use std::{collections::HashMap, sync::OnceLock};

use inflector::Inflector;

use crate::{
  config::ColumnConfig,
  parse::{self, type_name, ParseContext, Type, TypeName},
};

pub fn remove_spaces_from_keys(
  map: &HashMap<String, String>,
) -> HashMap<String, String> {
  map
    .iter()
    .map(|(k, v)| (k.replace(' ', ""), v.clone()))
    .collect::<HashMap<String, String>>()
}

pub fn to_rust_path(path: &str) -> String {
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

pub fn is_rust_keyword(str: &str) -> bool {
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

pub fn get_field_name(
  column_config: Option<&ColumnConfig>,
  column_name: &str,
) -> String {
  fn imp(cc: Option<&ColumnConfig>, column_name: &str) -> String {
    if let Some(column) = cc {
      return column.rename.as_deref().unwrap_or(column_name).to_string();
    }

    column_name.to_string()
  }

  let field_name = imp(column_config, column_name);

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

   BYTEA => "Vec<u8>",
   BINARY =>"Vec<u8>",
   VARBINARY => "Vec<u8>",
   BLOB => "Vec<u8>",
   TINYBLOB => "Vec<u8>",
   MEDIUMBLOB => "Vec<u8>",
   LONGBLOB => "Vec<u8>",
   BIT => "u8",
   INET => "ipnetwork::IpNetwork",
   TINYTEXT => "String",
   MEDIUMTEXT => "String",
   LONGTEXT => "String",
  });
}

pub fn get_type(
  type_overrides: &HashMap<String, String>,
  ty: &Type,
  column_config: Option<&ColumnConfig>,
) -> Option<String> {
  init_type_map();

  if let Some(col) = column_config {
    if let Some(r#override) = &col.type_override {
      return Some(
        parse::r#type(&mut ParseContext::new(r#override))
          .expect("Could not parse type")
          .to_string(),
      );
    }
  }

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
      .map(|i| get_type(type_overrides, i, None))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("{}<{}>", t, params.join(", ")));
  }

  None
}

pub fn get_ref_type(
  type_overrides: &HashMap<String, String>,
  ty: &Type,
  column_config: Option<&ColumnConfig>,
  lifetime: Option<&str>,
) -> Option<String> {
  init_type_map();

  if let Some(conf) = column_config {
    if let Some(ft) = &conf.ref_type_override {
      let mut ty = parse::r#type(&mut ParseContext::new(ft))
        .expect("Could not parse type");
      if let Type::Borrowed { lifetime: lt, .. } = &mut ty {
        *lt = Some(
          lifetime
            .map(|l| l.strip_prefix('\'').unwrap().to_string())
            .unwrap_or_else(|| "static".to_string()),
        );
      }

      return Some(ty.to_string());
    }
  }

  let lt = if let Some(lifetime) = lifetime {
    if lifetime.is_empty() {
      "".to_string()
    } else {
      format!("{} ", lifetime)
    }
  } else {
    "".to_string()
  };

  let ts = ty.to_string().replace(' ', "");

  if let Some(ty) = type_overrides.get(&ts) {
    let mut ty =
      parse::r#type(&mut ParseContext::new(ty)).expect("Could not parse type");

    if let Type::Borrowed { lifetime, .. } = &mut ty {
      *lifetime =
        Some(lifetime.clone().unwrap_or_else(|| "static".to_string()));
    }

    return Some(ty.to_string());
  }

  if ty.name().is_string_type() {
    return Some(format!("&{}str", lt));
  }

  if ty.name().is_byte_array_type() {
    return Some(format!("&{}[u8]", lt));
  }

  if *ty.name() == TypeName::Nullable {
    let params = ty
      .params()
      .iter()
      .map(|i| get_ref_type(type_overrides, i, None, Some(&lt)))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("Option<{}>", params.join(", ")));
  }

  let tp_map = RUST_TYPE_MAP.get().expect("RUST_TYPE_MAP not initialized");

  if let Some(ty) = tp_map.get(ts.as_str()) {
    return Some(format!("&{}{}", lt, ty));
  }

  if let Some(t) = tp_map.get(&ty.name().to_string().as_str()) {
    let params = ty
      .params()
      .iter()
      .map(|i| get_type(type_overrides, i, None))
      .collect::<Vec<_>>();

    if params.iter().any(|i| i.is_none()) {
      return None;
    }

    let params = params.into_iter().map(|i| i.unwrap()).collect::<Vec<_>>();

    return Some(format!("&{}{}<{}>", lt, t, params.join(", ")));
  }

  None
}
