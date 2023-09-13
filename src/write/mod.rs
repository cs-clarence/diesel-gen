#![allow(dead_code)]

pub mod async_graphql;

use std::{
  collections::{HashMap, HashSet},
  io::Write,
};

use inflector::Inflector;

use crate::{
  config::{OrderingOptionsConfig, SqlBackend, TableConfig},
  parse::{self, Column, File, ParseContext, Table},
  util::{get_field_name, get_ref_type, get_type, model_name},
};

pub fn rust_file_headers<W: Write>(mut writer: W) -> std::io::Result<()> {
  writeln!(writer, "// @generated automatically by diesel-gen\n")?;

  writeln!(writer, "#![allow(unused)]")?;
  writeln!(writer, "#![allow(clippy::all)]\n")?;

  Ok(())
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

pub struct TypeUsesArgs<'a> {
  pub tables: &'a Vec<Table>,
  pub types_uses: &'a HashMap<String, String>,
  pub type_overrides: &'a HashMap<String, String>,
}

pub fn type_uses<W: Write>(
  args: &TypeUsesArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let mut uses_set = HashSet::new();

  for table in args.tables {
    for c in &table.columns {
      if let Some(str) = get_type(args.type_overrides, &c.r#type) {
        let tp = parse::r#type(&mut ParseContext::new(&str))?;

        for i in tp.type_names() {
          if let Some(ty) = args.types_uses.get(&i.to_string().replace(' ', ""))
          {
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
}

pub fn models<W: Write>(
  &ModelsArgs {
    file,
    backend,
    ref_type_overrides,
    type_overrides,
    table_configs,
  }: &ModelsArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let wildcard_table_config = table_configs.get("*");

  for table in &file.tables {
    let table_config = table_configs.get(&table.name).or(wildcard_table_config);

    if let Some(conf) = table_config {
      if conf.skip.unwrap_or(false) {
        continue;
      }
    }

    model(
      &ModelArgs {
        backend,
        ref_type_overrides,
        type_overrides,
        table,
        table_config,
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
  cols: Option<&Vec<String>>,
) -> Vec<&'a Column> {
  const DEF_COL_NAMES: [&str; 4] = [
    "updated_at",
    "datetime_updated",
    "date_updated",
    "datetime_deleted",
  ];

  if let Some(cols) = cols {
    cols
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
  ref_type_overrides: &HashMap<String, String>,
  cols: &Vec<&Column>,
  lifetime: Option<&str>,
  mut w: W,
) -> std::io::Result<()> {
  for c in cols {
    write!(
      w,
      "{}: {}, ",
      c.name,
      if c.r#type.is_simple() {
        get_type(type_overrides, &c.r#type).expect("Unknown type encountered")
      } else {
        get_ref_type(ref_type_overrides, &c.r#type, lifetime)
          .expect("Unknown type encountered")
      }
    )?;
  }

  Ok(())
}

fn operation_sig<W: Write>(
  name: &str,
  conn_t_name: &str,
  lifetimes: Option<Vec<&str>>,
  mut w: W,
) -> std::io::Result<()> {
  writeln!(w, "  pub fn {}<", name)?;
  if let Some(lifetimes) = lifetimes {
    writeln!(w, "{}, ", lifetimes.join(", "))?;
  }
  writeln!(w, "{}>(", conn_t_name)?;

  Ok(())
}

fn operation_contraints<W: Write>(
  use_async: bool,
  conn_t_name: &str,
  backend: &SqlBackend,
  mut w: W,
) -> std::io::Result<()> {
  if use_async {
    writeln!(
      w,
      " where {}: diesel_async::AsyncConnection<Backend = {}> + Send",
      conn_t_name,
      backend.path()
    )?;
  } else {
    writeln!(
      w,
      " where {}: diesel::Connection<Backend = {}> + Send ",
      conn_t_name,
      backend.path()
    )?;
  }

  Ok(())
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd)]
struct DefaultUsesArgs {
  use_async: bool,
  query_dsl: bool,
  selectable_helper: bool,
  expression_methods: bool,
  sql_types: bool,
}

fn default_operation_uses<W: Write>(
  args: &DefaultUsesArgs,
  mut w: W,
) -> std::io::Result<()> {
  if args.selectable_helper {
    writeln!(w, "use diesel::SelectableHelper;")?;
  }

  if args.query_dsl {
    writeln!(w, "use diesel::QueryDsl;")?;
  }

  if args.expression_methods {
    writeln!(w, "use diesel::ExpressionMethods;")?;
  }

  if args.use_async {
    writeln!(w, "use diesel_async::RunQueryDsl;")?;
  } else {
    writeln!(w, "use diesel::RunQueryDsl;")?;
  }

  if args.sql_types {
    writeln!(w, "use diesel::sql_types::*;")?;
  }

  Ok(())
}

pub struct ModelArgs<'a> {
  pub table: &'a Table,
  pub table_config: Option<&'a TableConfig>,
  pub backend: &'a SqlBackend,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
}

pub fn model<W: Write>(
  &ModelArgs {
    backend,
    ref_type_overrides,
    table,
    table_config,
    type_overrides,
  }: &ModelArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let optional_updater_fields = table_config
    .and_then(|t| t.updater_fields_optional)
    .unwrap_or(true);

  let d = table_config.and_then(|t| t.model_derives.clone());

  if let Some(mut d) = d {
    d.dedup();
    if !d.is_empty() {
      writeln!(w, "#[derive({})]", d.vec().join(", "))?;
    }
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

  writeln!(w, "#[diesel(check_for_backend({}))]", backend.path())?;

  let inserter_prefix = table_config
    .and_then(|t| t.inserter_struct_name_prefix.as_deref())
    .unwrap_or("New");

  let inserter_suffix = table_config
    .and_then(|t| t.inserter_struct_name_suffix.as_deref())
    .unwrap_or("");

  let updater_prefix = table_config
    .and_then(|t| t.updater_struct_name_prefix.as_deref())
    .unwrap_or("");

  let updater_suffix = table_config
    .and_then(|t| t.updater_struct_name_suffix.as_deref())
    .unwrap_or("Update");

  let model_prefix = table_config
    .and_then(|t| t.model_struct_name_prefix.as_deref())
    .unwrap_or("");

  let model_suffix = table_config
    .and_then(|t| t.model_struct_name_suffix.as_deref())
    .unwrap_or("");

  let struct_name = model_name(&table.name);

  let final_model_name =
    format!("{}{}{}", model_prefix, struct_name, model_suffix);
  let final_inserter_name =
    format!("{}{}{}", inserter_prefix, struct_name, inserter_suffix);
  let final_updater_name =
    format!("{}{}{}", updater_prefix, struct_name, updater_suffix);

  let a = table_config.and_then(|t| t.attributes.clone());

  if let Some(mut a) = a {
    a.dedup();

    if !a.is_empty() {
      for a in a {
        writeln!(w, "#[{}]", a)?;
      }
    }
  }

  writeln!(w, "pub struct {} {{", final_model_name)?;

  for c in &table.columns {
    let config = table_config.and_then(|t| t.columns.get(&c.name));

    if let Some(c) = config {
      if c.omit_in_model.unwrap_or(false) {
        continue;
      }

      if let Some(a) = &c.model_attributes {
        for a in a {
          writeln!(w, "  #[{}]", a)?;
        }
      }
    }

    let field_name = get_field_name(config, &c.name);

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

  let inserter_structs =
    table_config.and_then(|t| t.inserter_struct).unwrap_or(true);

  if inserter_structs {
    let d = table_config.and_then(|t| t.inserter_derives.clone());

    if let Some(mut d) = d {
      d.dedup();
      if !d.is_empty() {
        writeln!(w, "#[derive({})]", d.vec().join(", "))?;
      }
    }

    writeln!(w, "{}", DIESEL_INSERTER_DERIVE)?;
    writeln!(w, "#[diesel(table_name = {})]", table.name)?;
    writeln!(w, "#[diesel(check_for_backend({}))]", backend.path())?;

    let a = table_config.and_then(|t| t.inserter_attributes.clone());

    if let Some(mut a) = a {
      a.dedup();

      if !a.is_empty() {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }
    }

    let lifetime = "'a";
    writeln!(w, "pub struct {}<{}>{{", final_inserter_name, lifetime)?;
    for c in &table.columns {
      let config = table_config.and_then(|t| t.columns.get(&c.name));

      if let Some(c) = config {
        if c.omit_in_inserter.unwrap_or(false) {
          continue;
        }

        if let Some(a) = &c.inserter_attributes {
          for a in a {
            writeln!(w, "  #[{}]", a)?;
          }
        }
      }

      let field_name = get_field_name(config, &c.name);

      if field_name != c.name {
        writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
      }

      let ty = if c.r#type.is_simple() {
        get_type(type_overrides, &c.r#type)
      } else {
        get_ref_type(ref_type_overrides, &c.r#type, Some(lifetime))
      }
      .ok_or_else(|| {
        anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
      })?;

      writeln!(w, "  pub {}: {},", field_name, ty,)?;
    }
    writeln!(w, "}}\n")?;
  }

  let non_primary_key_columns = table.non_primary_key_columns();

  let updater_structs =
    table_config.and_then(|t| t.updater_struct).unwrap_or(true);

  if updater_structs && !table.only_primary_key_columns() {
    let d = table_config.and_then(|t| t.updater_derives.clone());

    if let Some(mut d) = d {
      d.dedup();
      if !d.is_empty() {
        writeln!(w, "#[derive({})]", d.vec().join(", "))?;
      }
    }

    writeln!(w, "{}", DIESEL_UPDATER_DERIVE)?;
    writeln!(w, "#[diesel(table_name = {})]", table.name)?;
    writeln!(w, "#[diesel(check_for_backend({}))]", backend.path())?;

    let a = table_config.and_then(|t| t.updater_attributes.clone());

    if let Some(mut a) = a {
      a.dedup();
      if !a.is_empty() {
        for a in a {
          writeln!(w, "#[{}]", a)?;
        }
      }
    }

    let lifetime = "'a";
    writeln!(w, "pub struct {}<{}>{{", final_updater_name, lifetime)?;
    for c in &non_primary_key_columns {
      let config = table_config.and_then(|t| t.columns.get(&c.name));

      if let Some(c) = config {
        if c.omit_in_updater.unwrap_or(false) {
          continue;
        }

        if let Some(a) = &c.updater_attributes {
          for a in a {
            writeln!(w, "  #[{}]", a)?;
          }
        }
      }

      let field_name = get_field_name(config, &c.name);

      if field_name != c.name {
        writeln!(w, "  #[diesel(column_name = \"{}\")]", c.name)?;
      }

      let ty = if c.r#type.is_simple() {
        get_type(type_overrides, &c.r#type)
      } else {
        get_ref_type(ref_type_overrides, &c.r#type, Some(lifetime))
      }
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
      .and_then(|t| t.operations.as_ref())
      .cloned()
      .unwrap_or_default();

    let enable_operations = operations.enable.unwrap_or(false);

    if !enable_operations {
      return Ok(());
    }

    let use_async = operations.r#async.unwrap_or(true);

    let primary_keys = table.primary_key_columns();

    let update_config = operations.update.unwrap_or_default();
    let enable_update = update_config.enable.unwrap_or(true);
    let column_wise_update = update_config.column_wise_update.unwrap_or(true);
    let ut = update_config.update_timestamp_columns;
    let timestamp_columns =
      get_update_timestamp_columns(table, ut.map(|l| l.vec().clone()).as_ref());
    if !timestamp_columns.iter().all(|i| {
      i.r#type.is_datetime() || i.r#type.is_nullable_and(|t| t.is_datetime())
    }) {
      return Err(anyhow::anyhow!(
        "Only datetime columns are supported for update_timestamp_columns"
      ));
    }
    if enable_update && !table.only_primary_key_columns() {
      if !updater_structs {
        return Err(anyhow::anyhow!(
          "updater_structs must be enabled to generate update functions"
        ));
      }

      update(
        &UpdateArgs {
          type_overrides,
          model_name: &final_model_name,
          updater_name: &final_updater_name,
          use_async,
          ref_type_overrides,
          primary_keys: &primary_keys,
          non_primary_key_columns: &non_primary_key_columns,
          timestamp_columns: &timestamp_columns,
          table,
          table_config,
          backend,
          column_wise_update,
        },
        &mut w,
      )?;
    }

    let delete_config = operations.delete.unwrap_or_default();
    let enable_delete = delete_config.enable.unwrap_or(true);
    if enable_delete {
      delete(
        &DeleteArgs {
          type_overrides,
          model_name: &final_model_name,
          use_async,
          ref_type_overrides,
          primary_keys: &primary_keys,
          table,
          timestamp_columns: &timestamp_columns,
          backend,
        },
        &mut w,
      )?;
    }

    let soft_delete_config = operations.soft_delete.unwrap_or_default();
    let soft_delete_column = soft_delete_config.soft_delete_column;
    let enable_soft_delete = soft_delete_config.enable.unwrap_or(true);
    let soft_delete_column =
      get_soft_delete_column(table, soft_delete_column.as_deref());

    if enable_soft_delete {
      if let Some(c) = soft_delete_column {
        soft_delete(
          &SoftDeleteArgs {
            type_overrides,
            model_name: &final_model_name,
            use_async,
            ref_type_overrides,
            primary_keys: &primary_keys,
            table,
            timestamp_columns: &timestamp_columns,
            backend,
            soft_delete_column: c,
          },
          &mut w,
        )?;
      }
    }

    let insert_config = operations.insert.unwrap_or_default();
    let enable_insert = insert_config.enable.unwrap_or(true);
    if enable_insert {
      if !inserter_structs {
        return Err(anyhow::anyhow!(
          "inserter_structs must be enabled to generate insert functions"
        ));
      }

      insert(
        &InsertArgs {
          model_name: &final_model_name,
          inserter_name: &final_inserter_name,
          use_async,
          table,
          table_config,
          backend,
          primary_keys: &primary_keys,
          type_overrides,
          ref_type_overrides,
        },
        &mut w,
      )?;
    }

    let simple_paginate_config = operations.simple_paginate.unwrap_or_default();
    let enable_simple_paginate = simple_paginate_config.enable.unwrap_or(true);
    if enable_simple_paginate {
      simple_paginate(
        &SimplePaginateArgs {
          ordering_options: &simple_paginate_config
            .ordering_options
            .unwrap_or(OrderingOptionsConfig::All),
          columns: &table.columns,
          model_name: &final_model_name,
          use_async,
          table,
          backend,
          include_soft_deleted: simple_paginate_config
            .include_soft_deleted
            .unwrap_or(false),
          soft_delete_column,
        },
        &mut w,
      )?;
    }

    let cursor_paginate_config = operations.cursor_paginate.unwrap_or_default();
    let enable_cursor_paginate = cursor_paginate_config.enable.unwrap_or(true);
    if enable_cursor_paginate {
      cursor_paginate(
        &CursorPaginateArgs {
          model_name: &final_model_name,
          use_async,
          table,
          backend,
          include_soft_deleted: cursor_paginate_config
            .include_soft_deleted
            .unwrap_or(false),
          soft_delete_column,
        },
        &mut w,
      )?;
    }
  }
  Ok(())
}

struct InsertArgs<'a> {
  pub model_name: &'a str,
  pub inserter_name: &'a str,
  pub use_async: bool,
  pub table: &'a Table,
  pub table_config: Option<&'a TableConfig>,
  pub backend: &'a SqlBackend,
  pub primary_keys: &'a Vec<&'a Column>,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
}

fn insert<W: Write>(args: &InsertArgs<'_>, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  operation_sig("insert", "Conn", Some(vec!["'a"]), &mut w)?;
  write!(w, "data: &'a {}<'a>, ", args.inserter_name)?;
  write!(w, "conn: &'a mut Conn")?;

  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, false, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;

  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  write!(w, "{{")?;

  default_operation_uses(
    &DefaultUsesArgs {
      query_dsl: false,
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: false,
      sql_types: false,
    },
    &mut w,
  )?;
  writeln!(
    w,
    "
      diesel::insert_into({table}::table)
        .values(data)
        .returning({model}::as_returning())
        .get_result::<{model}>(conn)
    ",
    table = args.table.name,
    model = args.model_name
  )?;

  writeln!(w, "}}")?;

  writeln!(w, "}}\n")?;
  Ok(())
}

struct UpdateArgs<'a> {
  model_name: &'a str,
  updater_name: &'a str,
  use_async: bool,
  type_overrides: &'a HashMap<String, String>,
  ref_type_overrides: &'a HashMap<String, String>,
  primary_keys: &'a Vec<&'a Column>,
  non_primary_key_columns: &'a Vec<&'a Column>,
  timestamp_columns: &'a Vec<&'a Column>,
  table: &'a Table,
  table_config: Option<&'a TableConfig>,
  backend: &'a SqlBackend,
  column_wise_update: bool,
}

fn update<W: Write>(args: &UpdateArgs<'_>, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;

  operation_sig("update", "Conn", Some(vec!["'a"]), &mut w)?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    Some("'a"),
    &mut w,
  )?;
  write!(w, "changes: &'a {}<'a>, ", args.updater_name)?;
  write!(w, "conn: &'a mut Conn")?;
  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, false, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;

  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  write!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      use_async: args.use_async,
      query_dsl: false,
      selectable_helper: true,
      expression_methods: true,
      sql_types: false,
    },
    &mut w,
  )?;
  writeln!(w, "diesel::update({table}::table)", table = args.table.name)?;

  for i in args.primary_keys {
    writeln!(
      w,
      ".filter({table}::{column}.eq({column}))",
      table = args.table.name,
      column = i.name,
    )?;
  }

  writeln!(w, ".set((",)?;

  for i in args.timestamp_columns {
    writeln!(
      w,
      "{table}::{column}.eq(diesel::dsl::now),",
      table = args.table.name,
      column = i.name,
    )?;
  }

  writeln!(w, "changes,))")?;

  writeln!(
    w,
    ".returning({model}::as_returning()).get_result::<{model}>(conn)",
    model = args.model_name
  )?;
  writeln!(w, "}}")?;

  if args.column_wise_update {
    for c in args.non_primary_key_columns {
      let config = args.table_config.and_then(|t| t.columns.get(&c.name));
      let field_name = get_field_name(config, &c.name);

      operation_sig(
        &format!(
          "update_{}",
          field_name.strip_prefix("r#").unwrap_or(&field_name)
        ),
        "Conn",
        Some(vec!["'a"]),
        &mut w,
      )?;

      write_ref_fn_params(
        args.type_overrides,
        args.ref_type_overrides,
        args.primary_keys,
        Some("'a"),
        &mut w,
      )?;

      let ty = if c.r#type.is_simple() {
        get_type(args.type_overrides, &c.r#type)
      } else {
        get_ref_type(args.ref_type_overrides, &c.r#type, Some("'a"))
      }
      .ok_or_else(|| {
        anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
      })?;

      write!(w, "{}: {}, ", &c.name, ty,)?;
      write!(w, "conn: &'a mut Conn")?;

      write!(
        w,
        "\n  ) -> {}{}",
        return_type(args.use_async, false, args.model_name),
        if args.use_async { " + 'a" } else { "" },
      )?;
      operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
      write!(w, "{{")?;
      default_operation_uses(
        &DefaultUsesArgs {
          use_async: args.use_async,
          query_dsl: false,
          selectable_helper: true,
          expression_methods: true,
          sql_types: false,
        },
        &mut w,
      )?;
      writeln!(w, "diesel::update({table}::table)", table = args.table.name)?;

      for i in args.primary_keys {
        writeln!(
          w,
          ".filter({table}::{column}.eq({column}))",
          table = args.table.name,
          column = i.name,
        )?;
      }

      writeln!(w, ".set((")?;

      for i in args.timestamp_columns {
        if i.name == c.name {
          continue;
        }
        writeln!(
          w,
          "{table}::{column}.eq(diesel::dsl::now),",
          table = args.table.name,
          column = i.name,
        )?;
      }

      writeln!(
        w,
        "{table}::{column}.eq({column}),))",
        table = args.table.name,
        column = c.name,
      )?;

      writeln!(
        w,
        ".returning({model}::as_returning()).get_result::<{model}>(conn)",
        model = args.model_name
      )?;
      writeln!(w, "}}")?;
    }
  }

  writeln!(w, "}}\n")?;

  Ok(())
}

struct DeleteArgs<'a> {
  model_name: &'a str,
  use_async: bool,
  type_overrides: &'a HashMap<String, String>,
  ref_type_overrides: &'a HashMap<String, String>,
  primary_keys: &'a Vec<&'a Column>,
  table: &'a Table,
  timestamp_columns: &'a Vec<&'a Column>,
  backend: &'a SqlBackend,
}

fn delete<W: Write>(args: &DeleteArgs<'_>, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  operation_sig("delete", "Conn", Some(vec!["'a"]), &mut w)?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    Some("'a"),
    &mut w,
  )?;
  write!(w, "conn: &'a mut Conn")?;

  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, false, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;

  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  write!(w, "{{")?;

  default_operation_uses(
    &DefaultUsesArgs {
      query_dsl: false,
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: true,
      sql_types: false,
    },
    &mut w,
  )?;
  writeln!(w, "diesel::delete({table}::table)", table = args.table.name)?;

  for i in args.primary_keys {
    writeln!(
      w,
      ".filter({table}::{column}.eq({column}))",
      table = args.table.name,
      column = i.name,
    )?;
  }

  writeln!(
    w,
    ".returning({model}::as_returning()).get_result::<{model}>(conn)",
    model = args.model_name
  )?;
  writeln!(w, "}}")?;

  writeln!(w, "}}\n")?;

  Ok(())
}

struct SoftDeleteArgs<'a> {
  model_name: &'a str,
  use_async: bool,
  type_overrides: &'a HashMap<String, String>,
  ref_type_overrides: &'a HashMap<String, String>,
  primary_keys: &'a Vec<&'a Column>,
  table: &'a Table,
  timestamp_columns: &'a Vec<&'a Column>,
  backend: &'a SqlBackend,
  soft_delete_column: &'a Column,
}

fn soft_delete<W: Write>(
  args: &SoftDeleteArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  let c = args.soft_delete_column;
  if !(c.r#type.is_boolean()
    || c.r#type.is_nullable_and(|t| t.is_datetime())
    || c.r#type.is_integer())
  {
    return Err(anyhow::anyhow!(
              "Unsupported soft delete column type '{}' of column '{}' in table '{}'. Supported class of types are boolean, datetime, integer",
              c.r#type,
              c.name,
              args.table.name
            ));
  }

  operation_sig("soft_delete", "Conn", Some(vec!["'a"]), &mut w)?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    Some("'a"),
    &mut w,
  )?;
  write!(w, "conn: &'a mut Conn")?;
  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, false, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;
  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  write!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      query_dsl: false,
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: true,
      sql_types: false,
    },
    &mut w,
  )?;
  writeln!(w, "diesel::update({table}::table)", table = args.table.name)?;

  for i in args.primary_keys {
    writeln!(
      w,
      ".filter({table}::{column}.eq({column}))",
      table = args.table.name,
      column = i.name,
    )?;
  }

  writeln!(w, ".set((")?;

  for i in args.timestamp_columns {
    if i.name == c.name {
      continue;
    }
    writeln!(
      w,
      "{table}::{column}.eq(diesel::dsl::now),",
      table = args.table.name,
      column = i.name,
    )?;
  }

  if c.r#type.is_boolean() {
    writeln!(
      w,
      "{table}::{column}.eq(true),",
      table = args.table.name,
      column = c.name,
    )?;
  } else if c.r#type.is_integer() {
    writeln!(
      w,
      "{table}::{column}.eq(1),",
      table = args.table.name,
      column = c.name,
    )?;
  } else if c.r#type.is_nullable() {
    writeln!(
      w,
      "{table}::{column}.eq(diesel::dsl::now),",
      table = args.table.name,
      column = c.name,
    )?;
  }
  writeln!(w, "))")?;

  writeln!(
    w,
    ".returning({model}::as_returning()).get_result::<{model}>(conn)",
    model = args.model_name
  )?;
  writeln!(w, "}}")?;

  writeln!(w, "}}\n")?;

  Ok(())
}

struct SimplePaginateArgs<'a> {
  model_name: &'a str,
  use_async: bool,
  table: &'a Table,
  columns: &'a Vec<Column>,
  ordering_options: &'a OrderingOptionsConfig,
  backend: &'a SqlBackend,
  include_soft_deleted: bool,
  soft_delete_column: Option<&'a Column>,
}

fn simple_paginate<W: Write>(
  args: &SimplePaginateArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let order_enum_name = format!("{}OrderBy", args.model_name);
  writeln!(w, "pub enum {} {{", &order_enum_name)?;
  match args.ordering_options {
    OrderingOptionsConfig::None => {}
    OrderingOptionsConfig::All => {
      for c in args.columns {
        writeln!(w, "{}Asc,", c.name.to_pascal_case())?;
        writeln!(w, "{}Desc,", c.name.to_pascal_case())?;
      }
    }
    OrderingOptionsConfig::AllAsc => {
      for c in args.columns {
        writeln!(w, "{}Asc,", c.name.to_pascal_case())?;
      }
    }
    OrderingOptionsConfig::AllDesc => {
      for c in args.columns {
        writeln!(w, "{}Desc,", c.name.to_pascal_case())?;
      }
    }
    OrderingOptionsConfig::Columns(order_configs) => {
      for col in args.columns {
        if let Some(order) = order_configs.get(&col.name) {
          match order {
            crate::config::Order::Asc => {
              writeln!(w, "{}Asc,", col.name.to_pascal_case())?;
            }
            crate::config::Order::Desc => {
              writeln!(w, "{}Desc,", col.name.to_pascal_case())?;
            }
            crate::config::Order::Both => {
              writeln!(w, "{}Asc,", col.name.to_pascal_case())?;
              writeln!(w, "{}Desc,", col.name.to_pascal_case())?;
            }
          }
        }
      }
    }
  }

  writeln!(w, "}}",)?;

  writeln!(w, "impl {} {{", args.model_name)?;
  operation_sig("simple_paginate", "Conn", Some(vec!["'a"]), &mut w)?;

  if let OrderingOptionsConfig::None = args.ordering_options {
    write!(w, "offset: u32, limit: u32, conn: &'a mut Conn")?;
  } else {
    write!(
      w,
      "offset: u32, limit: u32, ordering: Option<&Vec<{}>>, conn: &'a mut Conn",
      &order_enum_name
    )?;
  }

  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, true, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;
  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  writeln!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      query_dsl: true,
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: !args.include_soft_deleted
        || !matches!(args.ordering_options, OrderingOptionsConfig::None)
          && args.soft_delete_column.is_some(),
      sql_types: false,
    },
    &mut w,
  )?;

  let query_name = "q";

  if let OrderingOptionsConfig::None = args.ordering_options {
    writeln!(w, "{table}::table", table = args.table.name,)?;
  } else {
    writeln!(
      w,
      "let mut {} = {table}::table",
      query_name,
      table = args.table.name,
    )?;
  }

  if let Some(col) = args.soft_delete_column {
    if !args.include_soft_deleted {
      if col.r#type.is_boolean() {
        writeln!(
          w,
          ".filter({table}::{column}.eq(false))",
          table = args.table.name,
          column = col.name,
        )?;
      } else if col.r#type.is_integer() {
        writeln!(
          w,
          ".filter({table}::{column}.eq(0))",
          table = args.table.name,
          column = col.name,
        )?;
      } else if col.r#type.is_nullable_and(|t| t.is_datetime()) {
        writeln!(
          w,
          ".filter({table}::{column}.is_not_null())",
          table = args.table.name,
          column = col.name,
        )?;
      } else {
        return Err(anyhow::anyhow!(
          "Unsupported soft delete column type '{}' of column '{}' in table '{}'. Supported class of types are boolean, datetime, integer",
          col.r#type,
          col.name,
          args.table.name
        ));
      }
    }
  }

  if !matches!(args.ordering_options, OrderingOptionsConfig::None) {
    write!(w, ".into_boxed();")?;
  }

  fn order(
    idx_name: &str,
    query_name: &str,
    table: &str,
    column: &str,
    desc: bool,
  ) -> String {
    format!(
      "
      {{ 
        if {index} == 0 {{
          {query} = {query}.order_by({table}::{column}.{ordering}());
        }} else {{
          {query} = {query}.then_order_by({table}::{column}.{ordering}());
        }}
      }}
      ",
      index = idx_name,
      query = query_name,
      table = table,
      column = column,
      ordering = if desc { "desc" } else { "asc" }
    )
  }

  let idx = "idx";
  let ord = "ord";

  if !matches!(args.ordering_options, OrderingOptionsConfig::None) {
    writeln!(w, "if let Some(ordering) = ordering {{")?;
    writeln!(
      w,
      "for ({}, {}) in ordering.iter().enumerate() {{",
      idx, ord
    )?;
    writeln!(w, "match {} {{", ord)?;
    match args.ordering_options {
      OrderingOptionsConfig::None => {}
      OrderingOptionsConfig::All => {
        for c in args.columns {
          writeln!(
            w,
            "{}::{}Asc => {}",
            &order_enum_name,
            c.name.to_pascal_case(),
            order(idx, query_name, &args.table.name, &c.name, false)
          )?;
          writeln!(
            w,
            "{}::{}Desc =>  {}",
            &order_enum_name,
            c.name.to_pascal_case(),
            order(idx, query_name, &args.table.name, &c.name, true)
          )?;
        }
      }
      OrderingOptionsConfig::AllAsc => {
        for c in args.columns {
          writeln!(
            w,
            "{}::{}Asc => {}",
            &order_enum_name,
            c.name.to_pascal_case(),
            order(idx, query_name, &args.table.name, &c.name, false)
          )?;
        }
      }
      OrderingOptionsConfig::AllDesc => {
        for c in args.columns {
          writeln!(
            w,
            "{}::{}Desc => {}",
            &order_enum_name,
            c.name.to_pascal_case(),
            order(idx, query_name, &args.table.name, &c.name, true)
          )?;
        }
      }
      OrderingOptionsConfig::Columns(order_configs) => {
        for col in args.columns {
          if let Some(o) = order_configs.get(&col.name) {
            match o {
              crate::config::Order::Asc => {
                writeln!(
                  w,
                  "{}::{}Asc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, query_name, &args.table.name, &col.name, false)
                )?;
              }
              crate::config::Order::Desc => {
                writeln!(
                  w,
                  "{}::{}Desc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, query_name, &args.table.name, &col.name, true)
                )?;
              }
              crate::config::Order::Both => {
                writeln!(
                  w,
                  "{}::{}Asc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, query_name, &args.table.name, &col.name, false)
                )?;
                writeln!(
                  w,
                  "{}::{}Desc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, query_name, &args.table.name, &col.name, true)
                )?;
              }
            }
          }
        }
      }
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
  }

  if !matches!(args.ordering_options, OrderingOptionsConfig::None) {
    write!(w, "{}", query_name)?;
  }
  writeln!(
    w,
    ".offset(offset.into()).limit(limit.into()).select({model}::as_select()).load::<{model}>(conn)",
    model = args.model_name
  )?;

  writeln!(w, "}}")?;
  writeln!(w, "}}\n")?;
  Ok(())
}

struct CursorPaginateArgs<'a> {
  model_name: &'a str,
  use_async: bool,
  table: &'a Table,
  backend: &'a SqlBackend,
  include_soft_deleted: bool,
  soft_delete_column: Option<&'a Column>,
}

fn cursor_paginate<W: Write>(
  args: &CursorPaginateArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  operation_sig("cursor_paginate", "Conn", Some(vec!["'a"]), &mut w)?;
  write!(w, "offset: usize, limit: usize, conn: &'a mut Conn")?;

  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, true, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;

  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  writeln!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      query_dsl: false,
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: !args.include_soft_deleted
        && args.soft_delete_column.is_some(),
      sql_types: false,
    },
    &mut w,
  )?;

  writeln!(w, "{table}::table", table = args.table.name,)?;

  if let Some(col) = args.soft_delete_column {
    if !args.include_soft_deleted {
      if col.r#type.is_boolean() {
        writeln!(
          w,
          ".filter({table}::{column}.eq(false))",
          table = args.table.name,
          column = col.name,
        )?;
      } else if col.r#type.is_integer() {
        writeln!(
          w,
          ".filter({table}::{column}.eq(0))",
          table = args.table.name,
          column = col.name,
        )?;
      } else if col.r#type.is_nullable_and(|t| t.is_datetime()) {
        writeln!(
          w,
          ".filter({table}::{column}.is_not_null())",
          table = args.table.name,
          column = col.name,
        )?;
      } else {
        return Err(anyhow::anyhow!(
          "Unsupported soft delete column type '{}' of column '{}' in table '{}'. Supported class of types are boolean, datetime, integer",
          col.r#type,
          col.name,
          args.table.name
        ));
      }
    }
  }

  writeln!(w, ".select({model}::as_select()).offset(offset).limit(limit).load::<{model}>(conn)",
    model = args.model_name
  )?;

  writeln!(w, "}}")?;
  writeln!(w, "}}\n")?;
  Ok(())
}

fn return_type(use_async: bool, multiple: bool, model_name: &str) -> String {
  if use_async {
    if multiple {
      format!(
      "impl std::future::Future<Output = Result<Vec<{model}>, diesel::result::Error>> + Send",
      model = model_name
    )
    } else {
      format!(
      "impl std::future::Future<Output = Result<{model}, diesel::result::Error>> + Send",
      model = model_name
    )
    }
  } else if multiple {
    format!(
      "Result<Vec<{model}>, diesel::result::Error>",
      model = model_name
    )
  } else {
    format!("Result<{model}, diesel::result::Error>", model = model_name)
  }
}
