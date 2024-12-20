use serde::{Deserialize, Serialize};

use super::JobStatus;

/*
    app.bsky.video.uploadVideo
*/

/// The response to a request to upload a video.
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadVideoResponse {
    /// The status of the job.
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus
}
