/// Contains types for the `app.bsky.actor` namespace.
pub mod actor;

/// Contains types for the `app.bsky.embed` namespace.
pub mod embed;

/// Contains types for the `app.bsky.feed` namespace.
pub mod feed;

/// Contains types for the `app.bsky.graph` namespace.
pub mod graph;

/// Contains types for the `app.bsky.labeler` namespace.
pub mod labeler;

/// Contains types for the `app.bsky.notification` namespace.
pub mod notification;

/// Contains types for the `app.bsky.richtext` namespace.
pub mod richtext;

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
