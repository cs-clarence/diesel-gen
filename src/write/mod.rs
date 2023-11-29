#![allow(dead_code)]

pub mod async_graphql;

use std::{
  collections::{HashMap, HashSet},
  io::Write,
};

use inflector::Inflector;
use merge::Merge;

use crate::{
  config::{
    ColumnConfig, CursorConfig, ListConfig, OrderingOptionsConfig, SqlBackend,
    TableConfig,
  },
  parse::{self, Column, File, ParseContext, Table},
  util::{
    final_inserter_name, final_model_name, final_updater_name, get_field_name,
    get_ref_type, get_type,
  },
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
  config: Option<&TableConfig>,
  types_uses: &HashMap<String, String>,
  type_overrides: &HashMap<String, String>,
  mut w: W,
) -> anyhow::Result<()> {
  let mut uses_set = HashSet::new();

  for c in &table.columns {
    let ty = config.and_then(|t| t.columns.get(&c.name));

    if let Some(str) = get_type(type_overrides, &c.r#type, ty) {
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
  pub configs: &'a HashMap<String, TableConfig>,
  pub types_uses: &'a HashMap<String, String>,
  pub type_overrides: &'a HashMap<String, String>,
}

pub fn type_uses<W: Write>(
  args: &TypeUsesArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let mut uses_set = HashSet::new();

  for table in args.tables {
    let config = args.configs.get(&table.name);

    for c in &table.columns {
      let config = config.and_then(|t| t.columns.get(&c.name));

      if let Some(str) = get_type(args.type_overrides, &c.r#type, config) {
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
    "#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Insertable, diesel::Selectable, diesel::Identifiable)]";

const DIESEL_DEFAULT_WITH_CHANGESET_DERIVE: &str =
    "#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]";

const DIESEL_INSERTER_DERIVE: &str = "#[derive(diesel::Insertable)]";

const DIESEL_UPDATER_DERIVE: &str = "#[derive(diesel::AsChangeset)]";

pub struct ModelsArgs<'a> {
  pub file: &'a File,
  pub table_imports_root: &'a str,
  pub backend: &'a SqlBackend,
  pub table_configs: &'a HashMap<String, TableConfig>,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
}

pub fn models<W: Write>(
  &ModelsArgs {
    table_imports_root,
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
        table_import_root: table_imports_root,
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
  configs: Option<&HashMap<String, ColumnConfig>>,
  lifetime: Option<&str>,
  mut w: W,
) -> anyhow::Result<()> {
  for col in cols {
    let config = configs.and_then(|t| t.get(&col.name));
    let rename = get_field_name(config, &col.name);

    write!(
      w,
      "{}: {}, ",
      rename,
      get_optimal_type(
        col,
        config,
        lifetime,
        type_overrides,
        ref_type_overrides
      )
      .ok_or_else(|| {
        anyhow::anyhow!("Unknown type: {}", col.r#type.to_string())
      })?
    )?;
  }

  Ok(())
}

struct FunctionSignatureArgs<'a> {
  use_async: bool,
  name: &'a str,
  generics: Option<Vec<&'a str>>,
}

fn function_signature<W: Write>(
  args: &FunctionSignatureArgs<'_>,
  mut w: W,
) -> std::io::Result<()> {
  writeln!(w, "  pub fn {}", args.name)?;
  if let Some(generics) = &args.generics {
    writeln!(w, "<{}>", generics.join(", "))?;
  }
  writeln!(w, "(",)?;

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
  into_sql: bool,
  bool_expression_methods: bool,
}

fn default_operation_uses<W: Write>(
  args: &DefaultUsesArgs,
  mut w: W,
) -> std::io::Result<()> {
  if args.selectable_helper {
    writeln!(w, "use diesel::SelectableHelper;")?;
  }

  if args.into_sql {
    writeln!(w, "use diesel::IntoSql;")?;
  }

  if args.query_dsl {
    writeln!(w, "use diesel::QueryDsl;")?;
  }

  if args.expression_methods {
    writeln!(w, "use diesel::ExpressionMethods;")?;
  }

  if args.bool_expression_methods {
    writeln!(w, "use diesel::prelude::BoolExpressionMethods;")?;
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
  pub table_import_root: &'a str,
  pub table_config: Option<&'a TableConfig>,
  pub backend: &'a SqlBackend,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
}

pub fn model<W: Write>(
  &ModelArgs {
    table_import_root: import_root,
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

  let final_model_name = final_model_name(table, table_config);
  let final_inserter_name = final_inserter_name(table, table_config);
  let final_updater_name = final_updater_name(table, table_config);

  let gen_model = table_config.and_then(|t| t.model).unwrap_or(true);

  if gen_model {
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

    let a = table_config.and_then(|t| t.model_attributes.clone());

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
        get_type(type_overrides, &c.r#type, config).ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      )?;
    }

    writeln!(w, "}}\n")?;
  }

  /*
   * ==================
   * INSERTER STRUCTS
   * ==================
   */

  let inserter_structs = table_config.and_then(|t| t.inserter).unwrap_or(true);
  let inserter_use_refs = table_config
    .and_then(|t| t.inserter_use_refs)
    .unwrap_or(true);
  let mut inserter_has_ref = false;

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
    let mut inserter_field_temps = Vec::new();
    for c in &table.columns {
      let config = table_config.and_then(|t| t.columns.get(&c.name));

      if let Some(c) = config {
        if c.omit_in_inserter.unwrap_or(false) {
          continue;
        }

        if let Some(a) = &c.inserter_attributes {
          for a in a {
            writeln!(inserter_field_temps, "  #[{}]", a)?;
          }
        }
      }

      let field_name = get_field_name(config, &c.name);

      if field_name != c.name {
        writeln!(
          inserter_field_temps,
          "  #[diesel(column_name = \"{}\")]",
          c.name
        )?;
      }

      let ty = if inserter_use_refs {
        get_optimal_type(
          c,
          config,
          Some(lifetime),
          type_overrides,
          ref_type_overrides,
        )
        .ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      } else {
        get_type(type_overrides, &c.r#type, config).ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      };

      writeln!(inserter_field_temps, "  pub {}: {},", field_name, ty,)?;
    }
    inserter_has_ref = inserter_field_temps.contains(&b'&');
    if inserter_has_ref && inserter_use_refs {
      writeln!(w, "pub struct {}<{}>{{", final_inserter_name, lifetime)?;
    } else {
      writeln!(w, "pub struct {}{{", final_inserter_name,)?;
    }
    w.write_all(&inserter_field_temps)?;
    writeln!(w, "}}\n")?;
  }

  /*
   * ==================
   * UPDATER STRUCTS
   * ==================
   */

  let non_primary_key_columns = table.non_primary_key_columns();
  let updater_structs =
    table_config.and_then(|t| t.updater_struct).unwrap_or(true);
  let mut updater_has_ref = false;
  let updater_use_refs = table_config
    .and_then(|t| t.updater_use_refs)
    .unwrap_or(true);

  let has_non_omitted_columns = non_primary_key_columns.iter().any(|c| {
    let config = table_config.and_then(|t| t.columns.get(&c.name));

    if let Some(c) = config {
      !c.omit_in_updater.unwrap_or(false)
    } else {
      true
    }
  });

  if updater_structs
    && !table.only_primary_key_columns()
    && has_non_omitted_columns
  {
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

    let mut updater_fields_tmp = Vec::new();
    for c in &non_primary_key_columns {
      let config = table_config.and_then(|t| t.columns.get(&c.name));

      if let Some(c) = config {
        if c.omit_in_updater.unwrap_or(false) {
          continue;
        }

        if let Some(a) = &c.updater_attributes {
          for a in a {
            writeln!(updater_fields_tmp, "  #[{}]", a)?;
          }
        }
      }

      let field_name = get_field_name(config, &c.name);

      if field_name != c.name {
        writeln!(
          updater_fields_tmp,
          "  #[diesel(column_name = \"{}\")]",
          c.name
        )?;
      }

      let ty = if updater_use_refs {
        get_optimal_type(
          c,
          config,
          Some(lifetime),
          type_overrides,
          ref_type_overrides,
        )
        .ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      } else {
        get_type(type_overrides, &c.r#type, config).ok_or_else(|| {
          anyhow::anyhow!("Unknown type: {}", c.r#type.to_string())
        })?
      };

      writeln!(
        updater_fields_tmp,
        "  pub {}: {},",
        field_name,
        if optional_updater_fields {
          format!("Option<{}>", ty)
        } else {
          ty
        }
      )?;
    }

    updater_has_ref = updater_fields_tmp.contains(&b'&');

    if updater_has_ref && updater_use_refs {
      writeln!(w, "pub struct {}<{}>{{", final_updater_name, lifetime)?;
    } else {
      writeln!(w, "pub struct {}{{", final_updater_name)?;
    }

    w.write_all(&updater_fields_tmp)?;

    writeln!(w, "}}\n")?;
  }
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
  if enable_update
    && !table.only_primary_key_columns()
    && has_non_omitted_columns
  {
    if !updater_structs {
      return Err(anyhow::anyhow!(
        "updater_structs must be enabled to generate update functions"
      ));
    }

    update(
      &UpdateArgs {
        type_overrides,
        model_name: &final_model_name,
        updater_has_ref,
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
        config: table_config,
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
          config: table_config,
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
        inserter_has_ref,
        inserter_name: &final_inserter_name,
        use_async,
        table,
        many: insert_config.many.unwrap_or(true),
        table_config,
        backend,
        primary_keys: &primary_keys,
        type_overrides,
        ref_type_overrides,
      },
      &mut w,
    )?;
  }

  let get_config = operations.get.unwrap_or_default();
  let enable_get = get_config.enable.unwrap_or(true);

  if enable_get {
    get(
      &GetArgs {
        many: get_config.many.unwrap_or(true),
        primary_keys: &primary_keys,
        soft_delete_column,
        model_name: &final_model_name,
        table_config,
        table,
        type_overrides,
        ref_type_overrides,
        backend,
        use_async,
      },
      &mut w,
    )?;
  }

  let simple_paginate_config = operations.paginate.unwrap_or_default();
  let derives = simple_paginate_config
    .order_by_enum_derives
    .as_ref()
    .map(|e| e.vec());
  let enable_simple_paginate = simple_paginate_config.enable.unwrap_or(true);
  if enable_simple_paginate {
    simple_paginate(
      &SimplePaginateArgs {
        order_by_enum_derives: derives,
        ordering_options: &simple_paginate_config
          .ordering_options
          .unwrap_or(OrderingOptionsConfig::All),
        columns: &table.columns,
        model_name: &final_model_name,
        use_async,
        table,
        backend,
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
        default_cursor_derives: cursor_paginate_config
          .default_cursor_derives
          .as_ref(),
        table_imports_root: import_root,
        table_config,
        type_overrides,
        cursors: cursor_paginate_config.cursors.map(),
        model_name: &final_model_name,
        use_async,
        table,
        backend,
        soft_delete_column,
      },
      &mut w,
    )?;
  }

  let count_config = operations.count.unwrap_or_default();
  let enable_count = count_config.enable.unwrap_or(true);

  if enable_count {
    count(
      &CountArgs {
        model_name: &final_model_name,
        use_async,
        table,
        backend,
        soft_delete_column,
      },
      &mut w,
    )?;
  }
  Ok(())
}

struct InsertArgs<'a> {
  pub model_name: &'a str,
  pub inserter_name: &'a str,
  pub inserter_has_ref: bool,
  pub use_async: bool,
  pub table: &'a Table,
  pub many: bool,
  pub table_config: Option<&'a TableConfig>,
  pub backend: &'a SqlBackend,
  pub primary_keys: &'a Vec<&'a Column>,
  pub type_overrides: &'a HashMap<String, String>,
  pub ref_type_overrides: &'a HashMap<String, String>,
}

fn get_optimal_type(
  column: &Column,
  config: Option<&ColumnConfig>,
  lifetime: Option<&str>,
  type_overrides: &HashMap<String, String>,
  ref_type_overrides: &HashMap<String, String>,
) -> Option<String> {
  if column.r#type.is_simple() {
    get_type(type_overrides, &column.r#type, config)
  } else {
    get_ref_type(ref_type_overrides, &column.r#type, config, lifetime)
  }
}

fn insert<W: Write>(args: &InsertArgs<'_>, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "insert",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;

  if args.inserter_has_ref {
    write!(w, "data: &'a {}<'a>, ", args.inserter_name)?;
  } else {
    write!(w, "data: &'a {}, ", args.inserter_name)?;
  }
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
      into_sql: false,
      ..Default::default()
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

  if args.many {
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "insert_many",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;

    if args.inserter_has_ref {
      write!(w, "data: &'a [{}<'a>], ", args.inserter_name)?;
    } else {
      write!(w, "data: &'a [{}], ", args.inserter_name)?;
    }
    write!(w, "conn: &'a mut Conn")?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
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
        into_sql: false,
        ..Default::default()
      },
      &mut w,
    )?;
    writeln!(
      w,
      "
      diesel::insert_into({table}::table)
        .values(data)
        .returning({model}::as_returning())
        .get_results::<{model}>(conn)
    ",
      table = args.table.name,
      model = args.model_name
    )?;
    writeln!(w, "}}\n")?;
  }

  writeln!(w, "}}\n")?;
  Ok(())
}

struct UpdateArgs<'a> {
  model_name: &'a str,
  updater_has_ref: bool,
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

  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "update",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    args.table_config.map(|t| t.columns.map()),
    Some("'a"),
    &mut w,
  )?;

  if args.updater_has_ref {
    write!(w, "changes: &'a {}<'a>, ", args.updater_name)?;
  } else {
    write!(w, "changes: {}, ", args.updater_name)?;
  }
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
      into_sql: false,
      ..Default::default()
    },
    &mut w,
  )?;
  writeln!(w, "diesel::update({table}::table)", table = args.table.name)?;

  for i in args.primary_keys {
    let config = args.table_config.and_then(|t| t.columns.get(&i.name));
    let rename = get_field_name(config, &i.name);

    writeln!(
      w,
      ".filter({table}::{column}.eq({rename}))",
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

      function_signature(
        &FunctionSignatureArgs {
          use_async: args.use_async,
          name: &format!(
            "update_{}",
            field_name.strip_prefix("r#").unwrap_or(&field_name)
          ),
          generics: Some(vec!["'a", "Conn"]),
        },
        &mut w,
      )?;

      write_ref_fn_params(
        args.type_overrides,
        args.ref_type_overrides,
        args.primary_keys,
        args.table_config.map(|t| t.columns.map()),
        Some("'a"),
        &mut w,
      )?;

      let ty = get_optimal_type(
        c,
        config,
        Some("'a"),
        args.type_overrides,
        args.ref_type_overrides,
      )
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
          into_sql: false,
          ..Default::default()
        },
        &mut w,
      )?;
      writeln!(w, "diesel::update({table}::table)", table = args.table.name)?;

      for i in args.primary_keys {
        let config = args.table_config.and_then(|t| t.columns.get(&i.name));
        let rename = get_field_name(config, &i.name);

        writeln!(
          w,
          ".filter({table}::{column}.eq({rename}))",
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
  config: Option<&'a TableConfig>,
  timestamp_columns: &'a Vec<&'a Column>,
  backend: &'a SqlBackend,
}

fn delete<W: Write>(args: &DeleteArgs<'_>, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "delete",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    args.config.map(|t| t.columns.map()),
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
      into_sql: false,
      ..Default::default()
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
  config: Option<&'a TableConfig>,
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

  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "soft_delete",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    args.config.map(|t| t.columns.map()),
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
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: true,
      ..Default::default()
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
  order_by_enum_derives: Option<&'a Vec<String>>,
  soft_delete_column: Option<&'a Column>,
}

fn simple_paginate<W: Write>(
  args: &SimplePaginateArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  let order_enum_name = format!("{}OrderBy", args.model_name);
  if let Some(derives) = args.order_by_enum_derives {
    writeln!(w, "#[derive({})]", derives.join(", "))?;
  }
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
  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "paginate_extend",
      generics: Some(vec!["'a", "F", "Conn"]),
    },
    &mut w,
  )?;

  const EXTEND_NAME: &str = "extend";
  if let OrderingOptionsConfig::None = args.ordering_options {
    write!(
      w,
      "limit: usize, offset: usize, {EXTEND_NAME}: F, conn: &'a mut Conn",
    )?;
  } else {
    write!(
      w,
      "limit: usize, offset: usize, ordering: Option<&'a Vec<{order_enum_name}>>, {EXTEND_NAME}: F, conn: &'a mut Conn",
    )?;
  }

  write!(
    w,
    "\n  ) -> {}{}",
    return_type(args.use_async, true, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;
  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  writeln!(w, ",")?;
  writeln!(
    w,
    "
    F: for<'b> Fn(
      {table}::BoxedQuery<'b, {backend}>,
    ) -> {table}::BoxedQuery<'b, {backend}>,
  ",
    table = args.table.name,
    backend = args.backend.path(),
  )?;
  writeln!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      query_dsl: true,
      use_async: args.use_async,
      selectable_helper: true,
      expression_methods: !matches!(
        args.ordering_options,
        OrderingOptionsConfig::None
      ) && args.soft_delete_column.is_some(),
      ..Default::default()
    },
    &mut w,
  )?;

  const QUERY_NAME: &str = "q";

  if let OrderingOptionsConfig::None = args.ordering_options {
    writeln!(w, "{table}::table", table = args.table.name,)?;
  } else {
    writeln!(
      w,
      "let mut {} = {table}::table",
      QUERY_NAME,
      table = args.table.name,
    )?;
  }

  if args.soft_delete_column.is_some() {
    writeln!(
      w,
      ".{}",
      soft_delete_filter(&SoftDeleteFilterArgs {
        table_name: &args.table.name,
        soft_delete_column: args.soft_delete_column,
      })?
    )?;
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
            order(idx, QUERY_NAME, &args.table.name, &c.name, false)
          )?;
          writeln!(
            w,
            "{}::{}Desc =>  {}",
            &order_enum_name,
            c.name.to_pascal_case(),
            order(idx, QUERY_NAME, &args.table.name, &c.name, true)
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
            order(idx, QUERY_NAME, &args.table.name, &c.name, false)
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
            order(idx, QUERY_NAME, &args.table.name, &c.name, true)
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
                  order(idx, QUERY_NAME, &args.table.name, &col.name, false)
                )?;
              }
              crate::config::Order::Desc => {
                writeln!(
                  w,
                  "{}::{}Desc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, QUERY_NAME, &args.table.name, &col.name, true)
                )?;
              }
              crate::config::Order::Both => {
                writeln!(
                  w,
                  "{}::{}Asc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, QUERY_NAME, &args.table.name, &col.name, false)
                )?;
                writeln!(
                  w,
                  "{}::{}Desc => {}",
                  &order_enum_name,
                  col.name.to_pascal_case(),
                  order(idx, QUERY_NAME, &args.table.name, &col.name, true)
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
    write!(
      w,
      "q = {extend_name}({query_name}.offset(offset.try_into().unwrap()).limit(limit.try_into().unwrap()));",
      extend_name = EXTEND_NAME,
      query_name = QUERY_NAME
    )?;
  }
  writeln!(
    w,
    "{query_name}.select({model}::as_select()).load::<{model}>(conn)",
    query_name = QUERY_NAME,
    model = args.model_name
  )?;

  writeln!(w, "}}")?;

  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "paginate",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;

  if let OrderingOptionsConfig::None = args.ordering_options {
    write!(w, "offset: u32, limit: u32, conn: &'a mut Conn",)?;
  } else {
    write!(
      w,
      "offset: usize, limit: usize, ordering: Option<&'a Vec<{enum_name}>>, conn: &'a mut Conn",
      enum_name = &order_enum_name,
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
  writeln!(
    w,
    "{model_name}::paginate_extend(offset, limit, ordering, |q| q, conn)",
    model_name = args.model_name
  )?;
  writeln!(w, "}}")?;

  if args.soft_delete_column.is_some() {
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;

    const EXTEND_NAME: &str = "extend";
    if let OrderingOptionsConfig::None = args.ordering_options {
      write!(
        w,
        "limit: usize, offset: usize, {EXTEND_NAME}: F, conn: &'a mut Conn",
      )?;
    } else {
      write!(
      w,
      "limit: usize, offset: usize, ordering: Option<&'a Vec<{order_enum_name}>>, {EXTEND_NAME}: F, conn: &'a mut Conn",
    )?;
    }

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
    F: for<'b> Fn(
      {table}::BoxedQuery<'b, {backend}>,
    ) -> {table}::BoxedQuery<'b, {backend}>,
  ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        query_dsl: true,
        use_async: args.use_async,
        selectable_helper: true,
        expression_methods: !matches!(
          args.ordering_options,
          OrderingOptionsConfig::None
        ) && args.soft_delete_column.is_some(),
        ..Default::default()
      },
      &mut w,
    )?;

    const QUERY_NAME: &str = "q";

    if let OrderingOptionsConfig::None = args.ordering_options {
      writeln!(w, "{table}::table", table = args.table.name,)?;
    } else {
      writeln!(
        w,
        "let mut {} = {table}::table",
        QUERY_NAME,
        table = args.table.name,
      )?;
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
              order(idx, QUERY_NAME, &args.table.name, &c.name, false)
            )?;
            writeln!(
              w,
              "{}::{}Desc =>  {}",
              &order_enum_name,
              c.name.to_pascal_case(),
              order(idx, QUERY_NAME, &args.table.name, &c.name, true)
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
              order(idx, QUERY_NAME, &args.table.name, &c.name, false)
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
              order(idx, QUERY_NAME, &args.table.name, &c.name, true)
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
                    order(idx, QUERY_NAME, &args.table.name, &col.name, false)
                  )?;
                }
                crate::config::Order::Desc => {
                  writeln!(
                    w,
                    "{}::{}Desc => {}",
                    &order_enum_name,
                    col.name.to_pascal_case(),
                    order(idx, QUERY_NAME, &args.table.name, &col.name, true)
                  )?;
                }
                crate::config::Order::Both => {
                  writeln!(
                    w,
                    "{}::{}Asc => {}",
                    &order_enum_name,
                    col.name.to_pascal_case(),
                    order(idx, QUERY_NAME, &args.table.name, &col.name, false)
                  )?;
                  writeln!(
                    w,
                    "{}::{}Desc => {}",
                    &order_enum_name,
                    col.name.to_pascal_case(),
                    order(idx, QUERY_NAME, &args.table.name, &col.name, true)
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
      write!(
      w,
      "q = {extend_name}({query_name}.offset(offset.try_into().unwrap()).limit(limit.try_into().unwrap()));",
      extend_name = EXTEND_NAME,
      query_name = QUERY_NAME
    )?;
    }
    writeln!(
      w,
      "{query_name}.select({model}::as_select()).load::<{model}>(conn)",
      query_name = QUERY_NAME,
      model = args.model_name
    )?;

    writeln!(w, "}}")?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;

    if let OrderingOptionsConfig::None = args.ordering_options {
      write!(w, "offset: u32, limit: u32, conn: &'a mut Conn",)?;
    } else {
      write!(
      w,
      "offset: usize, limit: usize, ordering: Option<&'a Vec<{enum_name}>>, conn: &'a mut Conn",
      enum_name = &order_enum_name,
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
    writeln!(
    w,
    "{model_name}::paginate_with_soft_deleted_extend(offset, limit, ordering, |q| q, conn)",
    model_name = args.model_name
  )?;
    writeln!(w, "}}")?;
  }

  writeln!(w, "}}")?;
  Ok(())
}

struct CursorPaginateArgs<'a> {
  table_imports_root: &'a str,
  model_name: &'a str,
  use_async: bool,
  table: &'a Table,
  table_config: Option<&'a TableConfig>,
  cursors: &'a HashMap<String, CursorConfig>,
  backend: &'a SqlBackend,
  default_cursor_derives: Option<&'a ListConfig<String>>,
  type_overrides: &'a HashMap<String, String>,
  soft_delete_column: Option<&'a Column>,
}

fn cursor_paginate<W: Write>(
  args: &CursorPaginateArgs<'_>,
  mut w: W,
) -> anyhow::Result<()> {
  if args.cursors.is_empty() {
    return Ok(());
  }
  for (name, config) in args.cursors {
    if config.columns.is_empty() {
      return Err(anyhow::anyhow!(
        "Cursor '{}' has no columns specified",
        name
      ));
    }
  }

  for (name, config) in args.cursors {
    match (args.default_cursor_derives, config.derives.as_ref()) {
      (None, None) => {}
      (None, Some(derives)) => {
        writeln!(w, "#[derive({})]", derives.vec().join(", "))?;
      }
      (Some(def_derives), None) => {
        writeln!(w, "#[derive({})]", def_derives.vec().join(", "))?;
      }
      (Some(def_derives), Some(derives)) => {
        let mut def_derives = def_derives.clone();
        def_derives.merge(derives.clone());
        writeln!(w, "#[derive({})]", def_derives.vec().join(", "))?;
      }
    }

    let cursor_name = name.to_pascal_case();
    writeln!(
      w,
      "
      pub struct {cursor_name} {{
      ",
    )?;

    let mut field_names = HashMap::new();

    for c in config.columns.iter() {
      let col = args.table.get_column(c.name()).ok_or({
        anyhow::anyhow!(
          "Unknown column '{}' in table '{}'",
          c.name(),
          args.table.name
        )
      })?;
      let config = args.table_config.and_then(|t| t.columns.get(&col.name));

      let ty = get_type(args.type_overrides, &col.r#type, config).ok_or_else(
        || anyhow::anyhow!("Unknown type: {}", col.r#type.to_string()),
      )?;

      let name = get_field_name(
        args.table_config.and_then(|t| t.columns.get(&col.name)),
        c.name(),
      );

      field_names.insert(c.name(), name.clone());

      writeln!(w, "pub {}: {},", name, ty)?;
    }

    writeln!(w, "}}",)?;

    writeln!(
      w,
      "
      impl From<{model_name}> for {cursor_name} {{
      ",
      cursor_name = name.to_pascal_case(),
      model_name = args.model_name,
    )?;
    writeln!(
      w,
      "fn from(value: {model_name}) -> Self {{",
      model_name = args.model_name,
    )?;
    writeln!(w, "Self {{")?;
    for f in field_names.values() {
      writeln!(w, "{field_name}: value.{field_name},", field_name = f)?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
  }

  for (name, config) in args.cursors {
    let cursor_name = name.to_pascal_case();
    writeln!(w, "impl {} {{", &cursor_name)?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(
      w,
      "{{
      {cursor_name}::paginate_extend(after, before, limit, offset, |q| q, conn)
     }}"
    )?;

    const EXTEND_NAME: &str = "extend";
    const QUERY_NAME: &str = "q";
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, {EXTEND_NAME}: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        into_sql: true,
        selectable_helper: true,
        expression_methods: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "let create_query = || {{")?;

    let mut ordering = vec![];
    for (idx, col) in config.columns.iter().enumerate() {
      let name = col.name();
      match col.order() {
        crate::config::CursorColumnOrder::Asc => {
          writeln!(
            ordering,
            ".{order_fn}({table}::{column}.asc())",
            order_fn = if idx != 0 {
              "then_order_by"
            } else {
              "order_by"
            },
            table = args.table.name,
            column = name,
          )?;
        }
        crate::config::CursorColumnOrder::Desc => {
          writeln!(
            ordering,
            ".{order_fn}({table}::{column}.desc())",
            order_fn = if idx != 0 {
              "then_order_by"
            } else {
              "order_by"
            },
            table = args.table.name,
            column = name,
          )?;
        }
        crate::config::CursorColumnOrder::None => {}
      }
    }
    let ordering = String::from_utf8_lossy(&ordering);

    writeln!(
      w,
      "let mut {QUERY_NAME} = {table}::table{ordering}",
      table = args.table.name,
    )?;
    if args.soft_delete_column.is_some() {
      writeln!(
        w,
        ".{}",
        soft_delete_filter(&SoftDeleteFilterArgs {
          table_name: &args.table.name,
          soft_delete_column: args.soft_delete_column,
        })?
      )?;
    }
    writeln!(w, ".into_boxed();")?;

    let mut next_filter = String::new();
    let mut prev_filter = String::new();
    for (idx, cursor_col) in config.columns.iter().enumerate().rev() {
      let col = args.table.get_column(cursor_col.name()).unwrap();

      let field_name = get_field_name(
        args.table_config.and_then(|t| t.columns.get(&col.name)),
        cursor_col.name(),
      );
      let cursor_field = format!("{}.{}", "cursor", field_name);

      if idx == config.columns.len() - 1 {
        match cursor_col.order() {
          crate::config::CursorColumnOrder::Desc => {
            next_filter = format!(
              "{table}::{column}.lt(&{cursor_field})",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
            prev_filter = format!(
              "{table}::{column}.gt(&{cursor_field})",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
          }
          crate::config::CursorColumnOrder::Asc
          | crate::config::CursorColumnOrder::None => {
            next_filter = format!(
              "{table}::{column}.gt(&{cursor_field})",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
            prev_filter = format!(
              "{table}::{column}.lt(&{cursor_field})",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
          }
        }
      } else {
        match cursor_col.order() {
          crate::config::CursorColumnOrder::Desc => {
            next_filter = format!(
              "{table}::{column}.le(&{cursor_field}).and({table}::{column}.lt(&{cursor_field}).or({next_filter}))",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
            prev_filter = format!(
              "{table}::{column}.ge(&{cursor_field}).and({table}::{column}.gt(&{cursor_field}).or({prev_filter}))",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
          }
          crate::config::CursorColumnOrder::Asc
          | crate::config::CursorColumnOrder::None => {
            next_filter = format!(
              "{table}::{column}.ge(&{cursor_field}).and({table}::{column}.gt(&{cursor_field}).or({next_filter}))",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
            prev_filter = format!(
              "{table}::{column}.le(&{cursor_field}).and({table}::{column}.lt(&{cursor_field}).or({prev_filter}))",
              table = args.table.name,
              column = col.name,
              cursor_field = cursor_field,
            );
          }
        }
      }
    }

    writeln!(
      w,
      "
      if let Some(cursor) = after {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {next_filter}
        );
      }}
      ",
    )?;
    writeln!(
      w,
      "
      if let Some(cursor) = before {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {prev_filter}
        );
      }}
      ",
    )?;

    writeln!(w, "{EXTEND_NAME}({QUERY_NAME})")?;
    writeln!(w, "}};")?;

    writeln!(
      w,
      "
      let mut {QUERY_NAME} = create_query();

      if let Some(offset) = offset {{
        {QUERY_NAME} = {QUERY_NAME}.offset(offset.try_into().unwrap());
      }}

      if let Some(limit) = limit {{
        {QUERY_NAME} = {QUERY_NAME}.limit(limit.try_into().unwrap());
      }}

      "
    )?;

    writeln!(
      w,
      "{QUERY_NAME}.select({model}::as_select()).load::<{model}>(conn)",
      model = args.model_name
    )?;

    writeln!(w, "}}")?;
    // HAS NEXT

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_next",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(w, "cursor: &'a {cursor_name}, conn: &'a mut Conn")?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(
      w,
      "
      {{
         {cursor_name}::has_next_extend(cursor, |q| q, conn)
      }}
      "
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_next_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "cursor: &'a {cursor_name}, extend: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        into_sql: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;
    writeln!(
      w,
      "
         let {QUERY_NAME} = extend(
           {table}::table
             .filter(
              {next_filter}
             )
             {ordering}
             .limit(1)
      ",
      table = &args.table.name,
    )?;

    if args.soft_delete_column.is_some() {
      writeln!(
        w,
        ".{}",
        soft_delete_filter(&SoftDeleteFilterArgs {
          table_name: &args.table.name,
          soft_delete_column: args.soft_delete_column,
        })?
      )?;
    }

    writeln!(
      w,
      "
          .into_boxed(),
        );
         diesel::select(diesel::dsl::exists({QUERY_NAME})).get_result(conn)
      }}
      ",
    )?;
    // HAS NEXT

    // HAS PREVIOUS
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_previous",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(w, "cursor: &'a {cursor_name}, conn: &'a mut Conn")?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(
      w,
      "
      {{
         {cursor_name}::has_previous_extend(cursor, |q| q, conn)
      }}
      "
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_previous_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "cursor: &'a {cursor_name}, extend: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        into_sql: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;
    writeln!(
      w,
      "
         let {QUERY_NAME} = extend(
           {table}::table
             .filter(
              {prev_filter}
             )
             {ordering}
             .limit(1)
      ",
      table = &args.table.name,
    )?;
    if args.soft_delete_column.is_some() {
      writeln!(
        w,
        ".{}",
        soft_delete_filter(&SoftDeleteFilterArgs {
          table_name: &args.table.name,
          soft_delete_column: args.soft_delete_column,
        })?
      )?;
    }
    writeln!(
      w,
      "
            .into_boxed(),
         );
         diesel::select(diesel::dsl::exists({QUERY_NAME})).get_result(conn)
      }}
      ",
    )?;
    // HAS PREVIOUS

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, "{{
      {cursor_name}::paginate_with_soft_deleted_extend(after, before, limit, offset, |q| q, conn)
     }}"
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, {EXTEND_NAME}: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        into_sql: true,
        selectable_helper: true,
        expression_methods: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "let create_query = || {{")?;

    writeln!(
      w,
      "let mut {QUERY_NAME} = {table}::table{ordering}",
      table = args.table.name,
    )?;

    writeln!(w, ".into_boxed();")?;

    let mut vec = Vec::new();

    for cursor_col in config.columns.iter() {
      let col = args.table.get_column(cursor_col.name()).unwrap();

      let name = get_field_name(
        args.table_config.and_then(|t| t.columns.get(&col.name)),
        cursor_col.name(),
      );

      vec.push((
        format!("{}::{}", &args.table.name, cursor_col.name()),
        format!("{}.{}", "cursor", name),
        col.r#type.to_qualified_string(args.table_imports_root),
      ));
    }

    writeln!(
      w,
      "
      if let Some(cursor) = after {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {next_filter}
        );
      }}
      ",
    )?;
    writeln!(
      w,
      "
      if let Some(cursor) = before {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {prev_filter}
        );
      }}
      ",
    )?;

    writeln!(w, "{EXTEND_NAME}({QUERY_NAME})")?;
    writeln!(w, "}};")?;

    writeln!(
      w,
      "
      let mut {QUERY_NAME} = create_query();

      //let mut has_last = false;

      if let Some(offset) = offset {{
        {QUERY_NAME} = {QUERY_NAME}.offset(offset.try_into().unwrap());
      }}

      if let Some(limit) = limit {{
        {QUERY_NAME} = {QUERY_NAME}.limit(limit.try_into().unwrap());
      }}

      "
    )?;

    writeln!(
      w,
      "{QUERY_NAME}.select({model}::as_select()).load::<{model}>(conn)",
      model = args.model_name
    )?;

    writeln!(w, "}}")?;

    // HAS NEXT

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_next_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(w, "cursor: &'a {cursor_name}, conn: &'a mut Conn")?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(
      w,
      "
      {{
         {cursor_name}::has_next_with_soft_deleted_extend(cursor, |q| q, conn)
      }}
      ",
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_next_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "cursor: &'a {cursor_name}, extend: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        into_sql: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;
    writeln!(
      w,
      "
         let {QUERY_NAME} = extend(
           {table}::table
             .filter(
              {next_filter}
             )
             {ordering}
             .limit(1)
             .into_boxed(),
         );

         diesel::select(diesel::dsl::exists({QUERY_NAME})).get_result(conn)
      }}
      ",
      table = &args.table.name,
    )?;
    // HAS NEXT

    // HAS PREVIOUS
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_previous_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(w, "cursor: &'a {cursor_name}, conn: &'a mut Conn")?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(
      w,
      "
      {{
         {cursor_name}::has_previous_with_soft_deleted_extend(cursor, |q| q, conn)
      }}
      ",
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "has_previous_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "cursor: &'a {cursor_name}, extend: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, false, "bool"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        bool_expression_methods: true,
        into_sql: true,
        ..Default::default()
      },
      &mut w,
    )?;
    writeln!(
      w,
      "
         let {QUERY_NAME} = extend(
           {table}::table
             .filter(
              {prev_filter}
             )
             {ordering}
             .limit(1)
             .into_boxed(),
         );

         diesel::select(diesel::dsl::exists({QUERY_NAME})).get_result(conn)
      }}
      ",
      table = &args.table.name,
    )?;
    // HAS PREVIOUS

    // REVERSED paginate reversed
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_reversed",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(
      w,
      "{{
      {cursor_name}::paginate_reversed_extend(after, before, limit, offset, |q| q, conn)
     }}"
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_reversed_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, {EXTEND_NAME}: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        into_sql: true,
        selectable_helper: true,
        expression_methods: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "let create_query = || {{")?;

    let mut ordering = vec![];
    for (idx, col) in config.columns.iter().enumerate() {
      let name = col.name();
      match col.order() {
        crate::config::CursorColumnOrder::Asc => {
          writeln!(
            ordering,
            ".{order_fn}({table}::{column}.desc())",
            order_fn = if idx != 0 {
              "then_order_by"
            } else {
              "order_by"
            },
            table = args.table.name,
            column = name,
          )?;
        }
        crate::config::CursorColumnOrder::Desc => {
          writeln!(
            ordering,
            ".{order_fn}({table}::{column}.asc())",
            order_fn = if idx != 0 {
              "then_order_by"
            } else {
              "order_by"
            },
            table = args.table.name,
            column = name,
          )?;
        }
        crate::config::CursorColumnOrder::None => {}
      }
    }
    let ordering = String::from_utf8_lossy(&ordering);

    writeln!(
      w,
      "let mut {QUERY_NAME} = {table}::table{ordering}",
      table = args.table.name,
    )?;
    if args.soft_delete_column.is_some() {
      writeln!(
        w,
        ".{}",
        soft_delete_filter(&SoftDeleteFilterArgs {
          table_name: &args.table.name,
          soft_delete_column: args.soft_delete_column,
        })?
      )?;
    }
    writeln!(w, ".into_boxed();")?;

    writeln!(
      w,
      "
      if let Some(cursor) = before {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {next_filter}
        );
      }}
      ",
    )?;
    writeln!(
      w,
      "
      if let Some(cursor) = after {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {prev_filter}
        );
      }}
      ",
    )?;

    writeln!(w, "{EXTEND_NAME}({QUERY_NAME})")?;
    writeln!(w, "}};")?;

    writeln!(
      w,
      "
      let mut {QUERY_NAME} = create_query();

      if let Some(offset) = offset {{
        {QUERY_NAME} = {QUERY_NAME}.offset(offset.try_into().unwrap());
      }}

      if let Some(limit) = limit {{
        {QUERY_NAME} = {QUERY_NAME}.limit(limit.try_into().unwrap());
      }}

      "
    )?;

    writeln!(
      w,
      "{QUERY_NAME}.select({model}::as_select()).load::<{model}>(conn)",
      model = args.model_name
    )?;

    writeln!(w, "}}")?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_reversed_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, "{{
      {cursor_name}::paginate_reversed_with_soft_deleted_extend(after, before, limit, offset, |q| q, conn)
     }}"
    )?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "paginate_reversed_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    write!(
      w,
      "after: Option<&'a {cursor_name}>, before: Option<&'a {cursor_name}>, limit: Option<usize>, offset: Option<usize>, {EXTEND_NAME}: F, conn: &'a mut Conn"
    )?;

    write!(
      w,
      "\n  ) -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(
      w,
      "
      F: for<'b> Fn(
        {table}::BoxedQuery<'b, {backend}>,
      ) -> {table}::BoxedQuery<'b, {backend}>,
      ",
      table = args.table.name,
      backend = args.backend.path(),
    )?;
    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        into_sql: true,
        selectable_helper: true,
        expression_methods: true,
        bool_expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "let create_query = || {{")?;

    writeln!(
      w,
      "let mut {QUERY_NAME} = {table}::table{ordering}",
      table = args.table.name,
    )?;

    writeln!(w, ".into_boxed();")?;

    let mut vec = Vec::new();

    for cursor_col in config.columns.iter() {
      let col = args.table.get_column(cursor_col.name()).unwrap();

      let name = get_field_name(
        args.table_config.and_then(|t| t.columns.get(&col.name)),
        cursor_col.name(),
      );

      vec.push((
        format!("{}::{}", &args.table.name, cursor_col.name()),
        format!("{}.{}", "cursor", name),
        col.r#type.to_qualified_string(args.table_imports_root),
      ));
    }

    writeln!(
      w,
      "
      if let Some(cursor) = before {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {next_filter}
        );
      }}
      ",
    )?;
    writeln!(
      w,
      "
      if let Some(cursor) = after {{
        {QUERY_NAME} = {QUERY_NAME}.filter(
          {prev_filter}
        );
      }}
      ",
    )?;

    writeln!(w, "{EXTEND_NAME}({QUERY_NAME})")?;
    writeln!(w, "}};")?;

    writeln!(
      w,
      "
      let mut {QUERY_NAME} = create_query();

      //let mut has_last = false;

      if let Some(offset) = offset {{
        {QUERY_NAME} = {QUERY_NAME}.offset(offset.try_into().unwrap());
      }}

      if let Some(limit) = limit {{
        {QUERY_NAME} = {QUERY_NAME}.limit(limit.try_into().unwrap());
      }}

      "
    )?;

    writeln!(
      w,
      "{QUERY_NAME}.select({model}::as_select()).load::<{model}>(conn)",
      model = args.model_name
    )?;

    writeln!(w, "}}")?;

    writeln!(w, "}}\n")?;
  }
  Ok(())
}

pub struct CountArgs<'a> {
  use_async: bool,
  model_name: &'a str,
  table: &'a Table,
  backend: &'a SqlBackend,
  soft_delete_column: Option<&'a Column>,
}

fn count<W: Write>(args: &CountArgs, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "count_extend",
      generics: Some(vec!["'a", "F", "Conn"]),
    },
    &mut w,
  )?;
  writeln!(w, "extend: F, conn: &'a mut Conn",)?;
  writeln!(
    w,
    ")  -> {}{}",
    return_type(args.use_async, false, "i64"),
    if args.use_async { " + 'a" } else { "" },
  )?;
  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  writeln!(w, ",")?;
  writeln!(w,
    "
      F: for<'b> Fn({table}::BoxedQuery<'b, {backend}, diesel::sql_types::BigInt>) -> {table}::BoxedQuery<'b, {backend}, diesel::sql_types::BigInt>,
    ",
    table = args.table.name, backend = args.backend.path()
  )?;

  writeln!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      use_async: args.use_async,
      query_dsl: true,
      expression_methods: true,
      ..Default::default()
    },
    &mut w,
  )?;

  writeln!(w, "extend({table}::table.count()", table = args.table.name)?;
  if args.soft_delete_column.is_some() {
    writeln!(
      w,
      ".{}",
      soft_delete_filter(&SoftDeleteFilterArgs {
        table_name: &args.table.name,
        soft_delete_column: args.soft_delete_column,
      })?
    )?;
  }
  writeln!(w, ".into_boxed()).first(conn)",)?;

  writeln!(w, "}}")?;

  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "count",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;

  writeln!(w, "conn: &'a mut Conn",)?;

  writeln!(
    w,
    ")  -> {}{}",
    return_type(args.use_async, false, "i64"),
    if args.use_async { " + 'a" } else { "" },
  )?;

  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;

  writeln!(w, "{{")?;
  writeln!(
    w,
    "{model}::count_extend(|q| q, conn)",
    model = args.model_name
  )?;
  writeln!(w, "}}")?;

  if args.soft_delete_column.is_some() {
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "count_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    writeln!(w, "extend: F, conn: &'a mut Conn",)?;
    writeln!(
      w,
      ")  -> {}{}",
      return_type(args.use_async, false, "i64"),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(w,
    "
      F: for<'b> Fn({table}::BoxedQuery<'b, {backend}, diesel::sql_types::BigInt>) -> {table}::BoxedQuery<'b, {backend}, diesel::sql_types::BigInt>,
    ",
    table = args.table.name, backend = args.backend.path()
  )?;

    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "extend({table}::table.count()", table = args.table.name)?;

    writeln!(w, ".into_boxed()).first(conn)",)?;

    writeln!(w, "}}")?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "count_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;

    writeln!(w, "conn: &'a mut Conn",)?;

    writeln!(
      w,
      ")  -> {}{}",
      return_type(args.use_async, false, "i64"),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;

    writeln!(w, "{{")?;
    writeln!(
      w,
      "{model}::count_with_soft_deleted_extend(|q| q, conn)",
      model = args.model_name
    )?;
    writeln!(w, "}}")?;
  }
  writeln!(w, "}}")?;
  Ok(())
}

pub struct GetArgs<'a> {
  use_async: bool,
  model_name: &'a str,
  table: &'a Table,
  backend: &'a SqlBackend,
  many: bool,
  table_config: Option<&'a TableConfig>,
  primary_keys: &'a Vec<&'a Column>,
  type_overrides: &'a HashMap<String, String>,
  ref_type_overrides: &'a HashMap<String, String>,
  soft_delete_column: Option<&'a Column>,
}

fn get<W: Write>(args: &GetArgs, mut w: W) -> anyhow::Result<()> {
  writeln!(w, "impl {} {{", args.model_name)?;
  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "get_extend",
      generics: Some(vec!["'a", "F", "Conn"]),
    },
    &mut w,
  )?;

  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    args.table_config.map(|t| t.columns.map()),
    Some("'a"),
    &mut w,
  )?;
  writeln!(w, "extend: F, conn: &'a mut Conn",)?;
  writeln!(
    w,
    ")  -> {}{}",
    return_type(args.use_async, false, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;
  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
  writeln!(w, ",")?;
  writeln!(w,
    "
      F: for<'b> Fn({table}::BoxedQuery<'b, {backend}>) -> {table}::BoxedQuery<'b, {backend}>,
    ",
    table = args.table.name, backend = args.backend.path()
  )?;

  writeln!(w, "{{")?;
  default_operation_uses(
    &DefaultUsesArgs {
      use_async: args.use_async,
      query_dsl: true,
      expression_methods: true,
      ..Default::default()
    },
    &mut w,
  )?;

  writeln!(w, "extend({table}::table", table = args.table.name)?;

  if args.soft_delete_column.is_some() {
    writeln!(
      w,
      ".{}",
      soft_delete_filter(&SoftDeleteFilterArgs {
        table_name: &args.table.name,
        soft_delete_column: args.soft_delete_column,
      })?
    )?;
  }
  for i in args.primary_keys {
    let config = args.table_config.and_then(|t| t.columns.get(&i.name));
    let rename = get_field_name(config, &i.name);

    writeln!(
      w,
      ".filter({table}::{column}.eq({rename}))",
      table = args.table.name,
      column = i.name,
    )?;
  }

  writeln!(w, ".into_boxed()).first(conn)",)?;

  writeln!(w, "}}")?;

  function_signature(
    &FunctionSignatureArgs {
      use_async: args.use_async,
      name: "get",
      generics: Some(vec!["'a", "Conn"]),
    },
    &mut w,
  )?;
  write_ref_fn_params(
    args.type_overrides,
    args.ref_type_overrides,
    args.primary_keys,
    args.table_config.map(|t| t.columns.map()),
    Some("'a"),
    &mut w,
  )?;

  writeln!(w, "conn: &'a mut Conn",)?;

  writeln!(
    w,
    ")  -> {}{}",
    return_type(args.use_async, false, args.model_name),
    if args.use_async { " + 'a" } else { "" },
  )?;

  operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;

  writeln!(w, "{{")?;
  writeln!(w, "{model}::get_extend(", model = args.model_name)?;
  for col in args.primary_keys {
    let config = args.table_config.and_then(|t| t.columns.get(&col.name));
    let rename = get_field_name(config, &col.name);

    writeln!(w, "{rename},", rename = rename,)?;
  }
  writeln!(w, "|q| q, conn)",)?;
  writeln!(w, "}}")?;

  if args.many {
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "get_many_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;
    writeln!(w, "extend: F, conn: &'a mut Conn",)?;
    writeln!(
      w,
      ")  -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(w,
    "
      F: for<'b> Fn({table}::BoxedQuery<'b, {backend}>) -> {table}::BoxedQuery<'b, {backend}>,
    ",
    table = args.table.name, backend = args.backend.path()
  )?;

    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "extend({table}::table", table = args.table.name)?;

    if args.soft_delete_column.is_some() {
      writeln!(
        w,
        ".{}",
        soft_delete_filter(&SoftDeleteFilterArgs {
          table_name: &args.table.name,
          soft_delete_column: args.soft_delete_column,
        })?
      )?;
    }

    writeln!(w, ".into_boxed()).load(conn)",)?;

    writeln!(w, "}}")?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "get_many",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;

    writeln!(w, "conn: &'a mut Conn",)?;

    writeln!(
      w,
      ")  -> {}{}",
      return_type(args.use_async, true, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;

    writeln!(w, "{{")?;
    writeln!(
      w,
      "{model}::get_many_extend(|q| q, conn)",
      model = args.model_name
    )?;
    writeln!(w, "}}")?;
  }

  if args.soft_delete_column.is_some() {
    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "get_with_soft_deleted_extend",
        generics: Some(vec!["'a", "F", "Conn"]),
      },
      &mut w,
    )?;

    write_ref_fn_params(
      args.type_overrides,
      args.ref_type_overrides,
      args.primary_keys,
      args.table_config.map(|t| t.columns.map()),
      Some("'a"),
      &mut w,
    )?;
    writeln!(w, "extend: F, conn: &'a mut Conn",)?;
    writeln!(
      w,
      ")  -> {}{}",
      return_type(args.use_async, false, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;
    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
    writeln!(w, ",")?;
    writeln!(w,
    "
      F: for<'b> Fn({table}::BoxedQuery<'b, {backend}>) -> {table}::BoxedQuery<'b, {backend}>,
    ",
    table = args.table.name, backend = args.backend.path()
  )?;

    writeln!(w, "{{")?;
    default_operation_uses(
      &DefaultUsesArgs {
        use_async: args.use_async,
        query_dsl: true,
        expression_methods: true,
        ..Default::default()
      },
      &mut w,
    )?;

    writeln!(w, "extend({table}::table", table = args.table.name)?;

    for i in args.primary_keys {
      let config = args.table_config.and_then(|t| t.columns.get(&i.name));
      let rename = get_field_name(config, &i.name);

      writeln!(
        w,
        ".filter({table}::{column}.eq({rename}))",
        table = args.table.name,
        column = i.name,
      )?;
    }

    writeln!(w, ".into_boxed()).first(conn)",)?;

    writeln!(w, "}}")?;

    function_signature(
      &FunctionSignatureArgs {
        use_async: args.use_async,
        name: "get_with_soft_deleted",
        generics: Some(vec!["'a", "Conn"]),
      },
      &mut w,
    )?;
    write_ref_fn_params(
      args.type_overrides,
      args.ref_type_overrides,
      args.primary_keys,
      args.table_config.map(|t| t.columns.map()),
      Some("'a"),
      &mut w,
    )?;

    writeln!(w, "conn: &'a mut Conn",)?;

    writeln!(
      w,
      ")  -> {}{}",
      return_type(args.use_async, false, args.model_name),
      if args.use_async { " + 'a" } else { "" },
    )?;

    operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;

    writeln!(w, "{{")?;
    writeln!(
      w,
      "{model}::get_with_soft_deleted_extend(",
      model = args.model_name
    )?;
    for col in args.primary_keys {
      let config = args.table_config.and_then(|t| t.columns.get(&col.name));
      let rename = get_field_name(config, &col.name);

      writeln!(w, "{rename},", rename = rename,)?;
    }
    writeln!(w, "|q| q, conn)",)?;
    writeln!(w, "}}")?;

    if args.many {
      function_signature(
        &FunctionSignatureArgs {
          use_async: args.use_async,
          name: "get_many_with_soft_deleted_extend",
          generics: Some(vec!["'a", "F", "Conn"]),
        },
        &mut w,
      )?;
      writeln!(w, "extend: F, conn: &'a mut Conn",)?;
      writeln!(
        w,
        ")  -> {}{}",
        return_type(args.use_async, true, args.model_name),
        if args.use_async { " + 'a" } else { "" },
      )?;
      operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;
      writeln!(w, ",")?;
      writeln!(w,
        "
          F: for<'b> Fn({table}::BoxedQuery<'b, {backend}>) -> {table}::BoxedQuery<'b, {backend}>,
        ",
        table = args.table.name, backend = args.backend.path()
      )?;

      writeln!(w, "{{")?;
      default_operation_uses(
        &DefaultUsesArgs {
          use_async: args.use_async,
          query_dsl: true,
          expression_methods: true,
          ..Default::default()
        },
        &mut w,
      )?;

      writeln!(w, "extend({table}::table", table = args.table.name)?;

      writeln!(w, ".into_boxed()).load(conn)",)?;

      writeln!(w, "}}")?;

      function_signature(
        &FunctionSignatureArgs {
          use_async: args.use_async,
          name: "get_many_with_soft_deleted",
          generics: Some(vec!["'a", "Conn"]),
        },
        &mut w,
      )?;

      writeln!(w, "conn: &'a mut Conn",)?;

      writeln!(
        w,
        ")  -> {}{}",
        return_type(args.use_async, true, args.model_name),
        if args.use_async { " + 'a" } else { "" },
      )?;

      operation_contraints(args.use_async, "Conn", args.backend, &mut w)?;

      writeln!(w, "{{")?;
      writeln!(
        w,
        "{model}::get_many_with_soft_deleted_extend(|q| q, conn)",
        model = args.model_name
      )?;
      writeln!(w, "}}")?;
    }
  }

  writeln!(w, "}}")?;
  Ok(())
}

fn return_type(use_async: bool, multiple: bool, result: &str) -> String {
  if use_async {
    if multiple {
      format!(
      "impl std::future::Future<Output = Result<Vec<{result}>, diesel::result::Error>> + Send",
    )
    } else {
      format!(
      "impl std::future::Future<Output = Result<{result}, diesel::result::Error>> + Send",
    )
    }
  } else if multiple {
    format!(
      "Result<Vec<{model}>, diesel::result::Error>",
      model = result
    )
  } else {
    format!("Result<{model}, diesel::result::Error>", model = result)
  }
}

pub struct SoftDeleteFilterArgs<'a> {
  pub table_name: &'a str,
  pub soft_delete_column: Option<&'a Column>,
}

fn soft_delete_filter(
  args: &SoftDeleteFilterArgs<'_>,
) -> Result<String, anyhow::Error> {
  let mut str = Vec::new();

  if let Some(col) = args.soft_delete_column {
    if col.r#type.is_boolean() {
      writeln!(
        str,
        "filter({table}::{column}.eq(false))",
        table = args.table_name,
        column = col.name
      )?;
    } else if col.r#type.is_integer() {
      writeln!(
        str,
        "filter({table}::{column}.eq(0))",
        table = args.table_name,
        column = col.name
      )?;
    } else if col.r#type.is_nullable_and(|t| t.is_datetime()) {
      writeln!(
        str,
        "filter({table}::{column}.is_null())",
        table = args.table_name,
        column = col.name
      )?;
    } else {
      return Err(anyhow::anyhow!(
          "Unsupported soft delete column type '{}' of column '{}' in table '{}'. Supported class of types are boolean, datetime, integer",
          col.r#type,
          col.name,
          args.table_name
        ));
    }
  }

  Ok(String::from_utf8_lossy(&str).to_string())
}
