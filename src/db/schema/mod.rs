// @generated automatically by Diesel CLI.

pub mod iam {
  pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "credential_type", schema = "iam"))]
    pub struct CredentialType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gender", schema = "iam"))]
    pub struct Gender;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "staff_role", schema = "iam"))]
    pub struct StaffRole;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "subject_type", schema = "iam"))]
    pub struct SubjectType;
  }

  diesel::table! {
      iam._prisma_migrations (id) {
          #[max_length = 36]
          id -> Varchar,
          #[max_length = 64]
          checksum -> Varchar,
          finished_at -> Nullable<Timestamptz>,
          #[max_length = 255]
          migration_name -> Varchar,
          logs -> Nullable<Text>,
          rolled_back_at -> Nullable<Timestamptz>,
          started_at -> Timestamptz,
          applied_steps_count -> Int4,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::CredentialType;

      iam.credentials (id) {
          id -> Uuid,
          identity_id -> Uuid,
          enabled -> Bool,
          #[sql_name = "type"]
          type_ -> CredentialType,
          #[max_length = 1023]
          content -> Varchar,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
      }
  }

  diesel::table! {
      iam.email_address_verification_codes (id) {
          id -> Uuid,
          #[max_length = 255]
          value -> Varchar,
          expires_at -> Timestamptz,
          email_address_id -> Uuid,
          created_at -> Timestamptz,
          invalidated_at -> Nullable<Timestamptz>,
      }
  }

  diesel::table! {
      iam.email_addresses (id) {
          id -> Uuid,
          identity_id -> Uuid,
          #[max_length = 320]
          value -> Varchar,
          primary -> Bool,
          verified_at -> Nullable<Timestamptz>,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::Gender;

      iam.identities (id) {
          id -> Uuid,
          username -> Varchar,
          display_picture_url -> Nullable<Varchar>,
          first_name -> Varchar,
          middle_name -> Nullable<Varchar>,
          last_name -> Varchar,
          name_suffix -> Nullable<Varchar>,
          gender -> Nullable<Gender>,
          other_gender -> Nullable<Text>,
      }
  }

  diesel::table! {
      iam.password_reset_tokens (id) {
          id -> Uuid,
          #[max_length = 255]
          value -> Varchar,
          expires_at -> Timestamptz,
          created_at -> Timestamptz,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::SubjectType;

      iam.refresh_tokens (id) {
          id -> Uuid,
          identity_id -> Uuid,
          value -> Text,
          scope -> Text,
          audience -> Text,
          subject -> Uuid,
          subject_type -> SubjectType,
          expires_at -> Timestamptz,
          created_at -> Timestamptz,
          invalidated_at -> Nullable<Timestamptz>,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::StaffRole;

      iam.staffs (id) {
          id -> Uuid,
          role -> StaffRole,
          identity_id -> Uuid,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
          deleted_at -> Nullable<Timestamptz>,
      }
  }

  diesel::table! {
      iam.users (id) {
          id -> Uuid,
          identity_id -> Uuid,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
          deleted_at -> Nullable<Timestamptz>,
      }
  }

  diesel::joinable!(credentials -> identities (identity_id));
  diesel::joinable!(email_address_verification_codes -> email_addresses (email_address_id));
  diesel::joinable!(email_addresses -> identities (identity_id));
  diesel::joinable!(refresh_tokens -> identities (identity_id));
  diesel::joinable!(staffs -> identities (identity_id));
  diesel::joinable!(users -> identities (identity_id));

  diesel::allow_tables_to_appear_in_same_query!(
    _prisma_migrations,
    credentials,
    email_address_verification_codes,
    email_addresses,
    identities,
    password_reset_tokens,
    refresh_tokens,
    staffs,
    users,
  );
}
