/// Contains types for the `com.atproto.admin` namespace.
pub mod admin {
    mod defs;
    mod delete_account;
    mod disable_account_invites;
    mod disable_invite_codes;
    mod enable_account_invites;
    mod get_account_info;
    mod get_account_infos;
    mod get_invite_codes;
    mod get_subject_status;
    mod search_accounts;
    mod send_email;
    mod update_account_email;
    mod update_account_handle;
    mod update_account_password;
    mod update_subject_status;

    pub use self::{
        defs::*,
        delete_account::*,
        disable_account_invites::*,
        disable_invite_codes::*,
        enable_account_invites::*,
        get_account_info::*,
        get_account_infos::*,
        get_invite_codes::*,
        get_subject_status::*,
        search_accounts::*,
        send_email::*,
        update_account_email::*,
        update_account_handle::*,
        update_account_password::* // update_subject_status::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `com.atproto.identity` namespace.
pub mod identity {
    mod get_recommended_did_credentials;
    mod resolve_handle;
    mod sign_plc_operation;
    mod submit_plc_operation;
    mod update_handle;

    pub use self::{
        get_recommended_did_credentials::*,
        resolve_handle::*,
        sign_plc_operation::*,
        submit_plc_operation::*,
        update_handle::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `com.atproto.label` namespace.
pub mod label {
    mod defs;
    mod query_labels;
    mod subscribe_labels;

    pub use self::{defs::*, query_labels::*, subscribe_labels::*};
}

/// Contains types for the `com.atproto.moderation` namespace.
pub mod moderation {
    mod create_report;

    pub use self::create_report::*;

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `com.atproto.repo` namespace.
pub mod repo {
    mod apply_writes;
    mod create_record;
    mod defs;
    mod delete_record;
    mod describe_repo;
    mod get_record;
    mod list_missing_blobs;
    mod list_records;
    mod put_record;
    mod upload_blob;

    pub use self::{
        apply_writes::*,
        create_record::*,
        defs::*,
        delete_record::*,
        describe_repo::*,
        get_record::*,
        list_missing_blobs::*,
        list_records::*,
        put_record::*,
        upload_blob::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `com.atproto.server` namespace.
pub mod server {
    mod check_account_status;
    mod confirm_email;
    mod create_account;
    mod create_app_password;
    mod create_invite_code;
    mod create_invite_codes;
    mod create_session;
    mod deactivate_account;
    mod defs;
    mod delete_account;
    mod describe_server;
    mod get_account_invite_codes;
    mod get_service_auth;
    mod get_session;
    mod list_app_passwords;
    mod refresh_session;
    mod request_email_update;
    mod request_password_reset;
    mod reserve_signing_key;
    mod reset_password;
    mod revoke_app_password;
    mod update_email;

    pub use self::{
        check_account_status::*,
        confirm_email::*,
        create_account::*,
        create_app_password::*,
        create_invite_code::*,
        create_invite_codes::*,
        create_session::*,
        deactivate_account::*,
        defs::*,
        delete_account::*,
        describe_server::*,
        // get_account_invite_codes::*,
        get_service_auth::*,
        get_session::*,
        list_app_passwords::*,
        refresh_session::*,
        request_email_update::*,
        request_password_reset::*,
        reserve_signing_key::*,
        reset_password::*,
        revoke_app_password::*,
        update_email::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `com.atproto.sync` namespace.
pub mod sync {
    mod get_head;
    mod get_latest_commit;
    mod get_repo_status;
    mod list_blobs;
    mod list_repos;
    mod notify_of_update;
    mod request_crawl;
    mod subscribe_repos;

    pub use self::{
        get_head::*,
        get_latest_commit::*,
        get_repo_status::*,
        list_blobs::*,
        list_repos::*,
        notify_of_update::*,
        request_crawl::*,
        subscribe_repos::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}
