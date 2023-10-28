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

impl PrismaMigration {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a PrismaMigrationUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_checksum<'a, Conn>(
    id: &'a TestType,
    checksum: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_finished_at<'a, Conn>(
    id: &'a TestType,
    finished_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_migration_name<'a, Conn>(
    id: &'a TestType,
    migration_name: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_logs<'a, Conn>(
    id: &'a TestType,
    logs: Option<&'a str>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_rolled_back_at<'a, Conn>(
    id: &'a TestType,
    rolled_back_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_started_at<'a, Conn>(
    id: &'a TestType,
    started_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
  pub fn update_applied_steps_count<'a, Conn>(
    id: &'a TestType,
    applied_steps_count: i32,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PrismaMigration>(conn)
  }
}

impl PrismaMigration {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(_prisma_migrations::table)
      .filter(_prisma_migrations::id.eq(id))
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(conn)
  }
}

impl PrismaMigration {
  pub fn insert<'a, Conn>(
    data: &'a NewPrismaMigration<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(_prisma_migrations::table)
      .values(data)
      .returning(PrismaMigration::as_returning())
      .get_result::<PrismaMigration>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewPrismaMigration<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PrismaMigration>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(_prisma_migrations::table)
      .values(data)
      .returning(PrismaMigration::as_returning())
      .get_results::<PrismaMigration>(conn)
  }
}

impl PrismaMigration {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      _prisma_migrations::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> _prisma_migrations::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      _prisma_migrations::table
        .filter(_prisma_migrations::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PrismaMigration, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PrismaMigration::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PrismaMigration>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      _prisma_migrations::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> _prisma_migrations::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(_prisma_migrations::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PrismaMigration>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PrismaMigration::get_many_extend(|q| q, conn)
  }
}
pub enum PrismaMigrationOrderBy {
  IdAsc,
  IdDesc,
  ChecksumAsc,
  ChecksumDesc,
  FinishedAtAsc,
  FinishedAtDesc,
  MigrationNameAsc,
  MigrationNameDesc,
  LogsAsc,
  LogsDesc,
  RolledBackAtAsc,
  RolledBackAtDesc,
  StartedAtAsc,
  StartedAtDesc,
  AppliedStepsCountAsc,
  AppliedStepsCountDesc,
}
impl PrismaMigration {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<PrismaMigrationOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PrismaMigration>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      _prisma_migrations::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> _prisma_migrations::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = _prisma_migrations::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          PrismaMigrationOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::id.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::id.asc());
            }
          }

          PrismaMigrationOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::id.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::id.desc());
            }
          }

          PrismaMigrationOrderBy::ChecksumAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::checksum.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::checksum.asc());
            }
          }

          PrismaMigrationOrderBy::ChecksumDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::checksum.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::checksum.desc());
            }
          }

          PrismaMigrationOrderBy::FinishedAtAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::finished_at.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::finished_at.asc());
            }
          }

          PrismaMigrationOrderBy::FinishedAtDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::finished_at.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::finished_at.desc());
            }
          }

          PrismaMigrationOrderBy::MigrationNameAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::migration_name.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::migration_name.asc());
            }
          }

          PrismaMigrationOrderBy::MigrationNameDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::migration_name.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::migration_name.desc());
            }
          }

          PrismaMigrationOrderBy::LogsAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::logs.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::logs.asc());
            }
          }

          PrismaMigrationOrderBy::LogsDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::logs.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::logs.desc());
            }
          }

          PrismaMigrationOrderBy::RolledBackAtAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::rolled_back_at.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::rolled_back_at.asc());
            }
          }

          PrismaMigrationOrderBy::RolledBackAtDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::rolled_back_at.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::rolled_back_at.desc());
            }
          }

          PrismaMigrationOrderBy::StartedAtAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::started_at.asc());
            } else {
              q = q.then_order_by(_prisma_migrations::started_at.asc());
            }
          }

          PrismaMigrationOrderBy::StartedAtDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::started_at.desc());
            } else {
              q = q.then_order_by(_prisma_migrations::started_at.desc());
            }
          }

          PrismaMigrationOrderBy::AppliedStepsCountAsc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::applied_steps_count.asc());
            } else {
              q =
                q.then_order_by(_prisma_migrations::applied_steps_count.asc());
            }
          }

          PrismaMigrationOrderBy::AppliedStepsCountDesc => {
            if idx == 0 {
              q = q.order_by(_prisma_migrations::applied_steps_count.desc());
            } else {
              q =
                q.then_order_by(_prisma_migrations::applied_steps_count.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(PrismaMigration::as_select())
      .load::<PrismaMigration>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<PrismaMigrationOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PrismaMigration>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PrismaMigration::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}
impl PrismaMigration {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      _prisma_migrations::BoxedQuery<
        'b,
        diesel::pg::Pg,
        diesel::sql_types::BigInt,
      >,
    ) -> _prisma_migrations::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(_prisma_migrations::table.count().into_boxed()).first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PrismaMigration::count_extend(|q| q, conn)
  }
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

impl Credential {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a CredentialUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
  pub fn update_identity_id<'a, Conn>(
    id: &'a TestType,
    identity_id: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
  pub fn update_enabled<'a, Conn>(
    id: &'a TestType,
    enabled: bool,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
  pub fn update_type<'a, Conn>(
    id: &'a TestType,
    type_: CredentialType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
  pub fn update_content<'a, Conn>(
    id: &'a TestType,
    content: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
  pub fn update_updated_at<'a, Conn>(
    id: &'a TestType,
    updated_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Credential>(conn)
  }
}

impl Credential {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(credentials::table)
      .filter(credentials::id.eq(id))
      .returning(Credential::as_returning())
      .get_result::<Credential>(conn)
  }
}

impl Credential {
  pub fn insert<'a, Conn>(
    data: &'a NewCredential<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(credentials::table)
      .values(data)
      .returning(Credential::as_returning())
      .get_result::<Credential>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewCredential<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Credential>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(credentials::table)
      .values(data)
      .returning(Credential::as_returning())
      .get_results::<Credential>(conn)
  }
}

impl Credential {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      credentials::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> credentials::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      credentials::table
        .filter(credentials::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Credential, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Credential::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Credential>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      credentials::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> credentials::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(credentials::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Credential>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Credential::get_many_extend(|q| q, conn)
  }
}
pub enum CredentialOrderBy {
  IdAsc,
  IdDesc,
  IdentityIdAsc,
  IdentityIdDesc,
  EnabledAsc,
  EnabledDesc,
  TypeAsc,
  TypeDesc,
  ContentAsc,
  ContentDesc,
  CreatedAtAsc,
  CreatedAtDesc,
  UpdatedAtAsc,
  UpdatedAtDesc,
}
impl Credential {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<CredentialOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Credential>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      credentials::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> credentials::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = credentials::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          CredentialOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(credentials::id.asc());
            } else {
              q = q.then_order_by(credentials::id.asc());
            }
          }

          CredentialOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(credentials::id.desc());
            } else {
              q = q.then_order_by(credentials::id.desc());
            }
          }

          CredentialOrderBy::IdentityIdAsc => {
            if idx == 0 {
              q = q.order_by(credentials::identity_id.asc());
            } else {
              q = q.then_order_by(credentials::identity_id.asc());
            }
          }

          CredentialOrderBy::IdentityIdDesc => {
            if idx == 0 {
              q = q.order_by(credentials::identity_id.desc());
            } else {
              q = q.then_order_by(credentials::identity_id.desc());
            }
          }

          CredentialOrderBy::EnabledAsc => {
            if idx == 0 {
              q = q.order_by(credentials::enabled.asc());
            } else {
              q = q.then_order_by(credentials::enabled.asc());
            }
          }

          CredentialOrderBy::EnabledDesc => {
            if idx == 0 {
              q = q.order_by(credentials::enabled.desc());
            } else {
              q = q.then_order_by(credentials::enabled.desc());
            }
          }

          CredentialOrderBy::TypeAsc => {
            if idx == 0 {
              q = q.order_by(credentials::type_.asc());
            } else {
              q = q.then_order_by(credentials::type_.asc());
            }
          }

          CredentialOrderBy::TypeDesc => {
            if idx == 0 {
              q = q.order_by(credentials::type_.desc());
            } else {
              q = q.then_order_by(credentials::type_.desc());
            }
          }

          CredentialOrderBy::ContentAsc => {
            if idx == 0 {
              q = q.order_by(credentials::content.asc());
            } else {
              q = q.then_order_by(credentials::content.asc());
            }
          }

          CredentialOrderBy::ContentDesc => {
            if idx == 0 {
              q = q.order_by(credentials::content.desc());
            } else {
              q = q.then_order_by(credentials::content.desc());
            }
          }

          CredentialOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(credentials::created_at.asc());
            } else {
              q = q.then_order_by(credentials::created_at.asc());
            }
          }

          CredentialOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(credentials::created_at.desc());
            } else {
              q = q.then_order_by(credentials::created_at.desc());
            }
          }

          CredentialOrderBy::UpdatedAtAsc => {
            if idx == 0 {
              q = q.order_by(credentials::updated_at.asc());
            } else {
              q = q.then_order_by(credentials::updated_at.asc());
            }
          }

          CredentialOrderBy::UpdatedAtDesc => {
            if idx == 0 {
              q = q.order_by(credentials::updated_at.desc());
            } else {
              q = q.then_order_by(credentials::updated_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(Credential::as_select()).load::<Credential>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<CredentialOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Credential>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Credential::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}
impl Credential {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      credentials::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> credentials::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(credentials::table.count().into_boxed()).first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Credential::count_extend(|q| q, conn)
  }
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

impl EmailAddressVerificationCode {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a EmailAddressVerificationCodeUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<EmailAddressVerificationCode>(conn)
  }
  pub fn update_value<'a, Conn>(
    id: &'a TestType,
    value: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<EmailAddressVerificationCode>(conn)
  }
  pub fn update_expires_at<'a, Conn>(
    id: &'a TestType,
    expires_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<EmailAddressVerificationCode>(conn)
  }
  pub fn update_email_address_id<'a, Conn>(
    id: &'a TestType,
    email_address_id: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<EmailAddressVerificationCode>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<EmailAddressVerificationCode>(conn)
  }
  pub fn update_invalidated_at<'a, Conn>(
    id: &'a TestType,
    invalidated_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<EmailAddressVerificationCode>(conn)
  }
}

impl EmailAddressVerificationCode {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(email_address_verification_codes::table)
      .filter(email_address_verification_codes::id.eq(id))
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(conn)
  }
}

impl EmailAddressVerificationCode {
  pub fn insert<'a, Conn>(
    data: &'a NewEmailAddressVerificationCode<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_address_verification_codes::table)
      .values(data)
      .returning(EmailAddressVerificationCode::as_returning())
      .get_result::<EmailAddressVerificationCode>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewEmailAddressVerificationCode<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddressVerificationCode>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_address_verification_codes::table)
      .values(data)
      .returning(EmailAddressVerificationCode::as_returning())
      .get_results::<EmailAddressVerificationCode>(conn)
  }
}

impl EmailAddressVerificationCode {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_address_verification_codes::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> email_address_verification_codes::BoxedQuery<
      'b,
      diesel::pg::Pg,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      email_address_verification_codes::table
        .filter(email_address_verification_codes::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<EmailAddressVerificationCode, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddressVerificationCode::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddressVerificationCode>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_address_verification_codes::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> email_address_verification_codes::BoxedQuery<
      'b,
      diesel::pg::Pg,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(email_address_verification_codes::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddressVerificationCode>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddressVerificationCode::get_many_extend(|q| q, conn)
  }
}
pub enum EmailAddressVerificationCodeOrderBy {
  IdAsc,
  IdDesc,
  ValueAsc,
  ValueDesc,
  ExpiresAtAsc,
  ExpiresAtDesc,
  EmailAddressIdAsc,
  EmailAddressIdDesc,
  CreatedAtAsc,
  CreatedAtDesc,
  InvalidatedAtAsc,
  InvalidatedAtDesc,
}
impl EmailAddressVerificationCode {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<EmailAddressVerificationCodeOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddressVerificationCode>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_address_verification_codes::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> email_address_verification_codes::BoxedQuery<
      'b,
      diesel::pg::Pg,
    >,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = email_address_verification_codes::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          EmailAddressVerificationCodeOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(email_address_verification_codes::id.asc());
            } else {
              q = q.then_order_by(email_address_verification_codes::id.asc());
            }
          }

          EmailAddressVerificationCodeOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(email_address_verification_codes::id.desc());
            } else {
              q = q.then_order_by(email_address_verification_codes::id.desc());
            }
          }

          EmailAddressVerificationCodeOrderBy::ValueAsc => {
            if idx == 0 {
              q = q.order_by(email_address_verification_codes::value.asc());
            } else {
              q =
                q.then_order_by(email_address_verification_codes::value.asc());
            }
          }

          EmailAddressVerificationCodeOrderBy::ValueDesc => {
            if idx == 0 {
              q = q.order_by(email_address_verification_codes::value.desc());
            } else {
              q =
                q.then_order_by(email_address_verification_codes::value.desc());
            }
          }

          EmailAddressVerificationCodeOrderBy::ExpiresAtAsc => {
            if idx == 0 {
              q =
                q.order_by(email_address_verification_codes::expires_at.asc());
            } else {
              q = q.then_order_by(
                email_address_verification_codes::expires_at.asc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::ExpiresAtDesc => {
            if idx == 0 {
              q =
                q.order_by(email_address_verification_codes::expires_at.desc());
            } else {
              q = q.then_order_by(
                email_address_verification_codes::expires_at.desc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::EmailAddressIdAsc => {
            if idx == 0 {
              q = q.order_by(
                email_address_verification_codes::email_address_id.asc(),
              );
            } else {
              q = q.then_order_by(
                email_address_verification_codes::email_address_id.asc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::EmailAddressIdDesc => {
            if idx == 0 {
              q = q.order_by(
                email_address_verification_codes::email_address_id.desc(),
              );
            } else {
              q = q.then_order_by(
                email_address_verification_codes::email_address_id.desc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q =
                q.order_by(email_address_verification_codes::created_at.asc());
            } else {
              q = q.then_order_by(
                email_address_verification_codes::created_at.asc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q =
                q.order_by(email_address_verification_codes::created_at.desc());
            } else {
              q = q.then_order_by(
                email_address_verification_codes::created_at.desc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::InvalidatedAtAsc => {
            if idx == 0 {
              q = q.order_by(
                email_address_verification_codes::invalidated_at.asc(),
              );
            } else {
              q = q.then_order_by(
                email_address_verification_codes::invalidated_at.asc(),
              );
            }
          }

          EmailAddressVerificationCodeOrderBy::InvalidatedAtDesc => {
            if idx == 0 {
              q = q.order_by(
                email_address_verification_codes::invalidated_at.desc(),
              );
            } else {
              q = q.then_order_by(
                email_address_verification_codes::invalidated_at.desc(),
              );
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(EmailAddressVerificationCode::as_select())
      .load::<EmailAddressVerificationCode>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<EmailAddressVerificationCodeOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddressVerificationCode>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddressVerificationCode::paginate_extend(
      offset,
      limit,
      ordering,
      |q| q,
      conn,
    )
  }
}
impl EmailAddressVerificationCode {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_address_verification_codes::BoxedQuery<
        'b,
        diesel::pg::Pg,
        diesel::sql_types::BigInt,
      >,
    ) -> email_address_verification_codes::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(email_address_verification_codes::table.count().into_boxed())
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
    EmailAddressVerificationCode::count_extend(|q| q, conn)
  }
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

impl EmailAddress {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a EmailAddressUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
  pub fn update_identity_id<'a, Conn>(
    id: &'a TestType,
    identity_id: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
  pub fn update_value<'a, Conn>(
    id: &'a TestType,
    value: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
  pub fn update_primary<'a, Conn>(
    id: &'a TestType,
    primary: bool,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
  pub fn update_verified_at<'a, Conn>(
    id: &'a TestType,
    verified_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
  pub fn update_updated_at<'a, Conn>(
    id: &'a TestType,
    updated_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<EmailAddress>(conn)
  }
}

impl EmailAddress {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(email_addresses::table)
      .filter(email_addresses::id.eq(id))
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(conn)
  }
}

impl EmailAddress {
  pub fn insert<'a, Conn>(
    data: &'a NewEmailAddress<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_addresses::table)
      .values(data)
      .returning(EmailAddress::as_returning())
      .get_result::<EmailAddress>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewEmailAddress<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddress>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(email_addresses::table)
      .values(data)
      .returning(EmailAddress::as_returning())
      .get_results::<EmailAddress>(conn)
  }
}

impl EmailAddress {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_addresses::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> email_addresses::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      email_addresses::table
        .filter(email_addresses::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<EmailAddress, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddress::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddress>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_addresses::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> email_addresses::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(email_addresses::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddress>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddress::get_many_extend(|q| q, conn)
  }
}
pub enum EmailAddressOrderBy {
  IdAsc,
  IdDesc,
  IdentityIdAsc,
  IdentityIdDesc,
  ValueAsc,
  ValueDesc,
  PrimaryAsc,
  PrimaryDesc,
  VerifiedAtAsc,
  VerifiedAtDesc,
  CreatedAtAsc,
  CreatedAtDesc,
  UpdatedAtAsc,
  UpdatedAtDesc,
}
impl EmailAddress {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<EmailAddressOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddress>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_addresses::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> email_addresses::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = email_addresses::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          EmailAddressOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::id.asc());
            } else {
              q = q.then_order_by(email_addresses::id.asc());
            }
          }

          EmailAddressOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::id.desc());
            } else {
              q = q.then_order_by(email_addresses::id.desc());
            }
          }

          EmailAddressOrderBy::IdentityIdAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::identity_id.asc());
            } else {
              q = q.then_order_by(email_addresses::identity_id.asc());
            }
          }

          EmailAddressOrderBy::IdentityIdDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::identity_id.desc());
            } else {
              q = q.then_order_by(email_addresses::identity_id.desc());
            }
          }

          EmailAddressOrderBy::ValueAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::value.asc());
            } else {
              q = q.then_order_by(email_addresses::value.asc());
            }
          }

          EmailAddressOrderBy::ValueDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::value.desc());
            } else {
              q = q.then_order_by(email_addresses::value.desc());
            }
          }

          EmailAddressOrderBy::PrimaryAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::primary.asc());
            } else {
              q = q.then_order_by(email_addresses::primary.asc());
            }
          }

          EmailAddressOrderBy::PrimaryDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::primary.desc());
            } else {
              q = q.then_order_by(email_addresses::primary.desc());
            }
          }

          EmailAddressOrderBy::VerifiedAtAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::verified_at.asc());
            } else {
              q = q.then_order_by(email_addresses::verified_at.asc());
            }
          }

          EmailAddressOrderBy::VerifiedAtDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::verified_at.desc());
            } else {
              q = q.then_order_by(email_addresses::verified_at.desc());
            }
          }

          EmailAddressOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::created_at.asc());
            } else {
              q = q.then_order_by(email_addresses::created_at.asc());
            }
          }

          EmailAddressOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::created_at.desc());
            } else {
              q = q.then_order_by(email_addresses::created_at.desc());
            }
          }

          EmailAddressOrderBy::UpdatedAtAsc => {
            if idx == 0 {
              q = q.order_by(email_addresses::updated_at.asc());
            } else {
              q = q.then_order_by(email_addresses::updated_at.asc());
            }
          }

          EmailAddressOrderBy::UpdatedAtDesc => {
            if idx == 0 {
              q = q.order_by(email_addresses::updated_at.desc());
            } else {
              q = q.then_order_by(email_addresses::updated_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(EmailAddress::as_select())
      .load::<EmailAddress>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<EmailAddressOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<EmailAddress>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddress::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}
impl EmailAddress {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      email_addresses::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> email_addresses::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(email_addresses::table.count().into_boxed()).first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    EmailAddress::count_extend(|q| q, conn)
  }
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

impl Identity {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a IdentityUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_username<'a, Conn>(
    id: &'a TestType,
    username: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_display_picture_url<'a, Conn>(
    id: &'a TestType,
    display_picture_url: Option<&'a str>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_first_name<'a, Conn>(
    id: &'a TestType,
    first_name: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_middle_name<'a, Conn>(
    id: &'a TestType,
    middle_name: Option<&'a str>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_last_name<'a, Conn>(
    id: &'a TestType,
    last_name: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_name_suffix<'a, Conn>(
    id: &'a TestType,
    name_suffix: Option<&'a str>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_gender<'a, Conn>(
    id: &'a TestType,
    gender: Option<Gender>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
  pub fn update_other_gender<'a, Conn>(
    id: &'a TestType,
    other_gender: Option<&'a str>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Identity>(conn)
  }
}

impl Identity {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(identities::table)
      .filter(identities::id.eq(id))
      .returning(Identity::as_returning())
      .get_result::<Identity>(conn)
  }
}

impl Identity {
  pub fn insert<'a, Conn>(
    data: &'a NewIdentity<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(identities::table)
      .values(data)
      .returning(Identity::as_returning())
      .get_result::<Identity>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewIdentity<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Identity>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(identities::table)
      .values(data)
      .returning(Identity::as_returning())
      .get_results::<Identity>(conn)
  }
}

impl Identity {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      identities::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> identities::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(identities::table.filter(identities::id.eq(id)).into_boxed())
      .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Identity, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Identity::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Identity>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      identities::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> identities::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(identities::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Identity>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Identity::get_many_extend(|q| q, conn)
  }
}
pub enum IdentityOrderBy {
  IdAsc,
  IdDesc,
  UsernameAsc,
  UsernameDesc,
  DisplayPictureUrlAsc,
  DisplayPictureUrlDesc,
  FirstNameAsc,
  FirstNameDesc,
  MiddleNameAsc,
  MiddleNameDesc,
  LastNameAsc,
  LastNameDesc,
  NameSuffixAsc,
  NameSuffixDesc,
  GenderAsc,
  GenderDesc,
  OtherGenderAsc,
  OtherGenderDesc,
}
impl Identity {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<IdentityOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Identity>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      identities::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> identities::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = identities::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          IdentityOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(identities::id.asc());
            } else {
              q = q.then_order_by(identities::id.asc());
            }
          }

          IdentityOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(identities::id.desc());
            } else {
              q = q.then_order_by(identities::id.desc());
            }
          }

          IdentityOrderBy::UsernameAsc => {
            if idx == 0 {
              q = q.order_by(identities::username.asc());
            } else {
              q = q.then_order_by(identities::username.asc());
            }
          }

          IdentityOrderBy::UsernameDesc => {
            if idx == 0 {
              q = q.order_by(identities::username.desc());
            } else {
              q = q.then_order_by(identities::username.desc());
            }
          }

          IdentityOrderBy::DisplayPictureUrlAsc => {
            if idx == 0 {
              q = q.order_by(identities::display_picture_url.asc());
            } else {
              q = q.then_order_by(identities::display_picture_url.asc());
            }
          }

          IdentityOrderBy::DisplayPictureUrlDesc => {
            if idx == 0 {
              q = q.order_by(identities::display_picture_url.desc());
            } else {
              q = q.then_order_by(identities::display_picture_url.desc());
            }
          }

          IdentityOrderBy::FirstNameAsc => {
            if idx == 0 {
              q = q.order_by(identities::first_name.asc());
            } else {
              q = q.then_order_by(identities::first_name.asc());
            }
          }

          IdentityOrderBy::FirstNameDesc => {
            if idx == 0 {
              q = q.order_by(identities::first_name.desc());
            } else {
              q = q.then_order_by(identities::first_name.desc());
            }
          }

          IdentityOrderBy::MiddleNameAsc => {
            if idx == 0 {
              q = q.order_by(identities::middle_name.asc());
            } else {
              q = q.then_order_by(identities::middle_name.asc());
            }
          }

          IdentityOrderBy::MiddleNameDesc => {
            if idx == 0 {
              q = q.order_by(identities::middle_name.desc());
            } else {
              q = q.then_order_by(identities::middle_name.desc());
            }
          }

          IdentityOrderBy::LastNameAsc => {
            if idx == 0 {
              q = q.order_by(identities::last_name.asc());
            } else {
              q = q.then_order_by(identities::last_name.asc());
            }
          }

          IdentityOrderBy::LastNameDesc => {
            if idx == 0 {
              q = q.order_by(identities::last_name.desc());
            } else {
              q = q.then_order_by(identities::last_name.desc());
            }
          }

          IdentityOrderBy::NameSuffixAsc => {
            if idx == 0 {
              q = q.order_by(identities::name_suffix.asc());
            } else {
              q = q.then_order_by(identities::name_suffix.asc());
            }
          }

          IdentityOrderBy::NameSuffixDesc => {
            if idx == 0 {
              q = q.order_by(identities::name_suffix.desc());
            } else {
              q = q.then_order_by(identities::name_suffix.desc());
            }
          }

          IdentityOrderBy::GenderAsc => {
            if idx == 0 {
              q = q.order_by(identities::gender.asc());
            } else {
              q = q.then_order_by(identities::gender.asc());
            }
          }

          IdentityOrderBy::GenderDesc => {
            if idx == 0 {
              q = q.order_by(identities::gender.desc());
            } else {
              q = q.then_order_by(identities::gender.desc());
            }
          }

          IdentityOrderBy::OtherGenderAsc => {
            if idx == 0 {
              q = q.order_by(identities::other_gender.asc());
            } else {
              q = q.then_order_by(identities::other_gender.asc());
            }
          }

          IdentityOrderBy::OtherGenderDesc => {
            if idx == 0 {
              q = q.order_by(identities::other_gender.desc());
            } else {
              q = q.then_order_by(identities::other_gender.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(Identity::as_select()).load::<Identity>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<IdentityOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<Identity>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Identity::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}
impl Identity {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      identities::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> identities::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(identities::table.count().into_boxed()).first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Identity::count_extend(|q| q, conn)
  }
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

impl PasswordResetToken {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a PasswordResetTokenUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PasswordResetToken>(conn)
  }
  pub fn update_value<'a, Conn>(
    id: &'a TestType,
    value: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PasswordResetToken>(conn)
  }
  pub fn update_expires_at<'a, Conn>(
    id: &'a TestType,
    expires_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PasswordResetToken>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
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
      .get_result::<PasswordResetToken>(conn)
  }
}

impl PasswordResetToken {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(password_reset_tokens::table)
      .filter(password_reset_tokens::id.eq(id))
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(conn)
  }
}

impl PasswordResetToken {
  pub fn insert<'a, Conn>(
    data: &'a NewPasswordResetToken<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(password_reset_tokens::table)
      .values(data)
      .returning(PasswordResetToken::as_returning())
      .get_result::<PasswordResetToken>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewPasswordResetToken<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PasswordResetToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(password_reset_tokens::table)
      .values(data)
      .returning(PasswordResetToken::as_returning())
      .get_results::<PasswordResetToken>(conn)
  }
}

impl PasswordResetToken {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      password_reset_tokens::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> password_reset_tokens::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      password_reset_tokens::table
        .filter(password_reset_tokens::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<PasswordResetToken, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PasswordResetToken::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PasswordResetToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      password_reset_tokens::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> password_reset_tokens::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(password_reset_tokens::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PasswordResetToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PasswordResetToken::get_many_extend(|q| q, conn)
  }
}
pub enum PasswordResetTokenOrderBy {
  IdAsc,
  IdDesc,
  ValueAsc,
  ValueDesc,
  ExpiresAtAsc,
  ExpiresAtDesc,
  CreatedAtAsc,
  CreatedAtDesc,
}
impl PasswordResetToken {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<PasswordResetTokenOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PasswordResetToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      password_reset_tokens::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> password_reset_tokens::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = password_reset_tokens::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          PasswordResetTokenOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::id.asc());
            } else {
              q = q.then_order_by(password_reset_tokens::id.asc());
            }
          }

          PasswordResetTokenOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::id.desc());
            } else {
              q = q.then_order_by(password_reset_tokens::id.desc());
            }
          }

          PasswordResetTokenOrderBy::ValueAsc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::value.asc());
            } else {
              q = q.then_order_by(password_reset_tokens::value.asc());
            }
          }

          PasswordResetTokenOrderBy::ValueDesc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::value.desc());
            } else {
              q = q.then_order_by(password_reset_tokens::value.desc());
            }
          }

          PasswordResetTokenOrderBy::ExpiresAtAsc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::expires_at.asc());
            } else {
              q = q.then_order_by(password_reset_tokens::expires_at.asc());
            }
          }

          PasswordResetTokenOrderBy::ExpiresAtDesc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::expires_at.desc());
            } else {
              q = q.then_order_by(password_reset_tokens::expires_at.desc());
            }
          }

          PasswordResetTokenOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::created_at.asc());
            } else {
              q = q.then_order_by(password_reset_tokens::created_at.asc());
            }
          }

          PasswordResetTokenOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(password_reset_tokens::created_at.desc());
            } else {
              q = q.then_order_by(password_reset_tokens::created_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(PasswordResetToken::as_select())
      .load::<PasswordResetToken>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<PasswordResetTokenOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<PasswordResetToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PasswordResetToken::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}
impl PasswordResetToken {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      password_reset_tokens::BoxedQuery<
        'b,
        diesel::pg::Pg,
        diesel::sql_types::BigInt,
      >,
    ) -> password_reset_tokens::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(password_reset_tokens::table.count().into_boxed()).first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    PasswordResetToken::count_extend(|q| q, conn)
  }
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

impl RefreshToken {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a RefreshTokenUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_identity_id<'a, Conn>(
    id: &'a TestType,
    identity_id: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_value<'a, Conn>(
    id: &'a TestType,
    value: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_scope<'a, Conn>(
    id: &'a TestType,
    scope: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_audience<'a, Conn>(
    id: &'a TestType,
    audience: &'a str,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_subject<'a, Conn>(
    id: &'a TestType,
    subject: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_subject_type<'a, Conn>(
    id: &'a TestType,
    subject_type: SubjectType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_expires_at<'a, Conn>(
    id: &'a TestType,
    expires_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
  pub fn update_invalidated_at<'a, Conn>(
    id: &'a TestType,
    invalidated_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<RefreshToken>(conn)
  }
}

impl RefreshToken {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(refresh_tokens::table)
      .filter(refresh_tokens::id.eq(id))
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(conn)
  }
}

impl RefreshToken {
  pub fn insert<'a, Conn>(
    data: &'a NewRefreshToken<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(refresh_tokens::table)
      .values(data)
      .returning(RefreshToken::as_returning())
      .get_result::<RefreshToken>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewRefreshToken<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<RefreshToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(refresh_tokens::table)
      .values(data)
      .returning(RefreshToken::as_returning())
      .get_results::<RefreshToken>(conn)
  }
}

impl RefreshToken {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      refresh_tokens::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> refresh_tokens::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      refresh_tokens::table
        .filter(refresh_tokens::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<RefreshToken, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    RefreshToken::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<RefreshToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      refresh_tokens::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> refresh_tokens::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(refresh_tokens::table.into_boxed()).load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<RefreshToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    RefreshToken::get_many_extend(|q| q, conn)
  }
}
pub enum RefreshTokenOrderBy {
  IdAsc,
  IdDesc,
  IdentityIdAsc,
  IdentityIdDesc,
  ValueAsc,
  ValueDesc,
  ScopeAsc,
  ScopeDesc,
  AudienceAsc,
  AudienceDesc,
  SubjectAsc,
  SubjectDesc,
  SubjectTypeAsc,
  SubjectTypeDesc,
  ExpiresAtAsc,
  ExpiresAtDesc,
  CreatedAtAsc,
  CreatedAtDesc,
  InvalidatedAtAsc,
  InvalidatedAtDesc,
}
impl RefreshToken {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<RefreshTokenOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<RefreshToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      refresh_tokens::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> refresh_tokens::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = refresh_tokens::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          RefreshTokenOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::id.asc());
            } else {
              q = q.then_order_by(refresh_tokens::id.asc());
            }
          }

          RefreshTokenOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::id.desc());
            } else {
              q = q.then_order_by(refresh_tokens::id.desc());
            }
          }

          RefreshTokenOrderBy::IdentityIdAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::identity_id.asc());
            } else {
              q = q.then_order_by(refresh_tokens::identity_id.asc());
            }
          }

          RefreshTokenOrderBy::IdentityIdDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::identity_id.desc());
            } else {
              q = q.then_order_by(refresh_tokens::identity_id.desc());
            }
          }

          RefreshTokenOrderBy::ValueAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::value.asc());
            } else {
              q = q.then_order_by(refresh_tokens::value.asc());
            }
          }

          RefreshTokenOrderBy::ValueDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::value.desc());
            } else {
              q = q.then_order_by(refresh_tokens::value.desc());
            }
          }

          RefreshTokenOrderBy::ScopeAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::scope.asc());
            } else {
              q = q.then_order_by(refresh_tokens::scope.asc());
            }
          }

          RefreshTokenOrderBy::ScopeDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::scope.desc());
            } else {
              q = q.then_order_by(refresh_tokens::scope.desc());
            }
          }

          RefreshTokenOrderBy::AudienceAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::audience.asc());
            } else {
              q = q.then_order_by(refresh_tokens::audience.asc());
            }
          }

          RefreshTokenOrderBy::AudienceDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::audience.desc());
            } else {
              q = q.then_order_by(refresh_tokens::audience.desc());
            }
          }

          RefreshTokenOrderBy::SubjectAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::subject.asc());
            } else {
              q = q.then_order_by(refresh_tokens::subject.asc());
            }
          }

          RefreshTokenOrderBy::SubjectDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::subject.desc());
            } else {
              q = q.then_order_by(refresh_tokens::subject.desc());
            }
          }

          RefreshTokenOrderBy::SubjectTypeAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::subject_type.asc());
            } else {
              q = q.then_order_by(refresh_tokens::subject_type.asc());
            }
          }

          RefreshTokenOrderBy::SubjectTypeDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::subject_type.desc());
            } else {
              q = q.then_order_by(refresh_tokens::subject_type.desc());
            }
          }

          RefreshTokenOrderBy::ExpiresAtAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::expires_at.asc());
            } else {
              q = q.then_order_by(refresh_tokens::expires_at.asc());
            }
          }

          RefreshTokenOrderBy::ExpiresAtDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::expires_at.desc());
            } else {
              q = q.then_order_by(refresh_tokens::expires_at.desc());
            }
          }

          RefreshTokenOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::created_at.asc());
            } else {
              q = q.then_order_by(refresh_tokens::created_at.asc());
            }
          }

          RefreshTokenOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::created_at.desc());
            } else {
              q = q.then_order_by(refresh_tokens::created_at.desc());
            }
          }

          RefreshTokenOrderBy::InvalidatedAtAsc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::invalidated_at.asc());
            } else {
              q = q.then_order_by(refresh_tokens::invalidated_at.asc());
            }
          }

          RefreshTokenOrderBy::InvalidatedAtDesc => {
            if idx == 0 {
              q = q.order_by(refresh_tokens::invalidated_at.desc());
            } else {
              q = q.then_order_by(refresh_tokens::invalidated_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(RefreshToken::as_select())
      .load::<RefreshToken>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<RefreshTokenOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<
    Output = Result<Vec<RefreshToken>, diesel::result::Error>,
  > + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    RefreshToken::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
}
impl RefreshToken {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      refresh_tokens::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> refresh_tokens::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(refresh_tokens::table.count().into_boxed()).first(conn)
  }
  pub fn count<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    RefreshToken::count_extend(|q| q, conn)
  }
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

impl Staff {
  pub fn update<'a, Conn>(
    id: &'a TestType,
    changes: &'a StaffUpdate<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
  pub fn update_role<'a, Conn>(
    id: &'a TestType,
    role: StaffRole,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
  pub fn update_identity_id<'a, Conn>(
    id: &'a TestType,
    identity_id: &'a uuid::Uuid,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
  pub fn update_created_at<'a, Conn>(
    id: &'a TestType,
    created_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
  pub fn update_updated_at<'a, Conn>(
    id: &'a TestType,
    updated_at: &'a time::OffsetDateTime,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
  pub fn update_deleted_at<'a, Conn>(
    id: &'a TestType,
    deleted_at: Option<&'a time::OffsetDateTime>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
}

impl Staff {
  pub fn delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::ExpressionMethods;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    diesel::delete(staffs::table)
      .filter(staffs::id.eq(id))
      .returning(Staff::as_returning())
      .get_result::<Staff>(conn)
  }
}

impl Staff {
  pub fn soft_delete<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
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
      .get_result::<Staff>(conn)
  }
}

impl Staff {
  pub fn insert<'a, Conn>(
    data: &'a NewStaff<'a>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(staffs::table)
      .values(data)
      .returning(Staff::as_returning())
      .get_result::<Staff>(conn)
  }
  pub fn insert_many<'a, Conn>(
    data: &'a [NewStaff<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;

    diesel::insert_into(staffs::table)
      .values(data)
      .returning(Staff::as_returning())
      .get_results::<Staff>(conn)
  }
}

impl Staff {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> staffs::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      staffs::table
        .filter(staffs::deleted_at.is_null())
        .filter(staffs::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> staffs::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      staffs::table
        .filter(staffs::deleted_at.is_null())
        .into_boxed(),
    )
    .load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::get_many_extend(|q| q, conn)
  }
  pub fn get_with_soft_deleted_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> staffs::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(staffs::table.filter(staffs::id.eq(id)).into_boxed()).first(conn)
  }
  pub fn get_with_soft_deleted<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Staff, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::get_with_soft_deleted_extend(id, |q| q, conn)
  }
  pub fn get_many_with_soft_deleted_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> staffs::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(staffs::table.into_boxed()).load(conn)
  }
  pub fn get_many_with_soft_deleted<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::get_many_with_soft_deleted_extend(|q| q, conn)
  }
}
pub enum StaffOrderBy {
  IdAsc,
  IdDesc,
  RoleAsc,
  RoleDesc,
  IdentityIdAsc,
  IdentityIdDesc,
  CreatedAtAsc,
  CreatedAtDesc,
  UpdatedAtAsc,
  UpdatedAtDesc,
  DeletedAtAsc,
  DeletedAtDesc,
}
impl Staff {
  pub fn paginate_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<StaffOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> staffs::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = staffs::table
      .filter(staffs::deleted_at.is_null())
      .into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          StaffOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(staffs::id.asc());
            } else {
              q = q.then_order_by(staffs::id.asc());
            }
          }

          StaffOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(staffs::id.desc());
            } else {
              q = q.then_order_by(staffs::id.desc());
            }
          }

          StaffOrderBy::RoleAsc => {
            if idx == 0 {
              q = q.order_by(staffs::role.asc());
            } else {
              q = q.then_order_by(staffs::role.asc());
            }
          }

          StaffOrderBy::RoleDesc => {
            if idx == 0 {
              q = q.order_by(staffs::role.desc());
            } else {
              q = q.then_order_by(staffs::role.desc());
            }
          }

          StaffOrderBy::IdentityIdAsc => {
            if idx == 0 {
              q = q.order_by(staffs::identity_id.asc());
            } else {
              q = q.then_order_by(staffs::identity_id.asc());
            }
          }

          StaffOrderBy::IdentityIdDesc => {
            if idx == 0 {
              q = q.order_by(staffs::identity_id.desc());
            } else {
              q = q.then_order_by(staffs::identity_id.desc());
            }
          }

          StaffOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(staffs::created_at.asc());
            } else {
              q = q.then_order_by(staffs::created_at.asc());
            }
          }

          StaffOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(staffs::created_at.desc());
            } else {
              q = q.then_order_by(staffs::created_at.desc());
            }
          }

          StaffOrderBy::UpdatedAtAsc => {
            if idx == 0 {
              q = q.order_by(staffs::updated_at.asc());
            } else {
              q = q.then_order_by(staffs::updated_at.asc());
            }
          }

          StaffOrderBy::UpdatedAtDesc => {
            if idx == 0 {
              q = q.order_by(staffs::updated_at.desc());
            } else {
              q = q.then_order_by(staffs::updated_at.desc());
            }
          }

          StaffOrderBy::DeletedAtAsc => {
            if idx == 0 {
              q = q.order_by(staffs::deleted_at.asc());
            } else {
              q = q.then_order_by(staffs::deleted_at.asc());
            }
          }

          StaffOrderBy::DeletedAtDesc => {
            if idx == 0 {
              q = q.order_by(staffs::deleted_at.desc());
            } else {
              q = q.then_order_by(staffs::deleted_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(Staff::as_select()).load::<Staff>(conn)
  }
  pub fn paginate<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<StaffOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::paginate_extend(offset, limit, ordering, |q| q, conn)
  }
  pub fn paginate_with_soft_deleted_extend<'a, F, Conn>(
    limit: usize,
    offset: usize,
    ordering: Option<&'a Vec<StaffOrderBy>>,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg>,
    ) -> staffs::BoxedQuery<'b, diesel::pg::Pg>,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::SelectableHelper;
    use diesel_async::RunQueryDsl;
    let mut q = staffs::table.into_boxed();
    if let Some(ordering) = ordering {
      for (idx, ord) in ordering.iter().enumerate() {
        match ord {
          StaffOrderBy::IdAsc => {
            if idx == 0 {
              q = q.order_by(staffs::id.asc());
            } else {
              q = q.then_order_by(staffs::id.asc());
            }
          }

          StaffOrderBy::IdDesc => {
            if idx == 0 {
              q = q.order_by(staffs::id.desc());
            } else {
              q = q.then_order_by(staffs::id.desc());
            }
          }

          StaffOrderBy::RoleAsc => {
            if idx == 0 {
              q = q.order_by(staffs::role.asc());
            } else {
              q = q.then_order_by(staffs::role.asc());
            }
          }

          StaffOrderBy::RoleDesc => {
            if idx == 0 {
              q = q.order_by(staffs::role.desc());
            } else {
              q = q.then_order_by(staffs::role.desc());
            }
          }

          StaffOrderBy::IdentityIdAsc => {
            if idx == 0 {
              q = q.order_by(staffs::identity_id.asc());
            } else {
              q = q.then_order_by(staffs::identity_id.asc());
            }
          }

          StaffOrderBy::IdentityIdDesc => {
            if idx == 0 {
              q = q.order_by(staffs::identity_id.desc());
            } else {
              q = q.then_order_by(staffs::identity_id.desc());
            }
          }

          StaffOrderBy::CreatedAtAsc => {
            if idx == 0 {
              q = q.order_by(staffs::created_at.asc());
            } else {
              q = q.then_order_by(staffs::created_at.asc());
            }
          }

          StaffOrderBy::CreatedAtDesc => {
            if idx == 0 {
              q = q.order_by(staffs::created_at.desc());
            } else {
              q = q.then_order_by(staffs::created_at.desc());
            }
          }

          StaffOrderBy::UpdatedAtAsc => {
            if idx == 0 {
              q = q.order_by(staffs::updated_at.asc());
            } else {
              q = q.then_order_by(staffs::updated_at.asc());
            }
          }

          StaffOrderBy::UpdatedAtDesc => {
            if idx == 0 {
              q = q.order_by(staffs::updated_at.desc());
            } else {
              q = q.then_order_by(staffs::updated_at.desc());
            }
          }

          StaffOrderBy::DeletedAtAsc => {
            if idx == 0 {
              q = q.order_by(staffs::deleted_at.asc());
            } else {
              q = q.then_order_by(staffs::deleted_at.asc());
            }
          }

          StaffOrderBy::DeletedAtDesc => {
            if idx == 0 {
              q = q.order_by(staffs::deleted_at.desc());
            } else {
              q = q.then_order_by(staffs::deleted_at.desc());
            }
          }
        }
      }
    }
    q = extend(
      q.offset(offset.try_into().unwrap())
        .limit(limit.try_into().unwrap()),
    );
    q.select(Staff::as_select()).load::<Staff>(conn)
  }
  pub fn paginate_with_soft_deleted<'a, Conn>(
    offset: usize,
    limit: usize,
    ordering: Option<&'a Vec<StaffOrderBy>>,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<Staff>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::paginate_with_soft_deleted_extend(
      offset,
      limit,
      ordering,
      |q| q,
      conn,
    )
  }
}
impl Staff {
  pub fn count_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> staffs::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(
      staffs::table
        .count()
        .filter(staffs::deleted_at.is_null())
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
    Staff::count_extend(|q| q, conn)
  }
  pub fn count_with_soft_deleted_extend<'a, F, Conn>(
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
    F: for<'b> Fn(
      staffs::BoxedQuery<'b, diesel::pg::Pg, diesel::sql_types::BigInt>,
    ) -> staffs::BoxedQuery<
      'b,
      diesel::pg::Pg,
      diesel::sql_types::BigInt,
    >,
  {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    extend(staffs::table.count().into_boxed()).first(conn)
  }
  pub fn count_with_soft_deleted<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    Staff::count_with_soft_deleted_extend(|q| q, conn)
  }
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
  pub fn insert_many<'a, Conn>(
    data: &'a [NewUser<'a>],
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
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
      .get_results::<User>(conn)
  }
}

impl User {
  pub fn get_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
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
    use diesel_async::RunQueryDsl;
    extend(
      users::table
        .filter(users::deleted_at.is_null())
        .filter(users::id.eq(id))
        .into_boxed(),
    )
    .first(conn)
  }
  pub fn get<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::get_extend(id, |q| q, conn)
  }
  pub fn get_many_extend<'a, F, Conn>(
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
    use diesel_async::RunQueryDsl;
    extend(
      users::table
        .filter(users::deleted_at.is_null())
        .into_boxed(),
    )
    .load(conn)
  }
  pub fn get_many<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::get_many_extend(|q| q, conn)
  }
  pub fn get_with_soft_deleted_extend<'a, F, Conn>(
    id: &'a TestType,
    extend: F,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
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
    use diesel_async::RunQueryDsl;
    extend(users::table.filter(users::id.eq(id)).into_boxed()).first(conn)
  }
  pub fn get_with_soft_deleted<'a, Conn>(
    id: &'a TestType,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<User, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::get_with_soft_deleted_extend(id, |q| q, conn)
  }
  pub fn get_many_with_soft_deleted_extend<'a, F, Conn>(
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
    use diesel_async::RunQueryDsl;
    extend(users::table.into_boxed()).load(conn)
  }
  pub fn get_many_with_soft_deleted<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<Vec<User>, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::get_many_with_soft_deleted_extend(|q| q, conn)
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
  pub fn paginate_with_soft_deleted_extend<'a, F, Conn>(
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
    let mut q = users::table.into_boxed();
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
  pub fn paginate_with_soft_deleted<'a, Conn>(
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
    User::paginate_with_soft_deleted_extend(
      offset,
      limit,
      ordering,
      |q| q,
      conn,
    )
  }
}

pub struct UserCursor {
  pub created_at: time::OffsetDateTime,
  pub id: TestType,
}

impl From<User> for UserCursor {
  fn from(value: User) -> Self {
    Self {
      created_at: value.created_at,
      id: value.id,
    }
  }
}
impl UserCursor {
  pub fn paginate<'a, Conn>(
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
    UserCursor::paginate_extend(after, before, limit, offset, |q| q, conn)
  }
  pub fn paginate_extend<'a, F, Conn>(
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
  pub fn has_next<'a, Conn>(
    cursor: &'a UserCursor,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    UserCursor::has_next_extend(cursor, |q| q, conn)
  }

  pub fn has_next_extend<'a, F, Conn>(
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
        .filter(users::deleted_at.is_null())
        .into_boxed(),
    );

    diesel::select(diesel::dsl::exists(q)).get_result(conn)
  }

  pub fn has_previous<'a, Conn>(
    cursor: &'a UserCursor,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    UserCursor::has_previous_extend(cursor, |q| q, conn)
  }

  pub fn has_previous_extend<'a, F, Conn>(
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
        .filter(users::deleted_at.is_null())
        .into_boxed(),
    );

    diesel::select(diesel::dsl::exists(q)).get_result(conn)
  }

  pub fn paginate_with_soft_deleted<'a, Conn>(
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
    UserCursor::paginate_with_solf_deleted_extend(
      after,
      before,
      limit,
      offset,
      |q| q,
      conn,
    )
  }
  pub fn paginate_with_soft_deleted_extend<'a, F, Conn>(
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
  pub fn has_next_with_soft_deleted<'a, Conn>(
    cursor: &'a UserCursor,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    UserCursor::has_next_with_soft_deleted_extend(cursor, |q| q, conn)
  }

  pub fn has_next_with_soft_deleted_extend<'a, F, Conn>(
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

  pub fn has_previous_with_soft_deleted<'a, Conn>(
    cursor: &'a UserCursor,
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<bool, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    UserCursor::has_previous_with_soft_deleted_extend(cursor, |q| q, conn)
  }

  pub fn has_previous_with_soft_deleted_extend<'a, F, Conn>(
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
  pub fn count_with_soft_deleted_extend<'a, F, Conn>(
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
    extend(users::table.count().into_boxed()).first(conn)
  }
  pub fn count_with_soft_deleted<'a, Conn>(
    conn: &'a mut Conn,
  ) -> impl std::future::Future<Output = Result<i64, diesel::result::Error>>
       + Send
       + 'a
  where
    Conn: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
  {
    User::count_with_soft_deleted_extend(|q| q, conn)
  }
}
