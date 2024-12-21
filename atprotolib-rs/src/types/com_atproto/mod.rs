/// Contains types for the `com.atproto.admin` namespace.
pub mod admin;

/// Contains types for the `com.atproto.identity` namespace.
pub mod identity;

/// Contains types for the `com.atproto.label` namespace.
pub mod label;

/// Contains types for the `com.atproto.moderation` namespace.
pub mod moderation;

pub mod repo;

/// Contains types for the `com.atproto.server` namespace.
pub mod server;

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
