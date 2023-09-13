// @generated automatically by diesel-gen

#![allow(unused)]
#![allow(clippy::all)]

pub mod enums;
use crate::db::model::CredentialType;
use crate::db::model::Gender;
use crate::db::model::StaffRole;
use crate::db::model::SubjectType;
use crate::db::schema::iam::{
  _prisma_migrations, credentials, email_address_verification_codes,
  email_addresses, identities, password_reset_tokens, refresh_tokens, staffs,
  users,
};
#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = _prisma_migrations)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PrismaMigration {
  pub id: String,
  pub checksum: String,
  pub finished_at: Option<time::OffsetDateTime>,
  pub migration_name: String,
  pub logs: Option<String>,
  pub rolled_back_at: Option<time::OffsetDateTime>,
  pub started_at: time::OffsetDateTime,
  pub applied_steps_count: i32,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = _prisma_migrations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPrismaMigration<'a> {
  pub id: &'a str,
  pub checksum: &'a str,
  pub finished_at: Option<&'a time::OffsetDateTime>,
  pub migration_name: &'a str,
  pub logs: Option<&'a str>,
  pub rolled_back_at: Option<&'a time::OffsetDateTime>,
  pub started_at: &'a time::OffsetDateTime,
  pub applied_steps_count: &'a i32,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = _prisma_migrations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PrismaMigrationUpdate<'a> {
  pub checksum: Option<&'a str>,
  pub finished_at: Option<Option<&'a time::OffsetDateTime>>,
  pub migration_name: Option<&'a str>,
  pub logs: Option<Option<&'a str>>,
  pub rolled_back_at: Option<Option<&'a time::OffsetDateTime>>,
  pub started_at: Option<&'a time::OffsetDateTime>,
  pub applied_steps_count: Option<&'a i32>,
}

impl PrismaMigration {
  pub async fn update(
    id: &str,
    changes: &PrismaMigrationUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((changes,))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_checksum(
    id: &str,
    checksum: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::checksum.eq(checksum),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_finished_at(
    id: &str,
    finished_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::finished_at.eq(finished_at),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_migration_name(
    id: &str,
    migration_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::migration_name.eq(migration_name),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_logs(
    id: &str,
    logs: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::logs.eq(logs),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_rolled_back_at(
    id: &str,
    rolled_back_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::rolled_back_at.eq(rolled_back_at),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_started_at(
    id: &str,
    started_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::started_at.eq(started_at),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
  pub async fn update_applied_steps_count(
    id: &str,
    applied_steps_count: &i32,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .set((_prisma_migrations::applied_steps_count.eq(applied_steps_count),))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
}

impl PrismaMigration {
  pub async fn delete(
    id: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(&mut conn)
      .await
  }
}

impl PrismaMigration {
  pub async fn insert(
    data: &NewPrismaMigration<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(_prisma_migrations::table)
      .values(data)
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub identity_id: uuid::Uuid,
  pub enabled: bool,
  #[diesel(column_name = "type_")]
  pub r#type: CredentialType,
  pub content: String,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCredential<'a> {
  pub id: &'a uuid::Uuid,
  pub identity_id: &'a uuid::Uuid,
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
  pub identity_id: Option<&'a uuid::Uuid>,
  pub enabled: Option<&'a bool>,
  #[diesel(column_name = "type_")]
  pub r#type: Option<CredentialType>,
  pub content: Option<&'a str>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

impl Credential {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &CredentialUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_identity_id(
    id: &uuid::Uuid,
    identity_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(credentials::table)
      .filter(credentials::id.eq(id))
      .set((
        credentials::updated_at.eq(diesel::dsl::now),
        credentials::identity_id.eq(identity_id),
      ))
      .returning(Credential::as_returning())
      .get_result::<Credential>(&mut conn)
      .await
  }
  pub async fn update_enabled(
    id: &uuid::Uuid,
    enabled: &bool,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_type(
    id: &uuid::Uuid,
    type_: CredentialType,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_content(
    id: &uuid::Uuid,
    content: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_updated_at(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewCredential<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(credentials::table)
      .values(data)
      .returning(Credential::as_returning())
      .get_result::<Credential>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub value: String,
  pub expires_at: time::OffsetDateTime,
  pub email_address_id: uuid::Uuid,
  pub created_at: time::OffsetDateTime,
  pub invalidated_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmailAddressVerificationCode<'a> {
  pub id: &'a uuid::Uuid,
  pub value: &'a str,
  pub expires_at: &'a time::OffsetDateTime,
  pub email_address_id: &'a uuid::Uuid,
  pub created_at: &'a time::OffsetDateTime,
  pub invalidated_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddressVerificationCodeUpdate<'a> {
  pub value: Option<&'a str>,
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub email_address_id: Option<&'a uuid::Uuid>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub invalidated_at: Option<Option<&'a time::OffsetDateTime>>,
}

impl EmailAddressVerificationCode {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &EmailAddressVerificationCodeUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_value(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_expires_at(
    id: &uuid::Uuid,
    expires_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_email_address_id(
    id: &uuid::Uuid,
    email_address_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_invalidated_at(
    id: &uuid::Uuid,
    invalidated_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
}

impl EmailAddressVerificationCode {
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewEmailAddressVerificationCode<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_address_verification_codes::table)
      .values(data)
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub identity_id: uuid::Uuid,
  pub value: String,
  pub primary: bool,
  pub verified_at: Option<time::OffsetDateTime>,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = email_addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmailAddress<'a> {
  pub id: &'a uuid::Uuid,
  pub identity_id: &'a uuid::Uuid,
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
  pub identity_id: Option<&'a uuid::Uuid>,
  pub value: Option<&'a str>,
  pub primary: Option<&'a bool>,
  pub verified_at: Option<Option<&'a time::OffsetDateTime>>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

impl EmailAddress {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &EmailAddressUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_identity_id(
    id: &uuid::Uuid,
    identity_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .set((
        email_addresses::updated_at.eq(diesel::dsl::now),
        email_addresses::identity_id.eq(identity_id),
      ))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(&mut conn)
      .await
  }
  pub async fn update_value(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_primary(
    id: &uuid::Uuid,
    primary: &bool,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_verified_at(
    id: &uuid::Uuid,
    verified_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_updated_at(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewEmailAddress<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_addresses::table)
      .values(data)
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = identities)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
  pub id: uuid::Uuid,
  pub username: String,
  pub display_picture_url: Option<String>,
  pub first_name: String,
  pub middle_name: Option<String>,
  pub last_name: String,
  pub name_suffix: Option<String>,
  pub gender: Option<Gender>,
  pub other_gender: Option<String>,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewIdentity<'a> {
  pub id: &'a uuid::Uuid,
  pub username: &'a str,
  pub display_picture_url: Option<&'a str>,
  pub first_name: &'a str,
  pub middle_name: Option<&'a str>,
  pub last_name: &'a str,
  pub name_suffix: Option<&'a str>,
  pub gender: Option<Gender>,
  pub other_gender: Option<&'a str>,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = identities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IdentityUpdate<'a> {
  pub username: Option<&'a str>,
  pub display_picture_url: Option<Option<&'a str>>,
  pub first_name: Option<&'a str>,
  pub middle_name: Option<Option<&'a str>>,
  pub last_name: Option<&'a str>,
  pub name_suffix: Option<Option<&'a str>>,
  pub gender: Option<Option<Gender>>,
  pub other_gender: Option<Option<&'a str>>,
}

impl Identity {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &IdentityUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((changes,))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_username(
    id: &uuid::Uuid,
    username: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::username.eq(username),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_display_picture_url(
    id: &uuid::Uuid,
    display_picture_url: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::display_picture_url.eq(display_picture_url),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_first_name(
    id: &uuid::Uuid,
    first_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::first_name.eq(first_name),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_middle_name(
    id: &uuid::Uuid,
    middle_name: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::middle_name.eq(middle_name),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_last_name(
    id: &uuid::Uuid,
    last_name: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::last_name.eq(last_name),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_name_suffix(
    id: &uuid::Uuid,
    name_suffix: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::name_suffix.eq(name_suffix),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_gender(
    id: &uuid::Uuid,
    gender: Option<Gender>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::gender.eq(gender),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
  pub async fn update_other_gender(
    id: &uuid::Uuid,
    other_gender: Option<&str>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(identities::table)
      .filter(identities::id.eq(id))
      .set((identities::other_gender.eq(other_gender),))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
}

impl Identity {
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(identities::table)
      .filter(identities::id.eq(id))
      .returning(Identity::as_returning())
      .get_result::<Identity>(&mut conn)
      .await
  }
}

impl Identity {
  pub async fn insert(
    data: &NewIdentity<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(identities::table)
      .values(data)
      .returning(Identity::as_returning())
      .get_result::<Identity>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub value: String,
  pub expires_at: time::OffsetDateTime,
  pub created_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPasswordResetToken<'a> {
  pub id: &'a uuid::Uuid,
  pub value: &'a str,
  pub expires_at: &'a time::OffsetDateTime,
  pub created_at: &'a time::OffsetDateTime,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordResetTokenUpdate<'a> {
  pub value: Option<&'a str>,
  pub expires_at: Option<&'a time::OffsetDateTime>,
  pub created_at: Option<&'a time::OffsetDateTime>,
}

impl PasswordResetToken {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &PasswordResetTokenUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_value(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .set((password_reset_tokens::value.eq(value),))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(&mut conn)
      .await
  }
  pub async fn update_expires_at(
    id: &uuid::Uuid,
    expires_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewPasswordResetToken<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(password_reset_tokens::table)
      .values(data)
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub identity_id: uuid::Uuid,
  pub value: String,
  pub scope: String,
  pub audience: String,
  pub subject: uuid::Uuid,
  pub subject_type: SubjectType,
  pub expires_at: time::OffsetDateTime,
  pub created_at: time::OffsetDateTime,
  pub invalidated_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRefreshToken<'a> {
  pub id: &'a uuid::Uuid,
  pub identity_id: &'a uuid::Uuid,
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
  pub identity_id: Option<&'a uuid::Uuid>,
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
  pub async fn update(
    id: &uuid::Uuid,
    changes: &RefreshTokenUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_identity_id(
    id: &uuid::Uuid,
    identity_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .set((refresh_tokens::identity_id.eq(identity_id),))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(&mut conn)
      .await
  }
  pub async fn update_value(
    id: &uuid::Uuid,
    value: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_scope(
    id: &uuid::Uuid,
    scope: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_audience(
    id: &uuid::Uuid,
    audience: &str,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_subject(
    id: &uuid::Uuid,
    subject: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_subject_type(
    id: &uuid::Uuid,
    subject_type: SubjectType,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_expires_at(
    id: &uuid::Uuid,
    expires_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_invalidated_at(
    id: &uuid::Uuid,
    invalidated_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewRefreshToken<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(refresh_tokens::table)
      .values(data)
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub role: StaffRole,
  pub identity_id: uuid::Uuid,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
  pub deleted_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = staffs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStaff<'a> {
  pub id: &'a uuid::Uuid,
  pub role: StaffRole,
  pub identity_id: &'a uuid::Uuid,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
  pub deleted_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = staffs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StaffUpdate<'a> {
  pub role: Option<StaffRole>,
  pub identity_id: Option<&'a uuid::Uuid>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
  pub deleted_at: Option<Option<&'a time::OffsetDateTime>>,
}

impl Staff {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &StaffUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_role(
    id: &uuid::Uuid,
    role: StaffRole,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_identity_id(
    id: &uuid::Uuid,
    identity_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(staffs::table)
      .filter(staffs::id.eq(id))
      .set((
        staffs::updated_at.eq(diesel::dsl::now),
        staffs::identity_id.eq(identity_id),
      ))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_updated_at(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_deleted_at(
    id: &uuid::Uuid,
    deleted_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(staffs::table)
      .filter(staffs::id.eq(id))
      .returning(Staff::as_returning())
      .get_result::<Staff>(&mut conn)
      .await
  }
}

impl Staff {
  pub async fn soft_delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewStaff<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(staffs::table)
      .values(data)
      .returning(Staff::as_returning())
      .get_result::<Staff>(conn)
      .await
  }
}

#[derive(
  Clone,
  Debug,
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
  pub identity_id: uuid::Uuid,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
  pub deleted_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
  pub id: &'a uuid::Uuid,
  pub identity_id: &'a uuid::Uuid,
  pub created_at: &'a time::OffsetDateTime,
  pub updated_at: &'a time::OffsetDateTime,
  pub deleted_at: Option<&'a time::OffsetDateTime>,
}

#[derive(Clone, Debug, Default, diesel::AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserUpdate<'a> {
  pub identity_id: Option<&'a uuid::Uuid>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
  pub deleted_at: Option<Option<&'a time::OffsetDateTime>>,
}

impl User {
  pub async fn update(
    id: &uuid::Uuid,
    changes: &UserUpdate<'_>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_identity_id(
    id: &uuid::Uuid,
    identity_id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::update(users::table)
      .filter(users::id.eq(id))
      .set((
        users::updated_at.eq(diesel::dsl::now),
        users::identity_id.eq(identity_id),
      ))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
  pub async fn update_created_at(
    id: &uuid::Uuid,
    created_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_updated_at(
    id: &uuid::Uuid,
    updated_at: &time::OffsetDateTime,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn update_deleted_at(
    id: &uuid::Uuid,
    deleted_at: Option<&time::OffsetDateTime>,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
}

impl User {
  pub async fn delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(users::table)
      .filter(users::id.eq(id))
      .returning(User::as_returning())
      .get_result::<User>(&mut conn)
      .await
  }
}

impl User {
  pub async fn soft_delete(
    id: &uuid::Uuid,
    mut conn: Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
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
  pub async fn insert(
    data: &NewUser<'_>,
    conn: &mut Conn,
  ) -> Result<Self, diesel::result::Error>
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(users::table)
      .values(data)
      .returning(User::as_returning())
      .get_result::<User>(conn)
      .await
  }
}
