// @generated automatically by Diesel CLI.

diesel::table! {
    mastodon_posts (id) {
        id -> Uuid,
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
        id -> Uuid,
        mastodon_post_id -> VarChar,
        bsky_post_cid -> VarChar,
        bsky_post_uri -> VarChar,
    }
}

diesel::table! {
    cached_files (id) {
        id -> Uuid,
        file_path -> VarChar
    }
}

diesel::table! {
    cached_service_tokens(id) {
        id -> Uuid,
        service_name -> VarChar,
        access_token -> VarChar,
        refresh_token -> Nullable<VarChar>,
        expires_in -> Nullable<Timestamp>,
        scopes -> Nullable<VarChar>,
    }
}

diesel::table! {
    mastodon_post_retry_queue (id) {
        id -> Uuid,
        mastodon_post_id -> VarChar,
        failure_reason -> VarChar
    }
}
