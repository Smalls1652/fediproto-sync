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
    synced_posts (id) {
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
