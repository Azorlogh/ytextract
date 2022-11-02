use std::time::Duration;

use reqwest::Url;
use serde::Deserialize;

use super::Thumbnails;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Result<T> {
    Ok(T),
    Error {
        #[serde(rename = "playabilityStatus")]
        playability_status: PlayabilityStatus,
    },
}

impl<T> Result<T> {
    pub fn into_std(self) -> crate::Result<T> {
        match self {
            Self::Error { playability_status } => Err(crate::Error::Youtube(
                crate::error::Youtube(playability_status.reason),
            )),
            Self::Ok(ok) => Ok(ok),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub video_details: VideoDetails,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub title: String,
    pub video_id: crate::video::Id,
    #[serde_as(as = "serde_with::DurationSeconds<String>")]
    pub length_seconds: Duration,

    #[serde(default)]
    pub keywords: Vec<String>,
    pub channel_id: crate::channel::Id,

    pub author: String,
    pub short_description: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub view_count: u64,

    pub is_live_content: bool,

    pub thumbnail: Thumbnails,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamPlayerResponse {
    pub streaming_data: StreamingData,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde(default)]
    pub adaptive_formats: Vec<Format>,
}

#[derive(Deserialize, Clone)]
pub struct Format {
    #[serde(flatten)]
    pub base: CommonFormat,
    #[serde(flatten)]
    pub ty: FormatType,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonFormat {
    pub url: Url,
    pub mime_type: String,
    pub itag: u64,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(default)]
    pub content_length: Option<u64>,
    pub bitrate: u64,
    #[serde_as(as = "Option<serde_with::DurationMilliSeconds<String>>")]
    #[serde(default, rename = "approxDurationMs")]
    pub duration: Option<Duration>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum FormatType {
    Audio(AudioFormat),
    Video(VideoFormat),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoFormat {
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub quality_label: String,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioFormat {
    pub loudness_db: Option<f64>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub audio_sample_rate: u64,
    pub audio_quality: String,
    pub audio_channels: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub reason: String,
}
