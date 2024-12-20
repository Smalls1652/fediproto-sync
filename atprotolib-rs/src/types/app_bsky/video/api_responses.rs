use serde::{Deserialize, Serialize};

use super::JobStatus;

/// The response to a request for the status of a video upload job.
#[derive(Serialize, Deserialize, Debug)]
pub struct JobStatusResponse {
    /// The status of the job.
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus
}

/// The response to a request for the upload limits of a PDS.
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadLimitsResponse {
    /// Whether the user can upload a video.
    #[serde(rename = "canUpload", default)]
    pub can_upload: bool,

    /// The remaining daily video uploads.
    #[serde(rename = "remainingDailyVideos", default)]
    pub remaining_daily_videos: i32,

    /// The remaining daily bytes of video uploads.
    #[serde(rename = "remainingDailyBytes", default)]
    pub remaining_daily_bytes: i32,

    /// A message about the upload limits.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// An error message.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>
}

/// The response to a request to upload a video.
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadVideoResponse {
    /// The status of the job.
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus
}
