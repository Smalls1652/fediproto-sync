// @generated automatically by Diesel CLI.

diesel::table! {
    mastodon_posts (id) {
        id -> crate::db::type_impls::MultiBackendUuid,
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
        id -> crate::db::type_impls::MultiBackendUuid,
        mastodon_post_id -> VarChar,
        bsky_post_cid -> VarChar,
        bsky_post_uri -> VarChar,
    }
}

diesel::table! {
    cached_service_tokens(id) {
        id -> crate::db::type_impls::MultiBackendUuid,
        service_name -> VarChar,
        access_token -> VarChar,
        refresh_token -> Nullable<VarChar>,
        expires_in -> Nullable<Integer>,
        scopes -> Nullable<VarChar>,
    }
}

diesel::table! {
    cached_files (id) {
        id -> crate::db::type_impls::MultiBackendUuid,
        file_path -> VarChar
    }
}
