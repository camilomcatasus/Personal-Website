use anyhow::Context;
use serde::{Serialize, Deserialize};
use enum_macros::EnumArray;
use rand::seq::SliceRandom;

mod environment;

#[derive(Serialize, Deserialize)]
pub struct ResponseObject {
    pub inner_text: String,
    pub help_text: Option<String>,
    pub url: String
}

#[derive(EnumArray)]
pub enum RequestObject {
    AirQualityRequest,
    LastAnimeRequest,
    LeagueRankRequest,
    //AnimeStatsRequest,
}

pub async fn getApiText( debug_option: Option<&RequestObject>) -> anyhow::Result<ResponseObject> {
    
    let choice: &RequestObject;
    match debug_option {
        Some(requested_object) => choice = requested_object,
        None => choice = RequestObject::VARIANTS.choose(&mut rand::thread_rng()).unwrap()
    }
    
    let new_inner_text: String;
    let new_help_text: Option<String>;
    let new_url: String;
    let client = reqwest::Client::new();

    match *choice {
        RequestObject::AirQualityRequest => {
            let response: AirQuality = client.get("https://api.api-ninjas.com/v1/airquality?city=Miami")
                .header("X-Api-Key", environment::NINJA_API_KEY)
                .send().await?
                .json().await?;

            new_inner_text = format!("The overall airquality index in Miami is {}", response.overall_aqi);
            new_help_text = Some(format!("Air Quality Index Key:<br><ul>{}{}{}{}{}{}</ul>", 
                                    "<li class='text-green-500'>Good: 0 to 50</li>",
                                    "<li class='text-yellow-200'>Moderate: 51 to 100</li>",
                                    "<li class='text-orange-400'>Unhealthy for Sensitive Groups: 101 to 150</li>",
                                    "<li class='text-red-500'>Unhealthy: 151 to 200</li>",
                                    "<li class='text-indigo-600'>Very Unhealthy: 201 to 300</li>",
                                    "<li class='text-rose-700'>Hazardous: 301 and higher</li>"));
            new_url = String::from("https://api-ninjas.com/api/airquality");
        },
        RequestObject::LastAnimeRequest => {
            let response: MalResponse = client.get("https://api.myanimelist.net/v2/users/unkownfire25/animelist?sort=list_updated_at&status=completed")
                .header("X-MAL-CLIENT-ID", environment::MAL_API_KEY)
                .send().await?
                .json().await?;

            let last_daum = response.data[0].node.clone();

            new_inner_text = format!("The last anime I watched was <a href='https://myanimelist.net/anime/{}' target='_blank' >{}<a>", last_daum.id, last_daum.title);
            new_help_text = Some(format!("The information here is derived from my MyAnimeList profile and might not be completely accurate"));
            new_url = format!("https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get");
        },
        RequestObject::LeagueRankRequest => {
            let response: Vec<LeagueRankResponse> = client.get("https://na1.api.riotgames.com/lol/league/v4/entries/by-summoner/cbOutPi9615Fn8WgciJfCwBv5A72Fuheo5zAFcIaGec2jI4")
                .header("X-Riot-Token", environment::RIOT_API_KEY)
                .send().await?
                .json().await?;

            let ranked_flex_response = response.iter().find( |e| e.queue_type == "RANKED_FLEX_SR").context("Ranked Flex not found")?;
            let rank = ranked_flex_response.rank.clone().context("Rank not found in response")?;
            let tier = ranked_flex_response.tier.clone().context("Tier not found in response")?;

            new_inner_text = format!("I am currently rank {} {} in League of Legends", tier, rank);
            new_help_text = None;
            new_url = String::from("https://developer.riotgames.com/apis#league-v4/GET_getLeagueEntriesForSummoner");
        },
    }

    return Ok(ResponseObject {
        inner_text: new_inner_text,
        help_text: new_help_text,
        url: new_url
    })
}

#[derive(Serialize, Deserialize)]
pub struct AirQuality {
    overall_aqi: usize
}
 
#[derive(Serialize, Deserialize)]
pub struct AirQualityField {
    concentration: f64,
    aqi: usize
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
#[serde(rename_all = "camelCase")]
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
