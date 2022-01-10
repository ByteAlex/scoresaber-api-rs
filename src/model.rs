use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

macro_rules! serde_to_string {
    ($res:ident $(=)? $value:ident) => {
        let val = serde_json::to_string($value).unwrap();
        $res = val[1..val.len() - 1].to_owned();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: String,
    pub name: String,
    pub profile_picture: String,
    pub country: String,
    pub pp: f64,
    pub country_rank: u32,
    pub role: Option<String>,
    pub badges: Option<Vec<Badge>>,
    pub histories: String,
    pub score_stats: Option<ScoreStats>,
    pub permissions: u64,
    pub banned: bool,
    pub inactive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreStats {
    pub total_score: u64,
    pub total_ranked_score: u64,
    pub average_ranked_accuracy: f32,
    pub total_play_count: u32,
    pub ranked_play_count: u32,
    pub replays_watched: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub description: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScore {
    pub score: Score,
    pub leaderboard: LeaderboardInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: u64,
    pub leaderboard_player_info: Option<LeaderboardPlayer>,
    pub rank: u32,
    pub base_score: u32,
    pub modified_score: i32, // it can be negative, gg
    pub pp: f32,
    pub weight: f32,
    pub modifiers: String,
    pub multiplier: f32,
    pub bad_cuts: u32,
    pub missed_notes: u32,
    pub max_combo: u32,
    pub full_combo: bool,
    pub hmd: HeadsetRepr,
    pub has_replay: bool,
    pub time_set: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardPlayer {
    pub id: String,
    pub name: String,
    pub profile_picture: String,
    pub country: String,
    pub permissions: u64,
    pub role: Option<String>,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Eq, PartialEq, Hash)]
pub enum HeadsetRepr {
    Unknown = 0,
    OculusRiftCV1 = 1,
    HtcVive = 2,
    HtcVivePro = 4,
    WindowsMixedReality = 8,
    OculusRiftS = 16,
    OculusQuest = 32,
    ValveIndex = 64,
    HtcViveCosmos = 128,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Headset {
    Unknown,
    #[serde(rename = "OCULUS_RIFT_CV1")]
    OculusRiftCV1,
    HtcVive,
    HtcVivePro,
    WindowsMixedReality,
    OculusRiftS,
    OculusQuest,
    ValveIndex,
    HtcViveCosmos
}

impl Into<Headset> for HeadsetRepr {
    fn into(self) -> Headset {
        match self {
            HeadsetRepr::Unknown => Headset::Unknown,
            HeadsetRepr::OculusRiftCV1 => Headset::OculusRiftCV1,
            HeadsetRepr::HtcVive => Headset::HtcVive,
            HeadsetRepr::HtcVivePro => Headset::HtcVivePro,
            HeadsetRepr::WindowsMixedReality => Headset::WindowsMixedReality,
            HeadsetRepr::OculusRiftS => Headset::OculusRiftS,
            HeadsetRepr::OculusQuest => Headset::OculusQuest,
            HeadsetRepr::ValveIndex => Headset::ValveIndex,
            HeadsetRepr::HtcViveCosmos => Headset::HtcViveCosmos,
        }
    }
}

impl From<Headset> for HeadsetRepr {
    fn from(headset: Headset) -> Self {
        match headset {
            Headset::Unknown => HeadsetRepr::Unknown,
            Headset::OculusRiftCV1 => HeadsetRepr::OculusRiftCV1,
            Headset::HtcVive => HeadsetRepr::HtcVive,
            Headset::HtcVivePro => HeadsetRepr::HtcVivePro,
            Headset::WindowsMixedReality => HeadsetRepr::WindowsMixedReality,
            Headset::OculusRiftS => HeadsetRepr::OculusRiftS,
            Headset::OculusQuest => HeadsetRepr::OculusQuest,
            Headset::ValveIndex => HeadsetRepr::ValveIndex,
            Headset::HtcViveCosmos => HeadsetRepr::HtcViveCosmos,
        }
    }
}

impl HeadsetRepr {
    pub fn family(&self) -> HeadsetFamily {
        match self {
            HeadsetRepr::Unknown => HeadsetFamily::Unknown,
            HeadsetRepr::OculusRiftCV1 => HeadsetFamily::Oculus,
            HeadsetRepr::HtcVive => HeadsetFamily::HTC,
            HeadsetRepr::HtcVivePro => HeadsetFamily::HTC,
            HeadsetRepr::WindowsMixedReality => HeadsetFamily::Microsoft,
            HeadsetRepr::OculusRiftS => HeadsetFamily::Oculus,
            HeadsetRepr::OculusQuest => HeadsetFamily::Oculus,
            HeadsetRepr::ValveIndex => HeadsetFamily::Valve,
            HeadsetRepr::HtcViveCosmos => HeadsetFamily::HTC,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HeadsetFamily {
    Unknown,
    Oculus,
    #[serde(rename = "HTC")]
    HTC,
    Microsoft,
    Valve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardInfo {
    pub id: u32,
    pub song_hash: String,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub difficulty: Difficulty,
    pub max_score: u32,
    pub created_date: DateTime<Utc>,
    pub ranked_data: Option<DateTime<Utc>>,
    pub qualified_date: Option<DateTime<Utc>>,
    pub loved_date: Option<DateTime<Utc>>,
    pub ranked: bool,
    pub qualified: bool,
    pub loved: bool,
    #[serde(rename = "maxPP")]
    pub max_pp: f32,
    pub stars: f32,
    pub positive_modifiers: bool,
    pub players: Option<u32>,
    pub daily_players: Option<u32>,
    pub cover_image: String,
    pub player_score: Option<Score>,
    pub difficulties: Option<Vec<Difficulty>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub leaderboard_id: u32,
    pub difficulty: DifficultyEnum,
    pub game_mode: String,
    pub difficulty_raw: String,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Eq, PartialEq, Ord, PartialOrd)]
pub enum DifficultyEnum {
    Easy = 1,
    Normal = 3,
    Hard = 5,
    Expert = 7,
    ExpertPlus = 9,
}

impl ToString for DifficultyEnum {
    fn to_string(&self) -> String {
        let res;
        serde_to_string!(res = self);
        res
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub total: u32,
    pub page: u32,
    pub items_per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCollection {
    pub players: Vec<Player>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScoreCollection {
    pub player_scores: Vec<PlayerScore>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreCollection {
    pub scores: Vec<Score>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardInfoCollection {
    pub leaderboards: Vec<LeaderboardInfo>,
    pub metadata: Metadata,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
pub enum SortCategory {
    Trending = 0,
    DateRanked = 1,
    ScoresSet = 2,
    StarDifficulty = 3,
    Author = 4,
}

impl ToString for SortCategory {
    fn to_string(&self) -> String {
        let res;
        serde_to_string!(res = self);
        res
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
pub enum SortOrder {
    Descending = 0,
    Ascending = 1,
}

impl ToString for SortOrder {
    fn to_string(&self) -> String {
        let res;
        serde_to_string!(res = self);
        res
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScoreSort {
    Top, Recent
}

impl ToString for ScoreSort {
    fn to_string(&self) -> String {
        let res;
        serde_to_string!(res = self);
        res
    }
}