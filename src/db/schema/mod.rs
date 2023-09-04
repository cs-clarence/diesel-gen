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
      use diesel::sql_types::*;
      use super::sql_types::CredentialType;

      iam.credentials (id) {
          id -> Uuid,
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
          expires_at -> Timestamptz,
          email_address_id -> Uuid,
          created_at -> Timestamptz,
          invalidated_at -> Nullable<Timestamptz>,
          #[max_length = 255]
          value -> Varchar,
      }
  }

  diesel::table! {
      iam.email_addresses (id) {
          id -> Uuid,
          #[max_length = 320]
          value -> Varchar,
          primary -> Bool,
          verified_at -> Nullable<Timestamptz>,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
      }
  }

  diesel::table! {
      iam.password_reset_tokens (id) {
          id -> Uuid,
          #[max_length = 255]
          token -> Varchar,
          expires_at -> Timestamptz,
          created_at -> Timestamptz,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::SubjectType;

      iam.refresh_tokens (id) {
          id -> Uuid,
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
      iam.staff_credentials (staff_id, credential_id) {
          staff_id -> Uuid,
          credential_id -> Uuid,
      }
  }

  diesel::table! {
      iam.staff_email_addresses (staff_id, email_address_id) {
          staff_id -> Uuid,
          email_address_id -> Uuid,
      }
  }

  diesel::table! {
      iam.staff_password_reset_tokens (staff_id, password_reset_token_id) {
          staff_id -> Uuid,
          password_reset_token_id -> Uuid,
      }
  }

  diesel::table! {
      iam.staff_refresh_tokens (staff_id, refresh_token_id) {
          staff_id -> Uuid,
          refresh_token_id -> Uuid,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::StaffRole;

      iam.staffs (id) {
          id -> Uuid,
          username -> Varchar,
          display_picture_url -> Nullable<Varchar>,
          first_name -> Varchar,
          middle_name -> Nullable<Varchar>,
          last_name -> Varchar,
          name_suffix -> Nullable<Varchar>,
          role -> StaffRole,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
          deleted_at -> Nullable<Timestamptz>,
      }
  }

  diesel::table! {
      iam.user_credentials (user_id, credential_id) {
          user_id -> Uuid,
          credential_id -> Uuid,
      }
  }

  diesel::table! {
      iam.user_email_addresses (user_id, email_address_id) {
          user_id -> Uuid,
          email_address_id -> Uuid,
      }
  }

  diesel::table! {
      iam.user_password_reset_tokens (user_id, password_reset_token_id) {
          user_id -> Uuid,
          password_reset_token_id -> Uuid,
      }
  }

  diesel::table! {
      iam.user_refresh_tokens (user_id, refresh_token_id) {
          user_id -> Uuid,
          refresh_token_id -> Uuid,
      }
  }

  diesel::table! {
      use diesel::sql_types::*;
      use super::sql_types::Gender;

      iam.users (id) {
          id -> Uuid,
          username -> Varchar,
          display_picture_url -> Nullable<Varchar>,
          first_name -> Varchar,
          middle_name -> Nullable<Varchar>,
          last_name -> Varchar,
          name_suffix -> Nullable<Varchar>,
          created_at -> Timestamptz,
          updated_at -> Timestamptz,
          deleted_at -> Nullable<Timestamptz>,
          gender -> Nullable<Gender>,
          other_gender -> Nullable<Text>,
      }
  }

  diesel::joinable!(email_address_verification_codes -> email_addresses (email_address_id));
  diesel::joinable!(staff_credentials -> credentials (credential_id));
  diesel::joinable!(staff_credentials -> staffs (staff_id));
  diesel::joinable!(staff_email_addresses -> email_addresses (email_address_id));
  diesel::joinable!(staff_email_addresses -> staffs (staff_id));
  diesel::joinable!(staff_password_reset_tokens -> password_reset_tokens (password_reset_token_id));
  diesel::joinable!(staff_password_reset_tokens -> staffs (staff_id));
  diesel::joinable!(staff_refresh_tokens -> refresh_tokens (refresh_token_id));
  diesel::joinable!(staff_refresh_tokens -> staffs (staff_id));
  diesel::joinable!(user_credentials -> credentials (credential_id));
  diesel::joinable!(user_credentials -> users (user_id));
  diesel::joinable!(user_email_addresses -> email_addresses (email_address_id));
  diesel::joinable!(user_email_addresses -> users (user_id));
  diesel::joinable!(user_password_reset_tokens -> password_reset_tokens (password_reset_token_id));
  diesel::joinable!(user_password_reset_tokens -> users (user_id));
  diesel::joinable!(user_refresh_tokens -> refresh_tokens (refresh_token_id));
  diesel::joinable!(user_refresh_tokens -> users (user_id));

  diesel::allow_tables_to_appear_in_same_query!(
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
  );
}
