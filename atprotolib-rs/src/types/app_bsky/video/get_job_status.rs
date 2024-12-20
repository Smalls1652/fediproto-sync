use serde::{Deserialize, Serialize};

use super::JobStatus;

/*
    app.bsky.video.getJobStatus
*/

/// The response to a request for the status of a video upload job.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetJobStatusResponse {
    /// The status of the job.
    #[serde(rename = "jobStatus")]
    pub job_status: JobStatus
}
