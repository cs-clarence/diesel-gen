// @generated automatically by diesel-gen

#![allow(unused)]
#![allow(clippy::all)]

pub mod enums;
use crate::db::model::CredentialType;
use crate::db::model::Gender;
use crate::db::model::StaffRole;
use crate::db::model::SubjectType;
use crate::db::model::TestType;
use crate::db::schema::iam::{
  _prisma_migrations, credentials, email_address_verification_codes,
  email_addresses, identities, password_reset_tokens, refresh_tokens, staffs,
  users,
};
#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = _prisma_migrations)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PrismaMigration {
  pub id: TestType,
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
  pub id: &'a TestType,
  pub checksum: &'a str,
  pub finished_at: Option<&'a time::OffsetDateTime>,
  pub migration_name: &'a str,
  pub logs: Option<&'a str>,
  pub rolled_back_at: Option<&'a time::OffsetDateTime>,
  pub started_at: &'a time::OffsetDateTime,
  pub applied_steps_count: i32,
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
  pub applied_steps_count: Option<i32>,
}

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = credentials)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Credential {
  pub id: TestType,
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
  pub id: &'a TestType,
  pub identity_id: &'a uuid::Uuid,
  pub enabled: bool,
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
  pub enabled: Option<bool>,
  #[diesel(column_name = "type_")]
  pub r#type: Option<CredentialType>,
  pub content: Option<&'a str>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = email_address_verification_codes)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddressVerificationCode {
  pub id: TestType,
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
  pub id: &'a TestType,
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

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = email_addresses)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailAddress {
  pub id: TestType,
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
  pub id: &'a TestType,
  pub identity_id: &'a uuid::Uuid,
  pub value: &'a str,
  pub primary: bool,
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
  pub primary: Option<bool>,
  pub verified_at: Option<Option<&'a time::OffsetDateTime>>,
  pub created_at: Option<&'a time::OffsetDateTime>,
  pub updated_at: Option<&'a time::OffsetDateTime>,
}

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = identities)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identity {
  pub id: TestType,
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
  pub id: &'a TestType,
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

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordResetToken {
  pub id: TestType,
  pub value: String,
  pub expires_at: time::OffsetDateTime,
  pub created_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPasswordResetToken<'a> {
  pub id: &'a TestType,
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

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = refresh_tokens)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
  pub id: TestType,
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
  pub id: &'a TestType,
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

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = staffs)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Staff {
  pub id: TestType,
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
  pub id: &'a TestType,
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

#[derive(
  Clone,
  Debug,
  diesel::Queryable,
  diesel::QueryableByName,
  diesel::Insertable,
  diesel::Selectable,
  diesel::Identifiable,
  diesel::AsChangeset,
)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: TestType,
  pub identity_id: uuid::Uuid,
  pub created_at: time::OffsetDateTime,
  pub updated_at: time::OffsetDateTime,
  pub deleted_at: Option<time::OffsetDateTime>,
}

#[derive(Clone, Debug, diesel::Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
  pub id: &'a TestType,
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
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a UserUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<User>(conn)
  }
  pub fn update_identity_id<'a, Conn>(
    id: &'a TestType,
    identity_id: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<User>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<User>(conn)
  }
  pub fn update_updated_at<'a, Conn>(
    id: &'a TestType,
    updated_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<User>(conn)
  }
  pub fn update_deleted_at<'a, Conn>(
    id: &'a TestType,
    deleted_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<User>(conn)
  }
}

impl User {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(users::table)
      .filter(users::id.eq(id))
      .returning(User::as_returning())
      .get_result::<User>(conn)
  }
}

impl User {
  pub fn soft_delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<User>(conn)
  }
}

impl User {
  pub fn insert<'a, Conn>(
    data: &'a NewUser<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(users::table)
      .values(data)
      .returning(User::as_returning())
      .get_result::<User>(conn)
  }
}

pub enum UserOrderBy {
  IdAsc,
  IdDesc,
  IdentityIdAsc,
  IdentityIdDesc,
  CreatedAtAsc,
  CreatedAtDesc,
  UpdatedAtAsc,
  UpdatedAtDesc,
  DeletedAtAsc,
  DeletedAtDesc,
}
impl User {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<UserOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      users::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> users::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = users::table
      .filter(users::deleted_at.is_null())
      .into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          UserOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(users::id.asc());
            } else {
              q = q.then_order_by(users::id.asc());
            }
          }

          UserOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(users::id.desc());
            } else {
              q = q.then_order_by(users::id.desc());
            }
          }

          UserOrderBy::IdentityIdAsc => {
            if idx == 0 {
              q = q.order_by(users::identity_id.asc());
            } else {
              q = q.then_order_by(users::identity_id.asc());
            }
          }

          UserOrderBy::IdentityIdDesc => {
            if idx == 0 {
              q = q.order_by(users::identity_id.desc());
            } else {
              q = q.then_order_by(users::identity_id.desc());
            }
          }

          UserOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(users::created_at.asc());
            } else {
              q = q.then_order_by(users::created_at.asc());
            }
          }

          UserOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(users::created_at.desc());
            } else {
              q = q.then_order_by(users::created_at.desc());
            }
          }

          UserOrderBy::UpdatedAtAsc => {
            if idx == 0 {
              q = q.order_by(users::updated_at.asc());
            } else {
              q = q.then_order_by(users::updated_at.asc());
            }
          }

          UserOrderBy::UpdatedAtDesc => {
            if idx == 0 {
              q = q.order_by(users::updated_at.desc());
            } else {
              q = q.then_order_by(users::updated_at.desc());
            }
          }

          UserOrderBy::DeletedAtAsc => {
            if idx == 0 {
              q = q.order_by(users::deleted_at.asc());
            } else {
              q = q.then_order_by(users::deleted_at.asc());
            }
          }

          UserOrderBy::DeletedAtDesc => {
            if idx == 0 {
              q = q.order_by(users::deleted_at.desc());
            } else {
              q = q.then_order_by(users::deleted_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(User::as_select()).load::<User>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<UserOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}

pub struct UserCursor {
  pub created_at: time::OffsetDateTime,
  pub id: TestType,
}

impl From<User> for UserCursor {
  fn from(value: User) -> Self {
    Self {
      id: value.id,
      created_at: value.created_at,
    }
  }
}
impl User {
  pub fn paginate_by_user_cursor<'a, Conn>(
    after: Option<&'a UserCursor>,
    before: Option<&'a UserCursor>,
    limit: Option<usize>,
    offset: Option<usize>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::paginate_by_user_cursor_extend(
      after,
      before,
      limit,
      offset,
      |q| q,
      conn,
    )
  }
  pub fn paginate_by_user_cursor_extend<'a, F, Conn>(
    after: Option<&'a UserCursor>,
    before: Option<&'a UserCursor>,
    limit: Option<usize>,
    offset: Option<usize>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      users::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> users::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::IntoSql;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let create_query = || {
      let mut q = users::table
        .order_by(users::created_at.desc())
        .then_order_by(users::id.asc())
        .filter(users::deleted_at.is_null())
        .into_boxed();

      if let Some(cursor) = after {
        q = q.filter(
          (users::created_at, users::id)
            .into_sql::<diesel::sql_types::Record<_>>()
            .gt(
              (cursor.created_at, cursor.id)
                .into_sql::<diesel::sql_types::Record<(
                  diesel::sql_types::Timestamptz,
                  diesel::sql_types::Uuid,
                )>>(),
            ),
        );
      }

      if let Some(cursor) = before {
        q = q.filter(
          (users::created_at, users::id)
            .into_sql::<diesel::sql_types::Record<_>>()
            .lt(
              (cursor.created_at, cursor.id)
                .into_sql::<diesel::sql_types::Record<(
                  diesel::sql_types::Timestamptz,
                  diesel::sql_types::Uuid,
                )>>(),
            ),
        );
      }

      extend(q)
    };

    let mut q = create_query();

    //let mut has_last = false;

    if let Some(offset) = offset {
      q = q.offset(offset.try_into().unwrap());
    }

    if let Some(limit) = limit {
      q = q.limit(limit.try_into().unwrap());
    }

    q.select(User::as_select()).load::<User>(conn)
  }
  pub fn has_next_user_cursor<'a, Conn>(
    cursor: &'a UserCursor,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::has_next_user_cursor_extend(cursor, |q| q, conn)
  }

  pub fn has_next_user_cursor_extend<'a, F, Conn>(
    cursor: &'a UserCursor,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      users::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> users::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::IntoSql;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;

    let q = extend(
      users::table
        .filter(
          (users::created_at, users::id)
            .into_sql::<diesel::sql_types::Record<_>>()
            .gt(
              (cursor.created_at, cursor.id)
                .into_sql::<diesel::sql_types::Record<(
                  diesel::sql_types::Timestamptz,
                  diesel::sql_types::Uuid,
                )>>(),
            ),
        )
        .into_boxed(),
    );

    diesel::select(diesel::dsl::exists(q)).get_result(conn)
  }

  pub fn has_previous_user_cursor<'a, Conn>(
    cursor: &'a UserCursor,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::has_previous_user_cursor_extend(cursor, |q| q, conn)
  }

  pub fn has_previous_user_cursor_extend<'a, F, Conn>(
    cursor: &'a UserCursor,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      users::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> users::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::IntoSql;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;

    let q = extend(
      users::table
        .filter(
          (users::created_at, users::id)
            .into_sql::<diesel::sql_types::Record<_>>()
            .lt(
              (cursor.created_at, cursor.id)
                .into_sql::<diesel::sql_types::Record<(
                  diesel::sql_types::Timestamptz,
                  diesel::sql_types::Uuid,
                )>>(),
            ),
        )
        .into_boxed(),
    );

    diesel::select(diesel::dsl::exists(q)).get_result(conn)
  }
}

impl User {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      users::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> users::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      users::table
        .count()
        .filter(users::deleted_at.is_null())
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::count_extend(|q| q, conn)
  }
}
