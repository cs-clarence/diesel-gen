// @generated automatically by diesel-gen

#![allow(unused)]
#![allow(clippy::all)]

use crate::db::model::iam::{Identity as ModelIdentity, User as ModelUser};
use crate::db::model::CredentialType;
use crate::db::model::Gender;
use crate::db::model::StaffRole;
use crate::db::model::SubjectType;
#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct User {
  pub created_at: time::OffsetDateTime,
  pub deleted_at: Option<time::OffsetDateTime>,
  pub display_picture_url: Option<String>,
  pub first_name: String,
  pub gender: Option<Gender>,
  pub id: uuid::Uuid,
  pub identity_id: uuid::Uuid,
  pub last_name: String,
  pub middle_name: Option<String>,
  pub name_suffix: Option<String>,
  pub other_gender: Option<String>,
  pub updated_at: time::OffsetDateTime,
  pub username: String,
}
impl From<(ModelIdentity, ModelUser)> for User {
  fn from(val: (ModelIdentity, ModelUser)) -> Self {
    Self {
      created_at: val.1.created_at,
      deleted_at: val.1.deleted_at,
      display_picture_url: val.0.display_picture_url,
      first_name: val.0.first_name,
      gender: val.0.gender,
      id: val.1.id,
      identity_id: val.1.identity_id,
      last_name: val.0.last_name,
      middle_name: val.0.middle_name,
      name_suffix: val.0.name_suffix,
      other_gender: val.0.other_gender,
      updated_at: val.1.updated_at,
      username: val.0.username,
    }
  }
}
