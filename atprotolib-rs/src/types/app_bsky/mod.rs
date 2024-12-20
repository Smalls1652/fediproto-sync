/// Contains types for the `app.bsky.actor` namespace.
pub mod actor;

/// Contains types for the `app.bsky.embed` namespace.
pub mod embed;

/// Contains types for the `app.bsky.feed` namespace.
pub mod feed;

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
