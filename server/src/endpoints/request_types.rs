use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct AirQuality {
    pub overall_aqi: usize
}
 
#[derive(Serialize, Deserialize)]
pub struct AirQualityField {
    pub concentration: f64,
    pub aqi: usize
}

#[derive(Serialize, Deserialize)]
pub struct MalResponse {
    pub data: Vec<MalDaum>,
}

#[derive(Serialize, Deserialize)]
pub struct MalDaum {
    pub node: MalNode,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MalNode {
    pub id: i64,
    pub title: String,
    pub main_picture: MalMainPicture,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MalMainPicture {
    pub medium: String,
    pub large: String
}

#[derive(Serialize, Deserialize)]
pub struct MalListStatus {
    pub status: String,
    pub score: i64,
    pub num_watched_episodes: Option<i64>,
    pub is_rewatching: bool,
    pub updated_at: String,
    pub num_episodes_watched: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LeagueRankResponse {
    pub league_id: Option<String>,
    pub queue_type: String,
    pub tier: Option<String>,
    pub rank: Option<String>,
    pub summoner_id: String,
    pub summoner_name: String,
    pub league_points: u64,
    pub wins: u64,
    pub losses: u64,
    pub veteran: bool,
    pub inactive: bool,
    pub fresh_blood: bool,
    pub hot_streak: bool
}

#[derive(Serialize, Deserialize)]
pub struct HistoricalEventResponse {
    pub year: String,
    pub month: String,
    pub day: String,
    pub event: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BreakingNewsResponse {
    pub status: String,
    pub total_results: String,
    pub results: Vec<BreakingNewsArticle>
}

#[derive(Serialize, Deserialize)]
pub struct BreakingNewsArticle {
    pub title: String,
    pub link: String,
    pub video_url: Option<String>,
    pub description: String,
}
