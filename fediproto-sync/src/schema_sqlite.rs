// @generated automatically by Diesel CLI.

diesel::table! {
    mastodon_posts (id) {
        id -> Text,
        account_id -> Text,
        post_id -> Text,
        created_at -> Timestamp,
        is_thread_post -> Bool,
        previous_post_id -> Nullable<Text>,
        bsky_post_id -> Nullable<Text>,
        root_mastodon_post_id -> Nullable<Text>,
    }
}

diesel::table! {
    synced_posts_bluesky_data (id) {
        id -> Text,
        mastodon_post_id -> Text,
        bsky_post_cid -> Text,
        bsky_post_uri -> Text,
    }
}

diesel::table! {
    cached_files (id) {
        id -> Text,
        file_path -> Text
    }
}

diesel::table! {
    cached_service_tokens(id) {
        id -> Text,
        service_name -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        expires_in -> Nullable<Integer>,
        scopes -> Nullable<Text>,
    }
}
