use std::{convert::Infallible, str::FromStr};
use thiserror::Error;

pub mod type_names {
  pub const ARRAY: &str = "Array";
  pub const INT2: &str = "Int2";
  pub const SMALLINT: &str = "SmallInt";
  pub const INT4: &str = "Int4";
  pub const INTEGER: &str = "Integer";
  pub const UNSIGNED: &str = "Unsigned";
  pub const INT8: &str = "Int8";
  pub const BIGINT: &str = "BigInt";
  pub const NUMERIC: &str = "Numeric";
  pub const DECIMAL: &str = "Decimal";
  pub const TEXT: &str = "Text";
  pub const DATE: &str = "Date";
  pub const TIME: &str = "Time";
  pub const DATETIME: &str = "Datetime";
  pub const TIMESTAMP: &str = "Timestamp";
  pub const TIMESTAMPTZ: &str = "Timestamptz";
  pub const FLOAT4: &str = "Float4";
  pub const FLOAT8: &str = "Float8";
  pub const FLOAT: &str = "Float";
  pub const BOOL: &str = "Bool";
  pub const JSON: &str = "Json";
  pub const JSONB: &str = "Jsonb";
  pub const UUID: &str = "Uuid";
  pub const CHAR: &str = "Char";
  pub const VARCHAR: &str = "Varchar";
  pub const BYTEA: &str = "Bytea";
  pub const BINARY: &str = "Binary";
  pub const VARBINARY: &str = "Varbinary";
  pub const BLOB: &str = "Blob";
  pub const TINYBLOB: &str = "Tinyblob";
  pub const MEDIUMBLOB: &str = "Mediumblob";
  pub const LONGBLOB: &str = "Longblob";
  pub const BIT: &str = "Bit";
  pub const INET: &str = "Inet";
  pub const TINYTEXT: &str = "Tinytext";
  pub const MEDIUMTEXT: &str = "Mediumtext";
  pub const LONGTEXT: &str = "Longtext";
  pub const DOUBLE: &str = "Double";
  pub const TINYINT: &str = "Tinyint";
  pub const NULLABLE: &str = "Nullable";
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum TypeName {
  Array,
  Int2,
  SmallInt,
  Int4,
  Integer,
  Unsigned,
  Int8,
  BigInt,
  Numeric,
  Decimal,
  Text,
  Date,
  Time,
  Datetime,
  Timestamp,
  Timestamptz,
  Float4,
  Float8,
  Float,
  Bool,
  Json,
  Jsonb,
  Uuid,
  Char,
  Varchar,
  Bytea,
  Binary,
  Varbinary,
  Blob,
  Tinyblob,
  Mediumblob,
  Longblob,
  Bit,
  Inet,
  Tinytext,
  Mediumtext,
  Longtext,
  Double,
  Tinyint,
  Nullable,
  Unknown(String),
}

impl ToString for TypeName {
  fn to_string(&self) -> String {
    use type_names::*;
    match self {
      TypeName::Array => ARRAY,
      TypeName::Int2 => INT2,
      TypeName::SmallInt => SMALLINT,
      TypeName::Int4 => INT4,
      TypeName::Integer => INTEGER,
      TypeName::Unsigned => UNSIGNED,
      TypeName::Int8 => INT8,
      TypeName::BigInt => BIGINT,
      TypeName::Numeric => NUMERIC,
      TypeName::Decimal => DECIMAL,
      TypeName::Text => TEXT,
      TypeName::Date => DATE,
      TypeName::Time => TIME,
      TypeName::Datetime => DATETIME,
      TypeName::Timestamp => TIMESTAMP,
      TypeName::Timestamptz => TIMESTAMPTZ,
      TypeName::Float4 => FLOAT4,
      TypeName::Float8 => FLOAT8,
      TypeName::Float => FLOAT,
      TypeName::Bool => BOOL,
      TypeName::Json => JSON,
      TypeName::Jsonb => JSONB,
      TypeName::Uuid => UUID,
      TypeName::Char => CHAR,
      TypeName::Varchar => VARCHAR,
      TypeName::Bytea => BYTEA,
      TypeName::Binary => BINARY,
      TypeName::Varbinary => VARBINARY,
      TypeName::Blob => BLOB,
      TypeName::Tinyblob => TINYBLOB,
      TypeName::Mediumblob => MEDIUMBLOB,
      TypeName::Longblob => LONGBLOB,
      TypeName::Bit => BIT,
      TypeName::Inet => INET,
      TypeName::Tinytext => TINYTEXT,
      TypeName::Mediumtext => MEDIUMTEXT,
      TypeName::Longtext => LONGTEXT,
      TypeName::Double => DOUBLE,
      TypeName::Tinyint => TINYINT,
      TypeName::Nullable => NULLABLE,
      TypeName::Unknown(name) => name,
    }
    .to_string()
  }
}

impl FromStr for TypeName {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      type_names::ARRAY => Ok(TypeName::Array),
      type_names::INT2 => Ok(TypeName::Int2),
      type_names::SMALLINT => Ok(TypeName::SmallInt),
      type_names::INT4 => Ok(TypeName::Int4),
      type_names::INTEGER => Ok(TypeName::Integer),
      type_names::UNSIGNED => Ok(TypeName::Unsigned),
      type_names::INT8 => Ok(TypeName::Int8),
      type_names::BIGINT => Ok(TypeName::BigInt),
      type_names::NUMERIC => Ok(TypeName::Numeric),
      type_names::DECIMAL => Ok(TypeName::Decimal),
      type_names::TEXT => Ok(TypeName::Text),
      type_names::DATE => Ok(TypeName::Date),
      type_names::TIME => Ok(TypeName::Time),
      type_names::DATETIME => Ok(TypeName::Datetime),
      type_names::TIMESTAMP => Ok(TypeName::Timestamp),
      type_names::TIMESTAMPTZ => Ok(TypeName::Timestamptz),
      type_names::FLOAT4 => Ok(TypeName::Float4),
      type_names::FLOAT8 => Ok(TypeName::Float8),
      type_names::FLOAT => Ok(TypeName::Float),
      type_names::BOOL => Ok(TypeName::Bool),
      type_names::JSON => Ok(TypeName::Json),
      type_names::JSONB => Ok(TypeName::Jsonb),
      type_names::UUID => Ok(TypeName::Uuid),
      type_names::CHAR => Ok(TypeName::Char),
      type_names::VARCHAR => Ok(TypeName::Varchar),
      type_names::BYTEA => Ok(TypeName::Bytea),
      type_names::BINARY => Ok(TypeName::Binary),
      type_names::VARBINARY => Ok(TypeName::Varbinary),
      type_names::BLOB => Ok(TypeName::Blob),
      type_names::TINYBLOB => Ok(TypeName::Tinyblob),
      type_names::MEDIUMBLOB => Ok(TypeName::Mediumblob),
      type_names::LONGBLOB => Ok(TypeName::Longblob),
      type_names::BIT => Ok(TypeName::Bit),
      type_names::INET => Ok(TypeName::Inet),
      type_names::TINYTEXT => Ok(TypeName::Tinytext),
      type_names::MEDIUMTEXT => Ok(TypeName::Mediumtext),
      type_names::LONGTEXT => Ok(TypeName::Longtext),
      type_names::DOUBLE => Ok(TypeName::Double),
      type_names::TINYINT => Ok(TypeName::Tinyint),
      type_names::NULLABLE => Ok(TypeName::Nullable),
      s => Ok(TypeName::Unknown(s.to_string())),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ParseContext<'a> {
  str: &'a str,
  current_index: usize,
  until: usize,
}

impl<'a> ParseContext<'a> {
  pub fn new(string: &'a str) -> Self {
    Self {
      str: string,
      current_index: 0,
      until: string.len(),
    }
  }

  pub fn current_index(&self) -> usize {
    self.current_index
  }

  pub fn until(&self) -> usize {
    self.until
  }

  pub fn current_line(&self) -> usize {
    self.str[..self.current_index].lines().count()
  }

  pub fn slice(&self) -> &'a str {
    &self.str[self.current_index..self.until]
  }

  pub fn current_column(&self) -> usize {
    self.str[..self.current_index]
      .lines()
      .last()
      .unwrap_or("")
      .len()
  }

  pub fn at_end(&self) -> bool {
    self.current_index >= self.until
  }

  pub fn skip_whitespace(&mut self) -> &mut Self {
    let first = self.slice().find(|c: char| !c.is_whitespace()).unwrap_or(0);

    let last = self
      .slice()
      .rfind(|c: char| !c.is_whitespace())
      .unwrap_or(self.str.len() - 1);

    self.current_index += first;

    if last >= first {
      self.until -= self.str.len() - (last + 1);
    }
    self
  }

  pub fn current_char(&self) -> Option<char> {
    self.str[self.current_index..].chars().next()
  }

  pub fn last_char(&self) -> Option<char> {
    self.str[self.current_index..self.until].chars().last()
  }

  pub fn look_ahead(&self) -> Option<char> {
    self.str[self.current_index + 1..].chars().next()
  }

  pub fn next(&mut self) -> Option<char> {
    let next = self.current_char();
    self.current_index += 1;
    next
  }

  pub fn current_line_and_column(&self) -> (usize, usize) {
    (self.current_line(), self.current_column())
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Type {
  pub name: TypeName,
  pub params: Vec<Type>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Attribute {
  pub content: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Column {
  pub name: String,
  pub attributes: Vec<Attribute>,
  pub r#type: Type,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Table {
  pub name: String,
  pub schema: Option<String>,
  pub columns: Vec<Column>,
}

#[derive(Error, Debug, Eq, PartialEq, Clone)]
#[error("ParseError: {message} at line {line} column {column}")]
pub struct ParseError {
  pub message: String,
  pub line: usize,
  pub column: usize,
}

impl ParseError {
  pub fn new(message: String, line: usize, column: usize) -> Self {
    Self {
      message,
      line,
      column,
    }
  }
}

pub fn parse_type(ctx: &mut ParseContext) -> Result<Type, ParseError> {
  let mut type_name = String::new();
  let mut params = Vec::new();
  let mut param_type_name = String::new();
  let mut depth = 0;
  let mut started_identifier = false;

  while let Some(c) = ctx.next() {
    match c {
      '<' => {
        depth += 1;
        if depth > 1 {
          param_type_name.push(c);
        }
        started_identifier = false;
      }
      '>' => {
        depth -= 1;
        if depth > 0 {
          param_type_name.push(c);
        } else {
          params.push(parse_type(&mut ParseContext::new(&param_type_name))?);
          param_type_name.clear();
        }
        started_identifier = false;
      }
      ',' => {
        params.push(parse_type(&mut ParseContext::new(&param_type_name))?);
        param_type_name.clear();
        started_identifier = false;
      }
      'A'..='Z' => {
        if depth > 0 {
          param_type_name.push(c);
        } else {
          type_name.push(c);
        }
        started_identifier = true;
      }
      'a'..='z' => {
        if depth > 0 {
          param_type_name.push(c);
        } else {
          type_name.push(c);
        }
        started_identifier = true;
      }
      '_' => {
        if depth > 0 {
          param_type_name.push(c);
        } else {
          type_name.push(c);
        }
        started_identifier = true;
      }
      '0'..='9' if started_identifier => {
        if depth > 0 {
          param_type_name.push(c);
        } else {
          type_name.push(c);
        }
      }
      ' ' | '\t' | '\n' => {}
      _ => {
        return Err(ParseError::new(
          format!("Unexpected character: {}", c),
          ctx.current_line(),
          ctx.current_column(),
        ))
      }
    }
  }

  Ok(Type {
    name: TypeName::from_str(&type_name).map_err(|_| {
      ParseError::new(
        "Could not parse type name".to_string(),
        ctx.current_line(),
        ctx.current_column(),
      )
    })?,
    params,
  })
}

pub fn parse_attribute(
  ctx: &mut ParseContext,
) -> Result<Attribute, ParseError> {
  let mut content = String::new();
  let mut encountered_hash = false;
  let mut encountered_opening_bracket = false;

  while let Some(c) = ctx.next() {
    match c {
      ' ' | '\t' | '\n' => {}
      '#'
        if !encountered_hash
          && !encountered_opening_bracket
          && content.is_empty() =>
      {
        encountered_hash = true;
      }
      '[' if encountered_hash => {
        encountered_opening_bracket = true;
      }
      ']' if encountered_opening_bracket => {
        break;
      }
      _ if encountered_opening_bracket => {
        content.push(c);
      }
      _ => {
        return Err(ParseError::new(
          format!("Unexpected character: {}", c),
          ctx.current_line(),
          ctx.current_column(),
        ))
      }
    }
  }

  Ok(Attribute { content })
}

pub fn parse_column(ctx: &mut ParseContext) -> Result<Column, ParseError> {
  if ctx.current_char().unwrap().is_whitespace() {
    return Err(ParseError::new(
      format!("Unexpected whitespace: {}, trim the context before calling this function", ctx.current_char().unwrap()),
      ctx.current_line(),
      ctx.current_column(),
    ));
  }

  if ctx.last_char().unwrap().is_whitespace() {
    return Err(ParseError::new(
      format!("Unexpected whitespace at the end: {}, trim the context before calling this function", ctx.last_char().unwrap()),
      ctx.current_line(),
      ctx.current_column(),
    ));
  }

  let mut attributes = Vec::new();

  let mut attribute = String::new();
  let mut name = String::new();
  let mut ty = String::new();

  let mut in_attribute = false;
  let mut in_name = !ctx.slice().starts_with('#');
  let mut in_arrow = false;
  let mut in_type = false;

  let mut parsed_attributes = !ctx.slice().starts_with('#');
  let mut parsed_name = false;
  let mut parsed_arrow = false;
  let mut parsed_type = false;

  while let Some(c) = ctx.next() {
    println!("c: {}", c);
    println!(
      "in_attribute: {}, in_name: {}, in_arrow: {}, in_type: {}",
      in_attribute, in_name, in_arrow, in_type
    );
    println!("parsed_attributes: {}, parsed_name: {}, parsed_arrow: {}, parsed_type: {}", parsed_attributes, parsed_name, parsed_arrow, parsed_type);

    println!("attribute: {}, name: {}, ty: {}", attribute, name, ty,);

    match c {
      '-'
        if parsed_attributes
          && parsed_name
          && !parsed_arrow
          && !parsed_type
          && !in_arrow =>
      {
        in_arrow = true;
      }
      '>' if in_arrow => {
        in_arrow = false;
        parsed_arrow = true;
      }
      '#'
        if !parsed_attributes
          && !parsed_name
          && !parsed_arrow
          && !parsed_type
          && !in_attribute =>
      {
        in_attribute = true;
        attribute.push(c);
      }
      '[' if in_attribute => {
        attribute.push(c);
      }
      ']' if in_attribute => {
        attributes.push(parse_attribute(&mut ParseContext::new(&attribute))?);
        attribute.clear();
        in_attribute = false;
      }
      c if in_attribute => {
        attribute.push(c);
      }
      c if in_arrow && !c.is_whitespace() => {
        ty.push(c);
      }
      c if in_type => {
        ty.push(c);
      }
      c if !parsed_attributes
        && !parsed_name
        && !parsed_arrow
        && !parsed_type
        && matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') =>
      {
        parsed_attributes = true;
        name.push(c);
        in_name = true;
      }
      c if in_name && matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') => {
        if c.is_ascii_digit() && !name.is_empty() {
          return Err(ParseError::new(
            format!("Unexpected character: {}, identifiers must start with a letter or underscore", c),
            ctx.current_line(),
            ctx.current_column(),
          ));
        }

        name.push(c);

        if ctx.current_char().unwrap().is_whitespace() {
          in_name = false;
          parsed_name = true;
        }
      }
      c if parsed_attributes
        && parsed_name
        && parsed_arrow
        && !parsed_type
        && !in_type
        && !c.is_whitespace() =>
      {
        ty.push(c);
        in_type = true;
      }
      c if in_type => {
        ty.push(c);

        if ctx.at_end() {
          parsed_type = true;
          in_type = false;
        }
      }
      ' ' | '\t' | '\n' => {
        if in_arrow {
          return Err(ParseError::new(
            format!("Unexpected whitespace: {}", c),
            ctx.current_line(),
            ctx.current_column(),
          ));
        }
      }
      _ => {
        return Err(ParseError::new(
          format!("Unexpected character: {}", c),
          ctx.current_line(),
          ctx.current_column(),
        ))
      }
    }
  }

  Ok(Column {
    name,
    attributes,
    r#type: parse_type(&mut ParseContext::new(&ty))?,
  })
}

#[cfg(test)]
mod test {

  use std::vec;

  use super::*;

  #[test]
  fn skip_whitespace() {
    let mut ctx = ParseContext::new("  test     ");

    assert_eq!(
      *ctx.skip_whitespace(),
      ParseContext {
        str: "  test     ",
        current_index: 2,
        until: 6,
      }
    );

    let mut ctx = ParseContext::new("       test     ");

    assert_eq!(
      *ctx.skip_whitespace(),
      ParseContext {
        str: "       test     ",
        current_index: 7,
        until: 11,
      }
    );
  }

  #[test]
  fn look_ahead() {
    let ctx = ParseContext::new("ab");

    assert_eq!(ctx.look_ahead(), Some('b'));
  }

  #[test]
  fn sql_type_with_type_params() {
    assert_eq!(
      parse_type(&mut ParseContext::new("Array<Int4>")),
      Ok(Type {
        name: TypeName::Array,
        params: vec![Type {
          name: TypeName::Int4,
          params: vec![],
        }],
      })
    )
  }

  #[test]
  fn sql_type_with_type_params_nested() {
    assert_eq!(
      parse_type(&mut ParseContext::new("Array<Array<Int4>>")),
      Ok(Type {
        name: TypeName::Array,
        params: vec![Type {
          name: TypeName::Array,
          params: vec![Type {
            name: TypeName::Int4,
            params: vec![],
          },],
        }],
      })
    )
  }

  #[test]
  fn sql_type_with_multiple_type_params() {
    assert_eq!(
      parse_type(&mut ParseContext::new("Array<Int4, Int4, Nullable<Int4>>")),
      Ok(Type {
        name: TypeName::Array,
        params: vec![
          Type {
            name: TypeName::Int4,
            params: vec![],
          },
          Type {
            name: TypeName::Int4,
            params: vec![],
          },
          Type {
            name: TypeName::Nullable,
            params: vec![Type {
              name: TypeName::Int4,
              params: vec![],
            }],
          },
        ],
      })
    )
  }

  #[test]
  fn sql_type_simple() {
    assert_eq!(
      parse_type(&mut ParseContext::new("Int4")),
      Ok(Type {
        name: TypeName::Int4,
        params: vec![],
      })
    )
  }

  #[test]
  fn attribute_simple() {
    assert_eq!(
      parse_attribute(&mut ParseContext::new("#[sql_type = \"Int4\"]")),
      Ok(Attribute {
        content: "sql_type=\"Int4\"".to_string(),
      })
    )
  }

  #[test]
  fn column_simple() {
    assert_eq!(
      parse_column(&mut ParseContext::new("id -> Uuid")),
      Ok(Column {
        name: "id".to_string(),
        attributes: vec![],
        r#type: Type {
          name: TypeName::Uuid,
          params: vec![]
        }
      })
    )
  }

  #[test]
  fn column_attribute() {
    assert_eq!(
      parse_column(&mut ParseContext::new(
        "#[max_length = 255] name -> Varchar"
      )),
      Ok(Column {
        name: "name".to_string(),
        attributes: vec![Attribute {
          content: "max_length=255".to_string()
        },],
        r#type: Type {
          name: TypeName::Varchar,
          params: vec![]
        }
      })
    )
  }

  #[test]
  fn column_multiple_attribute() {
    assert_eq!(
      parse_column(&mut ParseContext::new(
        "#[max_length = 255] #[min_length = 100] name -> Varchar"
      )),
      Ok(Column {
        name: "name".to_string(),
        attributes: vec![
          Attribute {
            content: "max_length=255".to_string()
          },
          Attribute {
            content: "min_length=100".to_string()
          },
        ],
        r#type: Type {
          name: TypeName::Varchar,
          params: vec![]
        }
      })
    )
  }

  #[test]
  fn column_multiple_attribute_and_type_params() {
    assert_eq!(
      parse_column(&mut ParseContext::new(
        "#[max_length = 255]\n #[min_length = 100]\n name -> Varchar<Int4, Nullable<Int4>>"
      )),
      Ok(Column {
        name: "name".to_string(),
        attributes: vec![
          Attribute {
            content: "max_length=255".to_string()
          },
          Attribute {
            content: "min_length=100".to_string()
          },
        ],
        r#type: Type {
          name: TypeName::Varchar,
          params: vec![
            Type {
              name: TypeName::Int4,
              params: vec![],
            },
            Type {
              name: TypeName::Nullable,
              params: vec![Type {
                name: TypeName::Int4,
                params: vec![],
              }],
            }
          ]
        }
      })
    )
  }
}
