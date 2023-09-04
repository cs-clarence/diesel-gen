// @generated automatically by diesel-gen

#![allow(unused)]
#![allow(clippy::all)]

use crate::db::schema::iam::{
  credentials,
  email_address_verification_codes,
  email_addresses,
  password_reset_tokens,
  refresh_tokens,
  staff_credentials,
  staff_email_addresses,
  staff_password_reset_tokens,
  staff_refresh_tokens,
  staffs,
  user_credentials,
  user_email_addresses,
  user_password_reset_tokens,
  user_refresh_tokens,
  users,
};

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = credentials)]
#[diesel(primary_key(id))]
pub struct Credential {
  pub id: uuid::Uuid,
  pub enabled: bool,
  #[diesel(column_name = "type_")]
  pub r#type: CredentialType,
  pub content: String,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = credentials)]
pub struct NewCredential<'a>{
  pub id: &'a uuid::Uuid,
  pub enabled: &'a bool,
  #[diesel(column_name = "type_")]
  pub r#type: CredentialType,
  pub content: &'a str,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = credentials)]
pub struct CredentialUpdate<'a>{
  pub enabled: Option<&'a bool>,
  #[diesel(column_name = "type_")]
  pub r#type: Option<CredentialType>,
  pub content: Option<&'a str>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(primary_key(id))]
pub struct EmailAddressVerificationCode {
  pub id: uuid::Uuid,
  pub expires_at: time::OffsetDateTime,
  pub email_address_id: uuid::Uuid,
  pub created_at: time::OffsetDateTime,
  pub invalidated_at: Option<time::OffsetDateTime>,
  pub value: String,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = email_address_verification_codes)]
pub struct NewEmailAddressVerificationCode<'a>{
  pub id: &'a uuid::Uuid,
  pub expires_at: &'a time::OffsetDateTime,
  pub email_address_id: &'a uuid::Uuid,
  pub created_at: &'a time::OffsetDateTime,
  pub invalidated_at: Option<&'a time::OffsetDateTime>,
  pub value: &'a str,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = email_address_verification_codes)]
pub struct EmailAddressVerificationCodeUpdate<'a>{
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub email_address_id: Option<&'a uuid::Uuid>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub invalidated_at: Option<Option<&'a time::OffsetDateTime>>,
  pub value: Option<&'a str>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = email_addresses)]
#[diesel(primary_key(id))]
pub struct EmailAddress {
  pub id: uuid::Uuid,
  pub value: String,
  pub primary: bool,
  pub verified_at: Option<time::OffsetDateTime>,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = email_addresses)]
pub struct NewEmailAddress<'a>{
  pub id: &'a uuid::Uuid,
  pub value: &'a str,
  pub primary: &'a bool,
  pub verified_at: Option<&'a time::OffsetDateTime>,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = email_addresses)]
pub struct EmailAddressUpdate<'a>{
  pub value: Option<&'a str>,
  pub primary: Option<&'a bool>,
  pub verified_at: Option<Option<&'a time::OffsetDateTime>>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(primary_key(id))]
pub struct PasswordResetToken {
  pub id: uuid::Uuid,
  pub token: String,
  pub expires_at: time::OffsetDateTime,
  pub created_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = password_reset_tokens)]
pub struct NewPasswordResetToken<'a>{
  pub id: &'a uuid::Uuid,
  pub token: &'a str,
  pub expires_at: &'a time::OffsetDateTime,
  pub created_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = password_reset_tokens)]
pub struct PasswordResetTokenUpdate<'a>{
  pub token: Option<&'a str>,
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub created_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = refresh_tokens)]
#[diesel(primary_key(id))]
pub struct RefreshToken {
  pub id: uuid::Uuid,
  pub value: String,
  pub scope: String,
  pub audience: String,
  pub subject: uuid::Uuid,
  pub subject_type: SubjectType,
  pub expires_at: time::OffsetDateTime,
  pub created_at: time::OffsetDateTime,
  pub invalidated_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct NewRefreshToken<'a>{
  pub id: &'a uuid::Uuid,
  pub value: &'a str,
  pub scope: &'a str,
  pub audience: &'a str,
  pub subject: &'a uuid::Uuid,
  pub subject_type: SubjectType,
  pub expires_at: &'a time::OffsetDateTime,
  pub created_at: &'a time::OffsetDateTime,
  pub invalidated_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = refresh_tokens)]
pub struct RefreshTokenUpdate<'a>{
  pub value: Option<&'a str>,
  pub scope: Option<&'a str>,
  pub audience: Option<&'a str>,
  pub subject: Option<&'a uuid::Uuid>,
  pub subject_type: Option<SubjectType>,
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub invalidated_at: Option<Option<&'a time::OffsetDateTime>>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = staff_credentials)]
#[diesel(primary_key(staff_id, credential_id))]
pub struct StaffCredential {
  pub staff_id: uuid::Uuid,
  pub credential_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = staff_credentials)]
pub struct NewStaffCredential<'a>{
  pub staff_id: &'a uuid::Uuid,
  pub credential_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = staff_email_addresses)]
#[diesel(primary_key(staff_id, email_address_id))]
pub struct StaffEmailAddress {
  pub staff_id: uuid::Uuid,
  pub email_address_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = staff_email_addresses)]
pub struct NewStaffEmailAddress<'a>{
  pub staff_id: &'a uuid::Uuid,
  pub email_address_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = staff_password_reset_tokens)]
#[diesel(primary_key(staff_id, password_reset_token_id))]
pub struct StaffPasswordResetToken {
  pub staff_id: uuid::Uuid,
  pub password_reset_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = staff_password_reset_tokens)]
pub struct NewStaffPasswordResetToken<'a>{
  pub staff_id: &'a uuid::Uuid,
  pub password_reset_token_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = staff_refresh_tokens)]
#[diesel(primary_key(staff_id, refresh_token_id))]
pub struct StaffRefreshToken {
  pub staff_id: uuid::Uuid,
  pub refresh_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = staff_refresh_tokens)]
pub struct NewStaffRefreshToken<'a>{
  pub staff_id: &'a uuid::Uuid,
  pub refresh_token_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = staffs)]
#[diesel(primary_key(id))]
pub struct Staff {
  pub id: uuid::Uuid,
  pub username: String,
  pub display_picture_url: Option<String>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: String,
  pub name_suffix: Option<String>,
  pub role: StaffRole,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
  pub deleted_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = staffs)]
pub struct NewStaff<'a>{
  pub id: &'a uuid::Uuid,
  pub username: &'a str,
  pub display_picture_url: Option<&'a str>,
  pub first_name: &'a str,
  pub middle_name: Option<&'a str>,
  pub last_name: &'a str,
  pub name_suffix: Option<&'a str>,
  pub role: StaffRole,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
  pub deleted_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = staffs)]
pub struct StaffUpdate<'a>{
  pub username: Option<&'a str>,
  pub display_picture_url: Option<Option<&'a str>>,
  pub first_name: Option<&'a str>,
  pub middle_name: Option<Option<&'a str>>,
  pub last_name: Option<&'a str>,
  pub name_suffix: Option<Option<&'a str>>,
  pub role: Option<StaffRole>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
  pub deleted_at: Option<Option<&'a time::OffsetDateTime>>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = user_credentials)]
#[diesel(primary_key(user_id, credential_id))]
pub struct UserCredential {
  pub user_id: uuid::Uuid,
  pub credential_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = user_credentials)]
pub struct NewUserCredential<'a>{
  pub user_id: &'a uuid::Uuid,
  pub credential_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = user_email_addresses)]
#[diesel(primary_key(user_id, email_address_id))]
pub struct UserEmailAddress {
  pub user_id: uuid::Uuid,
  pub email_address_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = user_email_addresses)]
pub struct NewUserEmailAddress<'a>{
  pub user_id: &'a uuid::Uuid,
  pub email_address_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = user_password_reset_tokens)]
#[diesel(primary_key(user_id, password_reset_token_id))]
pub struct UserPasswordResetToken {
  pub user_id: uuid::Uuid,
  pub password_reset_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = user_password_reset_tokens)]
pub struct NewUserPasswordResetToken<'a>{
  pub user_id: &'a uuid::Uuid,
  pub password_reset_token_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = user_refresh_tokens)]
#[diesel(primary_key(user_id, refresh_token_id))]
pub struct UserRefreshToken {
  pub user_id: uuid::Uuid,
  pub refresh_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = user_refresh_tokens)]
pub struct NewUserRefreshToken<'a>{
  pub user_id: &'a uuid::Uuid,
  pub refresh_token_id: &'a uuid::Uuid,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Queryable, diesel::Insertable, diesel::Selectable, diesel::Identifiable, diesel::AsChangeset)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
  pub id: uuid::Uuid,
  pub username: String,
  pub display_picture_url: Option<String>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: String,
  pub name_suffix: Option<String>,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
  pub deleted_at: Option<time::OffsetDateTime>,
  pub gender: Option<Gender>,
  pub other_gender: Option<String>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a>{
  pub id: &'a uuid::Uuid,
  pub username: &'a str,
  pub display_picture_url: Option<&'a str>,
  pub first_name: &'a str,
  pub middle_name: Option<&'a str>,
  pub last_name: &'a str,
  pub name_suffix: Option<&'a str>,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
  pub deleted_at: Option<&'a time::OffsetDateTime>,
  pub gender: Option<Gender>,
  pub other_gender: Option<&'a str>,
}

#[derive(Clone, Debug, Default)]
#[derive(diesel::AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdate<'a>{
  pub username: Option<&'a str>,
  pub display_picture_url: Option<Option<&'a str>>,
  pub first_name: Option<&'a str>,
  pub middle_name: Option<Option<&'a str>>,
  pub last_name: Option<&'a str>,
  pub name_suffix: Option<Option<&'a str>>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
  pub deleted_at: Option<Option<&'a time::OffsetDateTime>>,
  pub gender: Option<Option<Gender>>,
  pub other_gender: Option<Option<&'a str>>,
}

