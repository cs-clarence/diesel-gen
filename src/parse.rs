#![allow(dead_code)]
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

impl<'a> From<&'a str> for ParseContext<'a> {
  fn from(str: &'a str) -> Self {
    Self::new(str)
  }
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

  pub fn next_index(&self) -> usize {
    self.current_index + 1
  }

  pub fn previous_index(&self) -> usize {
    self.current_index - 1
  }

  pub fn until(&self) -> usize {
    self.until
  }

  pub fn current_line(&self) -> usize {
    self.str[..self.current_index].lines().count()
  }

  pub fn str(&self) -> &'a str {
    &self.str[self.current_index..self.until]
  }

  pub fn full_str(&self) -> &'a str {
    self.str
  }

  pub fn slice_outside(&self, start: usize, until: usize) -> ParseContext<'a> {
    Self {
      str: self.str,
      current_index: start,
      until,
    }
  }

  pub fn slice(&self, start: usize, until: usize) -> ParseContext<'a> {
    Self {
      str: self.str,
      current_index: self.current_index + start,
      until: self.until - (self.until - until),
    }
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

  pub fn skip_whitespaces(&mut self) -> &mut Self {
    self.skip_whitespaces_start().skip_whitespaces_end()
  }

  pub fn current_char(&self) -> Option<char> {
    self.str().chars().next()
  }

  pub fn last_char(&self) -> Option<char> {
    self.str().chars().last()
  }

  pub fn next_char(&self) -> Option<char> {
    self.str[self.current_index + 1..self.until].chars().next()
  }

  pub fn previous_char(&self) -> Option<char> {
    self.str[..self.current_index].chars().last()
  }

  pub fn next(&mut self) -> Option<char> {
    if self.current_index >= self.until {
      return None;
    }

    self.current_index += 1;

    self.str().chars().next()
  }

  pub fn current_line_and_column(&self) -> (usize, usize) {
    (self.current_line(), self.current_column())
  }

  pub fn ignore_str(&mut self, str: &str) -> Result<&mut Self, ParseError> {
    for c in str.chars() {
      let Some(cx) = self.current_char() else {
        return Err(ParseError::from_parse_context(
          "No more characters".to_string(),
          self,
        ));
      };

      if c != cx {
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}, expected {}", cx, c),
          self,
        ));
      }

      self.next();
    }

    Ok(self)
  }

  pub fn ignore_char(&mut self, c: char) -> Result<&mut Self, ParseError> {
    let Some(cx) = self.current_char() else {
      return Err(ParseError::from_parse_context(
        "No more characters".to_string(),
        self,
      ));
    };

    if c != cx {
      return Err(ParseError::from_parse_context(
        format!("Unexpected character: {}, expected {}", cx, c),
        self,
      ));
    }

    self.next();

    Ok(self)
  }

  pub fn ignore_until_char(
    &mut self,
    c: char,
  ) -> Result<&mut Self, ParseError> {
    while let Some(cx) = self.next() {
      if cx == c {
        break;
      }
    }
    Ok(self)
  }

  pub fn contains_char(&self, c: char) -> bool {
    self.str().contains(c)
  }

  pub fn contains_str(&self, str: &str) -> bool {
    self.str().contains(str)
  }

  pub fn move_to_str(&mut self, str: &str) -> Result<&mut Self, ParseError> {
    let idx = self.str().find(str);

    if let Some(idx) = idx {
      self.current_index += idx;
      Ok(self)
    } else {
      Err(ParseError::from_parse_context(
        format!("Could not find string: {}", str),
        self,
      ))
    }
  }

  pub fn skip_whitespaces_start(&mut self) -> &mut Self {
    while let Some(c) = self.current_char() {
      if !c.is_whitespace() {
        break;
      }
      self.next();
    }
    self
  }

  pub fn skip_whitespaces_end(&mut self) -> &mut Self {
    while let Some(c) = self.last_char() {
      if !c.is_whitespace() {
        break;
      }
      self.until -= 1;
    }
    self
  }

  pub fn expect_char(&mut self, c: char) -> Result<&mut Self, ParseError> {
    let cx = self.current_char().unwrap();
    if cx != c {
      return Err(ParseError::from_parse_context(
        format!("Unexpected character: {}, expected {}", cx, c),
        self,
      ));
    }

    Ok(self)
  }

  pub fn expect_str(&mut self, str: &str) -> Result<(), ParseError> {
    let mut chars = self.str().chars();

    for x in str.chars() {
      if let Some(y) = chars.next() {
        if x != y {
          return Err(ParseError::from_parse_context(
            format!(
              "Unexpected character: {}, expected {} from str: {}",
              y, x, str
            ),
            self,
          ));
        }
      } else {
        return Err(ParseError::from_parse_context(
          format!("Unexpected end of string, expected {}", x),
          self,
        ));
      }
    }

    Ok(())
  }

  pub fn extract_around_pair(
    &mut self,
    opening: char,
    closing: char,
  ) -> Result<Self, ParseError> {
    let mut depth = 0;
    let mut started = false;
    let mut start_index = self.current_index;
    let mut until = self.until;

    while let Some(c) = self.current_char() {
      if c == opening {
        depth += 1;

        if !started {
          start_index = self.current_index;
          started = true;
        }
      } else if started {
        if c == closing {
          depth -= 1;
          if depth == 0 {
            until = self.current_index + 1;
            break;
          }
        }
      } else {
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          self,
        ));
      }
      self.next();
    }
    Ok(Self {
      str: self.str,
      current_index: start_index,
      until,
    })
  }

  pub fn extract_within_pair(
    &mut self,
    opening: char,
    closing: char,
  ) -> Result<Self, ParseError> {
    let mut x = self.extract_around_pair(opening, closing)?;

    x.current_index += 1;
    x.until -= 1;

    Ok(x)
  }

  pub fn extract_until_whitespace(&mut self) -> Result<Self, ParseError> {
    let mut started = false;
    let mut current_index = self.current_index;
    let mut until = self.until;

    let has_whitespace = self.str().chars().any(|c| c.is_whitespace());

    if !has_whitespace {
      return Err(ParseError::from_parse_context(
        "No whitespace found".to_string(),
        self,
      ));
    }

    while let Some(c) = self.current_char() {
      if c.is_whitespace() {
        until = self.current_index;
        break;
      } else if !started {
        current_index = self.current_index - 1;
        started = true;
      }

      self.next();
    }

    Ok(Self {
      str: self.str,
      current_index,
      until,
    })
  }

  pub fn extract_until_str(&mut self, str: &str) -> Result<Self, ParseError> {
    if let Some(idx) = self.str().find(str) {
      let current_index = self.current_index;
      let until = self.current_index + idx;
      self.current_index += idx;

      Ok(Self {
        str: self.str,
        current_index,
        until,
      })
    } else {
      Err(ParseError::from_parse_context(
        format!("Could not find string: {}", str),
        self,
      ))
    }
  }

  pub fn extract_identifier(&mut self) -> Result<Self, ParseError> {
    let mut started = false;
    let mut current_index = self.current_index;
    let mut until = self.until;

    if !self.current_char().unwrap().is_alphabetic()
      || self.current_char().unwrap() != '_'
    {
      return Err(ParseError::from_parse_context(
        "Identifiers must start with a letter or underscore".to_string(),
        self,
      ));
    }

    while let Some(c) = self.current_char() {
      if !(c.is_alphanumeric() || c != '_') {
        break;
      }

      if !c.is_ascii_alphanumeric() && c != '_' {
        if !started {
          return Err(ParseError::from_parse_context(
            format!("Unexpected character: {}", c),
            self,
          ));
        }
        until = self.current_index;
        break;
      } else if !started {
        current_index = self.current_index;
        started = true;
      }

      self.next();
    }

    Ok(Self {
      str: self.str,
      current_index,
      until,
    })
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
  pub primary_key: Vec<String>,
  pub columns: Vec<Column>,
}

#[derive(Error, Debug, Eq, PartialEq, Clone)]
#[error("ParseError: {message} at line {line} column {column}")]
pub struct ParseError {
  pub message: String,
  pub string: String,
  pub line: usize,
  pub column: usize,
}

impl ParseError {
  pub fn new(
    message: String,
    string: String,
    line: usize,
    column: usize,
  ) -> Self {
    Self {
      message,
      string,
      line,
      column,
    }
  }

  pub fn from_parse_context(message: String, ctx: &ParseContext<'_>) -> Self {
    Self {
      message,
      string: ctx.full_str().to_string(),
      line: ctx.current_line(),
      column: ctx.current_column(),
    }
  }
}

pub fn parse_type(ctx: &mut ParseContext<'_>) -> Result<Type, ParseError> {
  ctx.skip_whitespaces();

  let mut type_name = String::new();
  let mut params = Vec::new();
  let mut param_type_name = String::new();
  let mut depth = 0;
  let mut started_identifier = false;

  while let Some(c) = ctx.current_char() {
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
      ',' if depth > 0 => {
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
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          ctx,
        ))
      }
    }

    ctx.next();
  }

  Ok(Type {
    name: TypeName::from_str(&type_name).map_err(|_| {
      ParseError::from_parse_context(
        "Could not parse type name".to_string(),
        ctx,
      )
    })?,
    params,
  })
}

pub fn parse_attribute(
  ctx: &mut ParseContext<'_>,
) -> Result<Attribute, ParseError> {
  ctx.skip_whitespaces();

  let mut content = String::new();
  let mut encountered_hash = false;
  let mut encountered_opening_bracket = false;

  while let Some(c) = ctx.current_char() {
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
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          ctx,
        ))
      }
    }
    ctx.next();
  }

  Ok(Attribute { content })
}

pub fn parse_column(ctx: &mut ParseContext<'_>) -> Result<Column, ParseError> {
  ctx.skip_whitespaces();

  if ctx.current_char().unwrap().is_whitespace() {
    return Err(ParseError::from_parse_context(
      format!("Unexpected whitespace: {}, trim the context before calling this function", ctx.current_char().unwrap()),
      ctx
    ));
  }

  if ctx.last_char().unwrap().is_whitespace() {
    return Err(ParseError::from_parse_context(
      format!("Unexpected whitespace at the end: {}, trim the context before calling this function", ctx.last_char().unwrap()),
      ctx
    ));
  }

  let mut attributes = Vec::new();

  let mut attribute = String::new();
  let mut name = String::new();
  let mut ty = String::new();

  let mut in_attribute = false;
  let mut in_name = !ctx.str().starts_with('#');
  let mut in_arrow = false;
  let mut in_type = false;

  let mut parsed_attributes = !ctx.str().starts_with('#');
  let mut parsed_name = false;
  let mut parsed_arrow = false;
  let mut parsed_type = false;
  let mut parsed_comma = false;

  let mut type_depth = 0;

  while let Some(c) = ctx.current_char() {
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
      c @ '<' if in_type => {
        type_depth += 1;
        ty.push(c);
      }
      c @ '>' if in_type => {
        type_depth -= 1;
        ty.push(c);

        if type_depth == 0 {
          parsed_type = true;
          in_type = false;
        }
      }
      ','
        if parsed_attributes
          && parsed_name
          && parsed_arrow
          && !parsed_comma
          && in_type
          && type_depth == 0 =>
      {
        parsed_type = true;
        in_type = false;
        parsed_comma = true;
      }
      c if in_type => {
        ty.push(c);
      }
      c if in_attribute => {
        attribute.push(c);
      }
      c if in_arrow && !c.is_whitespace() => {
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
          return Err(ParseError::from_parse_context(
            format!("Unexpected character: {}, identifiers must start with a letter or underscore", c),
            ctx
          ));
        }

        name.push(c);

        if ctx.next_char().unwrap().is_whitespace()
          || ctx.next_char().unwrap() == '-'
        {
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
          return Err(ParseError::from_parse_context(
            format!("Unexpected whitespace: {}", c),
            ctx,
          ));
        }
      }
      _ => {
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          ctx,
        ))
      }
    }

    ctx.next();
  }

  Ok(Column {
    name,
    attributes,
    r#type: parse_type(&mut ParseContext::new(&ty))?,
  })
}

pub fn parse_table_primary_key(
  ctx: &mut ParseContext<'_>,
) -> Result<Vec<String>, ParseError> {
  ctx.skip_whitespaces();

  if ctx.current_char().unwrap() != '(' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, primary key must start with (",
        ctx.current_char().unwrap()
      ),
      ctx,
    ));
  }

  if ctx.last_char().unwrap() != ')' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, primary key must end with )",
        ctx.last_char().unwrap()
      ),
      ctx,
    ));
  }

  let mut identifiers = Vec::new();
  let mut identifier = String::new();
  let mut in_identifier = false;

  while let Some(c) = ctx.current_char() {
    match c {
      '(' => {}
      c if !in_identifier
        && matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') =>
      {
        if c.is_ascii_digit() {
          return Err(ParseError::from_parse_context(
            format!("Unexpected character: {}, identifiers must start with a letter or underscore", c),
            ctx
          ));
        }

        identifier.push(c);

        in_identifier = true;
      }
      c if in_identifier
        && matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') =>
      {
        identifier.push(c);
        if ctx.next_char().unwrap().is_whitespace() {
          in_identifier = false;
          identifiers.push(identifier.clone());
          identifier.clear();
        }
      }
      ',' | ')' if in_identifier => {
        identifiers.push(identifier.clone());
        identifier.clear();
      }
      ' ' | '\t' | '\n' => {}
      _ => {
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          ctx,
        ))
      }
    }

    ctx.next();
  }

  Ok(identifiers)
}

pub fn parse_table_body(
  ctx: &mut ParseContext<'_>,
) -> Result<Vec<Column>, ParseError> {
  ctx.skip_whitespaces();
  if ctx.current_char().unwrap() != '{' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, table body must start with {{",
        ctx.current_char().unwrap()
      ),
      ctx,
    ));
  }

  if ctx.last_char().unwrap() != '}' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, table body must end with }}",
        ctx.last_char().unwrap()
      ),
      ctx,
    ));
  }

  let mut columns = Vec::new();
  let mut in_column = false;
  let mut column_start = 0;
  let mut column_until = 0;

  while let Some(c) = ctx.current_char() {
    match c {
      '{' => {}
      '}' | ',' if in_column => {
        columns.push(parse_column(
          &mut ctx.slice_outside(column_start, column_until),
        )?);
        in_column = false;
        column_start = 0;
        column_until = 0;
      }
      _ if in_column => {
        column_until += 1;
      }
      c if !c.is_whitespace() => {
        in_column = true;
        column_start = ctx.current_index();
        column_until = ctx.current_index() + 1;
      }
      ' ' | '\t' | '\n' => {}
      _ => {
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          ctx,
        ));
      }
    }
    ctx.next();
  }

  Ok(columns)
}

pub fn parse_table(ctx: &mut ParseContext<'_>) -> Result<Table, ParseError> {
  ctx.skip_whitespaces();

  if !ctx.current_char().unwrap().is_alphabetic() {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, table name must start with a letter",
        ctx.current_char().unwrap()
      ),
      ctx,
    ));
  }

  if ctx.last_char().unwrap() != '}' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, table name must end with }}",
        ctx.last_char().unwrap()
      ),
      ctx,
    ));
  }

  let mut in_name = false;
  let mut parsed_name = false;
  let mut name = String::new();

  let mut in_body = false;
  let mut parsed_body = false;
  let mut body = String::new();

  let mut in_primary_key = false;
  let mut parsed_primary_key = false;
  let mut primary_key = String::new();

  while let Some(c) = ctx.current_char() {
    match c {
      c if !parsed_name
        && !parsed_body
        && !parsed_primary_key
        && !in_name
        && matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '.') =>
      {
        if c.is_ascii_digit() && !name.is_empty() {
          return Err(ParseError::from_parse_context(
                    format!("Unexpected character: {}, identifiers must start with a letter or underscore", c),
                    ctx
                  ));
        }

        name.push(c);

        in_name = true;
      }
      c if in_name
        && matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '.') =>
      {
        if name.contains('.') && c == '.' {
          return Err(ParseError::from_parse_context(
            "Name can only contain one dot to separate schema and name"
              .to_string(),
            ctx,
          ));
        }

        name.push(c);

        if ctx.next_char().unwrap().is_whitespace()
          || ctx.next_char().unwrap() == '('
        {
          in_name = false;
          parsed_name = true;
        }
      }
      '('
        if parsed_name
          && !parsed_primary_key
          && !parsed_body
          && !in_primary_key =>
      {
        primary_key.push(c);

        in_primary_key = true;
      }
      ')' if in_primary_key => {
        primary_key.push(c);
        in_primary_key = false;
        parsed_primary_key = true;
      }
      c if in_primary_key => {
        primary_key.push(c);
      }
      '{' if parsed_name && parsed_primary_key && !parsed_body && !in_body => {
        body.push(c);

        in_body = true;
      }
      '}' if in_body => {
        body.push(c);
        in_body = false;
        parsed_body = true;
      }
      c if in_body => {
        body.push(c);
      }
      ' ' | '\t' | '\n' => {}
      _ => {
        return Err(ParseError::from_parse_context(
          format!("Unexpected character: {}", c),
          ctx,
        ));
      }
    }

    ctx.next();
  }

  let (schema, name) = if name.contains('.') {
    let mut parts = name.split('.');
    let schema = parts.next().unwrap();
    let name = parts.next().unwrap();

    (Some(schema.to_string()), name.to_string())
  } else {
    (None, name)
  };

  Ok(Table {
    name,
    schema,
    primary_key: parse_table_primary_key(&mut ParseContext::new(&primary_key))?,
    columns: parse_table_body(&mut ParseContext::new(&body))?,
  })
}

pub fn parse_table_declaration(
  ctx: &mut ParseContext<'_>,
) -> Result<Table, ParseError> {
  ctx.skip_whitespaces();
  if ctx.current_char().unwrap() != 'd' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, table declaration must start with d",
        ctx.current_char().unwrap()
      ),
      ctx,
    ));
  }

  if ctx.last_char().unwrap() != '}' {
    return Err(ParseError::from_parse_context(
      format!(
        "Unexpected character: {}, table declaration must end with }}",
        ctx.last_char().unwrap()
      ),
      ctx,
    ));
  }

  ctx.ignore_str("diesel::table!")?;
  ctx.skip_whitespaces_start();
  ctx.expect_char('{')?;
  let mut ctx = ctx.extract_within_pair('{', '}')?;
  while ctx.contains_str("pub use ") || ctx.contains_str("use ") {
    ctx.ignore_until_char(';')?;
  }
  ctx.ignore_char(';')?;

  ctx.skip_whitespaces();

  parse_table(&mut ctx)
}

pub fn parse_file(
  ctx: &mut ParseContext<'_>,
) -> Result<Vec<Table>, ParseError> {
  let mut tables = Vec::new();

  while !ctx.at_end() {
    ctx.skip_whitespaces();
    if ctx.contains_str("diesel::table!") {
      tables.push(parse_table_declaration(ctx)?);
    } else {
      ctx.ignore_until_char(';')?;
    }
  }

  Ok(tables)
}

#[cfg(test)]
mod test {

  use std::vec;

  use super::*;

  #[test]
  fn extract_within_pair() {
    let str = "{ my_name { test } }";
    let mut ctx = ParseContext::new(str);

    assert_eq!(
      ctx.extract_within_pair('{', '}'),
      Ok(ParseContext {
        str,
        current_index: 1,
        until: str.len() - 1,
      })
    );
  }

  #[test]
  fn ignore_str() {
    let mut ctx = ParseContext::new("test");

    assert_eq!(
      *ctx.ignore_str("test").unwrap(),
      ParseContext {
        str: "test",
        current_index: 4,
        until: 4,
      }
    );
  }

  #[test]
  pub fn move_to_str() {
    let mut ctx = ParseContext::new("test;");

    assert_eq!(
      *ctx.move_to_str(";").unwrap(),
      ParseContext {
        str: "test;",
        current_index: 4,
        until: 5,
      }
    );
  }

  #[test]
  fn ignore_whitespace() {
    let mut ctx = ParseContext::new("  test     ");

    assert_eq!(
      *ctx.skip_whitespaces(),
      ParseContext {
        str: "  test     ",
        current_index: 2,
        until: 6,
      }
    );

    let mut ctx = ParseContext::new("       test     ");

    assert_eq!(
      *ctx.skip_whitespaces(),
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

    assert_eq!(ctx.next_char(), Some('b'));
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
  fn column_simple_optional_comma() {
    assert_eq!(
      parse_column(&mut ParseContext::new("id -> Uuid,")),
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

  #[test]
  fn table_with_schema() {
    assert_eq!(
      parse_table(&mut ParseContext::new(
        r#"
          iam.password_reset_tokens (id) {
              id -> Uuid,
              #[max_length = 255]
              token -> Varchar,
              expires_at -> Timestamptz,
              created_at -> Timestamptz,
          }
        "#
      )),
      Ok(Table {
        name: "password_reset_tokens".to_string(),
        schema: Some("iam".to_string()),
        primary_key: vec!["id".to_string()],
        columns: vec![
          Column {
            name: "id".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Uuid,
              params: vec![],
            }
          },
          Column {
            name: "token".to_string(),
            attributes: vec![Attribute {
              content: "max_length=255".to_string()
            }],
            r#type: Type {
              name: TypeName::Varchar,
              params: vec![],
            }
          },
          Column {
            name: "expires_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Timestamptz,
              params: vec![],
            }
          },
          Column {
            name: "created_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Timestamptz,
              params: vec![],
            }
          },
        ],
      })
    )
  }

  #[test]
  fn table() {
    assert_eq!(
      parse_table(&mut ParseContext::new(
        r#"
          password_reset_tokens (id, created_at) {
              id -> Uuid,
              #[min_length = 255]
              #[max_length = 255]
              token -> Varchar,
              expires_at -> Timestamptz,
              created_at -> Timestamptz,
          }
          "#
      )),
      Ok(Table {
        name: "password_reset_tokens".to_string(),
        schema: None,
        primary_key: vec!["id".to_string(), "created_at".to_string()],
        columns: vec![
          Column {
            name: "id".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Uuid,
              params: vec![],
            }
          },
          Column {
            name: "token".to_string(),
            attributes: vec![
              Attribute {
                content: "min_length=255".to_string()
              },
              Attribute {
                content: "max_length=255".to_string()
              }
            ],
            r#type: Type {
              name: TypeName::Varchar,
              params: vec![],
            }
          },
          Column {
            name: "expires_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Timestamptz,
              params: vec![],
            }
          },
          Column {
            name: "created_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Timestamptz,
              params: vec![],
            }
          },
        ],
      })
    )
  }

  #[test]
  fn table_declaration() {
    assert_eq!(
      parse_table_declaration(&mut ParseContext::new(
        r#"
          diesel::table! {
            use diesel::sql_types::*;
            use super::sql_types::StaffRole;

            iam.staffs (id) {
              id->Uuid,
              created_at->Timestamptz,
              updated_at->Timestamptz,
              deleted_at->Nullable<Timestamptz>,
            }
          }
          "#
      )),
      Ok(Table {
        name: "staffs".to_string(),
        schema: Some("iam".to_string()),
        primary_key: vec!["id".to_string()],
        columns: vec![
          Column {
            name: "id".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Uuid,
              params: vec![],
            }
          },
          Column {
            name: "created_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Timestamptz,
              params: vec![],
            }
          },
          Column {
            name: "updated_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Timestamptz,
              params: vec![],
            }
          },
          Column {
            name: "deleted_at".to_string(),
            attributes: vec![],
            r#type: Type {
              name: TypeName::Nullable,
              params: vec![Type {
                name: TypeName::Timestamptz,
                params: vec![],
              },],
            }
          },
        ],
      })
    )
  }
}
