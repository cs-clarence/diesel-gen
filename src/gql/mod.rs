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
pub struct User {}
impl From<(ModelUser, ModelIdentity)> for User {
  fn from(val: (ModelUser, ModelIdentity)) -> Self {
    todo!()
  }
}
