use serde::{Serialize, Deserialize};

// An enum to store the body of a post request.
// To be used with serde_qs.
#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum PostBody {
    #[serde(rename = "enter")]
    Login {
        csrf_token: String,
        ftaa: String,
        bfaa: String,
        #[serde(rename = "_tta")]
        tta: String,
        #[serde(rename = "handleOrEmail")]
        handle_or_email: String,
        password: String,
        remember: String,
    },
    #[serde(rename = "submitSolutionFormSubmitted")]
    Submit {
        csrf_token: String,
        ftaa: String,
        bfaa: String,
        #[serde(rename = "_tta")]
        tta: String,
        #[serde(rename = "contestId")]
        contest_id: String,
        #[serde(rename = "submittedProblemIndex")]
        problem_index: String,
        #[serde(rename = "programTypeId")]
        language_id: u8,
        source: String,
        #[serde(rename = "tabSize")]
        tab_size: u8,
        #[serde(rename = "sourceFile")]
        source_file: String,
    },
}
