/// Contains types for the `app.bsky.actor` namespace.
pub mod actor;

/// Contains types for the `app.bsky.embed` namespace.
pub mod embed {
    mod defs;
    mod external;
    mod images;
    mod record;
    mod record_with_media;
    mod video;

    pub use self::{defs::*, external::*, images::*, record::*, record_with_media::*, video::*};
}

/// Contains types for the `app.bsky.feed` namespace.
pub mod feed {
    mod defs;
    mod describe_feed_generator;
    mod get_actor_feeds;
    mod get_actor_likes;
    mod get_author_feed;
    mod get_feed;
    mod get_feed_generator;
    mod get_feed_generators;
    mod get_feed_skeleton;
    mod get_likes;
    mod get_list_feed;
    mod get_post_thread;
    mod get_posts;
    mod get_quotes;
    mod get_reposted_by;
    mod get_suggested_feeds;
    mod get_timeline;
    mod post;
    mod postgate;
    mod search_posts;
    mod send_interactions;
    mod threadgate;

    pub use self::{
        defs::*,
        describe_feed_generator::*,
        get_actor_feeds::*,
        get_actor_likes::*,
        get_author_feed::*,
        get_feed::*,
        get_feed_generator::*,
        get_feed_generators::*,
        get_feed_skeleton::*,
        get_likes::*,
        get_list_feed::*,
        get_post_thread::*,
        get_posts::*,
        get_quotes::*,
        get_reposted_by::*,
        get_suggested_feeds::*,
        get_timeline::*,
        post::*,
        postgate::*,
        search_posts::*,
        send_interactions::*,
        threadgate::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `app.bsky.graph` namespace.
pub mod graph {
    mod defs;
    mod get_actor_starter_packs;
    mod get_blocks;
    mod get_followers;
    mod get_follows;
    mod get_known_followers;
    mod get_list;
    mod get_list_blocks;
    mod get_list_mutes;
    mod get_lists;
    mod get_mutes;
    mod get_relationships;
    mod get_starter_pack;
    mod get_starter_packs;
    mod get_suggested_follows_by_actor;
    mod mute_actor;
    mod mute_actor_list;
    mod mute_thread;
    mod search_starter_packs;
    mod starterpack;
    mod unmute_actor;
    mod unmute_actor_list;
    mod unmute_thread;

    pub use self::{
        defs::*,
        get_actor_starter_packs::*,
        get_blocks::*,
        get_followers::*,
        get_follows::*,
        get_known_followers::*,
        get_list::*,
        get_list_blocks::*,
        get_list_mutes::*,
        get_lists::*,
        get_mutes::*,
        get_relationships::*,
        get_starter_pack::*,
        get_starter_packs::*,
        get_suggested_follows_by_actor::*,
        mute_actor::*,
        mute_actor_list::*,
        mute_thread::*,
        search_starter_packs::*,
        starterpack::*,
        unmute_actor::*,
        unmute_actor_list::*,
        unmute_thread::*
    };

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}

/// Contains types for the `app.bsky.labeler` namespace.
pub mod labeler {
    mod defs;
    mod get_services;

    pub use self::{defs::*, get_services::*};
}

/// Contains types for the `app.bsky.notification` namespace.
pub mod notification {
    mod get_unread_count;
    mod list_notifications;
    mod put_preferences;
    mod register_push;
    mod update_seen;

    pub use self::{
        get_unread_count::*,
        list_notifications::*,
        put_preferences::*,
        register_push::*,
        update_seen::*
    };
}

/// Contains types for the `app.bsky.richtext` namespace.
pub mod richtext {
    mod facet;

    pub use self::facet::*;
}

/// Contains types for the `app.bsky.video` namespace.
pub mod video {
    mod defs;
    mod get_job_status;
    mod get_upload_limits;
    mod upload_video;

    pub use self::{defs::*, get_job_status::*, get_upload_limits::*, upload_video::*};

    #[cfg(feature = "apicalls")]
    mod api_calls;

    #[cfg(feature = "apicalls")]
    pub use self::api_calls::*;
}
