// @generated automatically by Diesel CLI.

diesel::table! {
    mastodon_posts (id) {
        id -> crate::type_impls::MultiBackendUuid,
        account_id -> VarChar,
        post_id -> VarChar,
        created_at -> Timestamp,
        is_thread_post -> Bool,
        previous_post_id -> Nullable<VarChar>,
        bsky_post_id -> Nullable<VarChar>,
        root_mastodon_post_id -> Nullable<VarChar>,
    }
}

diesel::table! {
    synced_posts_bluesky_data (id) {
        id -> crate::type_impls::MultiBackendUuid,
        mastodon_post_id -> VarChar,
        bsky_post_cid -> VarChar,
        bsky_post_uri -> VarChar,
    }
}

diesel::table! {
    cached_service_tokens(id) {
        id -> crate::type_impls::MultiBackendUuid,
        service_name -> VarChar,
        access_token -> VarChar,
        refresh_token -> Nullable<VarChar>,
        expires_in -> Nullable<Integer>,
        scopes -> Nullable<VarChar>,
    }
}

diesel::table! {
    cached_files (id) {
        id -> crate::type_impls::MultiBackendUuid,
        file_path -> VarChar
    }
}

diesel::table! {
    mastodon_post_retry_queue (id) {
        id -> BigInt,
        mastodon_post_id -> VarChar,
        failure_reason -> VarChar,
        last_retried_at -> Timestamp,
        retry_count -> Integer
    }
}
