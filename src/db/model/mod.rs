// @generated automatically by diesel-gen

#![allow(unused)]
#![allow(clippy::all)]

pub mod enums;
use crate::db::model::CredentialType;
use crate::db::model::Gender;
use crate::db::model::StaffRole;
use crate::db::model::SubjectType;
use crate::db::schema::iam::{
  credentials, email_address_verification_codes, email_addresses,
  password_reset_tokens, refresh_tokens, staff_credentials,
  staff_email_addresses, staff_password_reset_tokens, staff_refresh_tokens,
  staffs, user_credentials, user_email_addresses, user_password_reset_tokens,
  user_refresh_tokens, users,
};
#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = credentials)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Credential {
  pub id: uuid::Uuid,
  pub enabled: bool,
  #[diesel(column_name = "type_")]
  pub r#type: CredentialType,
  pub content: String,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCredential<'a> {
  pub id: &'a uuid::Uuid,
  pub enabled: &'a bool,
  #[diesel(column_name = "type_")]
  pub r#type: CredentialType,
  pub content: &'a str,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CredentialUpdate<'a> {
  pub enabled: Option<&'a bool>,
  #[diesel(column_name = "type_")]
  pub r#type: Option<CredentialType>,
  pub content: Option<&'a str>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

impl Credential {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(credentials::table)
      .filter(credentials::id.eq(id))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
}

impl Credential {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &CredentialUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((credentials::updated_at.eq(diesel::dsl::now), changes))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
  pub async fn update_enabled<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    enabled: &bool,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((
        credentials::updated_at.eq(diesel::dsl::now),
        credentials::enabled.eq(enabled),
      ))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
  pub async fn update_type<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    type_: CredentialType,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((
        credentials::updated_at.eq(diesel::dsl::now),
        credentials::type_.eq(type_),
      ))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
  pub async fn update_content<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    content: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((
        credentials::updated_at.eq(diesel::dsl::now),
        credentials::content.eq(content),
      ))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((
        credentials::updated_at.eq(diesel::dsl::now),
        credentials::created_at.eq(created_at),
      ))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
  pub async fn update_updated_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((credentials::updated_at.eq(updated_at),))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
}

impl Credential {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewCredential<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(credentials::table)
      .values(data)
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddressVerificationCode {
  pub id: uuid::Uuid,
  pub expires_at: time::OffsetDateTime,
  pub email_address_id: uuid::Uuid,
  pub created_at: time::OffsetDateTime,
  pub invalidated_at: Option<time::OffsetDateTime>,
  pub value: String,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmailAddressVerificationCode<'a> {
  pub id: &'a uuid::Uuid,
  pub expires_at: &'a time::OffsetDateTime,
  pub email_address_id: &'a uuid::Uuid,
  pub created_at: &'a time::OffsetDateTime,
  pub invalidated_at: Option<&'a time::OffsetDateTime>,
  pub value: &'a str,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddressVerificationCodeUpdate<'a> {
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub email_address_id: Option<&'a uuid::Uuid>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub invalidated_at: Option<Option<&'a time::OffsetDateTime>>,
  pub value: Option<&'a str>,
}

impl EmailAddressVerificationCode {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
}

impl EmailAddressVerificationCode {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &EmailAddressVerificationCodeUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .set((changes,))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
  pub async fn update_expires_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    expires_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .set((email_address_verification_codes::expires_at.eq(expires_at),))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
  pub async fn update_email_address_id<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    email_address_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .set((
        email_address_verification_codes::email_address_id.eq(email_address_id),
      ))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .set((email_address_verification_codes::created_at.eq(created_at),))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
  pub async fn update_invalidated_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    invalidated_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .set((
        email_address_verification_codes::invalidated_at.eq(invalidated_at),
      ))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
  pub async fn update_value<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .set((email_address_verification_codes::value.eq(value),))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
}

impl EmailAddressVerificationCode {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewEmailAddressVerificationCode<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_address_verification_codes::table)
      .values(data)
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(&mut conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = email_addresses)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddress {
  pub id: uuid::Uuid,
  pub value: String,
  pub primary: bool,
  pub verified_at: Option<time::OffsetDateTime>,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = email_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmailAddress<'a> {
  pub id: &'a uuid::Uuid,
  pub value: &'a str,
  pub primary: &'a bool,
  pub verified_at: Option<&'a time::OffsetDateTime>,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = email_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddressUpdate<'a> {
  pub value: Option<&'a str>,
  pub primary: Option<&'a bool>,
  pub verified_at: Option<Option<&'a time::OffsetDateTime>>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

impl EmailAddress {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
}

impl EmailAddress {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &EmailAddressUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((email_addresses::updated_at.eq(diesel::dsl::now), changes))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
  pub async fn update_value<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((
        email_addresses::updated_at.eq(diesel::dsl::now),
        email_addresses::value.eq(value),
      ))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
  pub async fn update_primary<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    primary: &bool,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((
        email_addresses::updated_at.eq(diesel::dsl::now),
        email_addresses::primary.eq(primary),
      ))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
  pub async fn update_verified_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    verified_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((
        email_addresses::updated_at.eq(diesel::dsl::now),
        email_addresses::verified_at.eq(verified_at),
      ))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((
        email_addresses::updated_at.eq(diesel::dsl::now),
        email_addresses::created_at.eq(created_at),
      ))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
  pub async fn update_updated_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((email_addresses::updated_at.eq(updated_at),))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
}

impl EmailAddress {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewEmailAddress<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_addresses::table)
      .values(data)
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordResetToken {
  pub id: uuid::Uuid,
  pub token: String,
  pub expires_at: time::OffsetDateTime,
  pub created_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPasswordResetToken<'a> {
  pub id: &'a uuid::Uuid,
  pub token: &'a str,
  pub expires_at: &'a time::OffsetDateTime,
  pub created_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordResetTokenUpdate<'a> {
  pub token: Option<&'a str>,
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub created_at: Option<&'a time::OffsetDateTime>,
}

impl PasswordResetToken {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
}

impl PasswordResetToken {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &PasswordResetTokenUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .set((changes,))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
  pub async fn update_token<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    token: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .set((password_reset_tokens::token.eq(token),))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
  pub async fn update_expires_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    expires_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .set((password_reset_tokens::expires_at.eq(expires_at),))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .set((password_reset_tokens::created_at.eq(created_at),))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
}

impl PasswordResetToken {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewPasswordResetToken<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(password_reset_tokens::table)
      .values(data)
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = refresh_tokens)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRefreshToken<'a> {
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

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshTokenUpdate<'a> {
  pub value: Option<&'a str>,
  pub scope: Option<&'a str>,
  pub audience: Option<&'a str>,
  pub subject: Option<&'a uuid::Uuid>,
  pub subject_type: Option<SubjectType>,
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub invalidated_at: Option<Option<&'a time::OffsetDateTime>>,
}

impl RefreshToken {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
}

impl RefreshToken {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &RefreshTokenUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((changes,))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_value<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::value.eq(value),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_scope<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    scope: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::scope.eq(scope),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_audience<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    audience: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::audience.eq(audience),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_subject<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    subject: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::subject.eq(subject),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_subject_type<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    subject_type: SubjectType,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::subject_type.eq(subject_type),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_expires_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    expires_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::expires_at.eq(expires_at),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::created_at.eq(created_at),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_invalidated_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    invalidated_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::invalidated_at.eq(invalidated_at),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
}

impl RefreshToken {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewRefreshToken<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(refresh_tokens::table)
      .values(data)
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = staff_credentials)]
#[diesel(primary_key(staff_id, credential_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StaffCredential {
  pub staff_id: uuid::Uuid,
  pub credential_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = staff_credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStaffCredential<'a> {
  pub staff_id: &'a uuid::Uuid,
  pub credential_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = staff_email_addresses)]
#[diesel(primary_key(staff_id, email_address_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StaffEmailAddress {
  pub staff_id: uuid::Uuid,
  pub email_address_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = staff_email_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStaffEmailAddress<'a> {
  pub staff_id: &'a uuid::Uuid,
  pub email_address_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = staff_password_reset_tokens)]
#[diesel(primary_key(staff_id, password_reset_token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StaffPasswordResetToken {
  pub staff_id: uuid::Uuid,
  pub password_reset_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = staff_password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStaffPasswordResetToken<'a> {
  pub staff_id: &'a uuid::Uuid,
  pub password_reset_token_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = staff_refresh_tokens)]
#[diesel(primary_key(staff_id, refresh_token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StaffRefreshToken {
  pub staff_id: uuid::Uuid,
  pub refresh_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = staff_refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStaffRefreshToken<'a> {
  pub staff_id: &'a uuid::Uuid,
  pub refresh_token_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = staffs)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = staffs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStaff<'a> {
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

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = staffs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StaffUpdate<'a> {
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

impl Staff {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(staffs::table)
      .filter(staffs::id.eq(id))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn soft_delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::deleted_at.eq(diesel::dsl::now),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
}

impl Staff {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &StaffUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((staffs::updated_at.eq(diesel::dsl::now), changes))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_username<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    username: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::username.eq(username),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_display_picture_url<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    display_picture_url: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::display_picture_url.eq(display_picture_url),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_first_name<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    first_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::first_name.eq(first_name),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_middle_name<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    middle_name: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::middle_name.eq(middle_name),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_last_name<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    last_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::last_name.eq(last_name),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_name_suffix<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    name_suffix: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::name_suffix.eq(name_suffix),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_role<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    role: StaffRole,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::role.eq(role),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::created_at.eq(created_at),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_updated_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((staffs::updated_at.eq(updated_at),))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_deleted_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    deleted_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::deleted_at.eq(deleted_at),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
}

impl Staff {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewStaff<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(staffs::table)
      .values(data)
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = user_credentials)]
#[diesel(primary_key(user_id, credential_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserCredential {
  pub user_id: uuid::Uuid,
  pub credential_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = user_credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserCredential<'a> {
  pub user_id: &'a uuid::Uuid,
  pub credential_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = user_email_addresses)]
#[diesel(primary_key(user_id, email_address_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserEmailAddress {
  pub user_id: uuid::Uuid,
  pub email_address_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = user_email_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserEmailAddress<'a> {
  pub user_id: &'a uuid::Uuid,
  pub email_address_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = user_password_reset_tokens)]
#[diesel(primary_key(user_id, password_reset_token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserPasswordResetToken {
  pub user_id: uuid::Uuid,
  pub password_reset_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = user_password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserPasswordResetToken<'a> {
  pub user_id: &'a uuid::Uuid,
  pub password_reset_token_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
)]
#[diesel(table_name = user_refresh_tokens)]
#[diesel(primary_key(user_id, refresh_token_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRefreshToken {
  pub user_id: uuid::Uuid,
  pub refresh_token_id: uuid::Uuid,
}

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = user_refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserRefreshToken<'a> {
  pub user_id: &'a uuid::Uuid,
  pub refresh_token_id: &'a uuid::Uuid,
}

#[derive(
  Clone,
  Debug,
  Default,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

#[derive(Clone, Debug, Default, diesel::Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
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

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserUpdate<'a> {
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

impl User {
  pub async fn delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(users::table)
      .filter(users::id.eq(id))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn soft_delete<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::deleted_at.eq(diesel::dsl::now),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
}

impl User {
  pub async fn update<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    changes: &UserUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((users::updated_at.eq(diesel::dsl::now), changes))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_username<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    username: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::username.eq(username),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_display_picture_url<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    display_picture_url: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::display_picture_url.eq(display_picture_url),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_first_name<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    first_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::first_name.eq(first_name),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_middle_name<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    middle_name: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::middle_name.eq(middle_name),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_last_name<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    last_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::last_name.eq(last_name),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_name_suffix<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    name_suffix: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::name_suffix.eq(name_suffix),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_created_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::created_at.eq(created_at),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_updated_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((users::updated_at.eq(updated_at),))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_deleted_at<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    deleted_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::deleted_at.eq(deleted_at),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_gender<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    gender: Option<Gender>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::gender.eq(gender),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_other_gender<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    other_gender: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::other_gender.eq(other_gender),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
}

impl User {
  pub async fn insert<
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg>,
  >(
    id: &uuid::Uuid,
    data: &NewUser<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error> {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(users::table)
      .values(data)
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
}
