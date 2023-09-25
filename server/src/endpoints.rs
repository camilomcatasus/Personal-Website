use anyhow::Context;
use serde::{Serialize, Deserialize};
use enum_macros::EnumArray;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use chrono::Datelike;

mod models;
mod environment;


pub const MINUTE: u64 = 60;
pub const HOUR: u64 = 3600;

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseObject {
    pub inner_text: String,
    pub help_text: Option<String>,
    pub url: String
}

#[derive(EnumArray, PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum RequestObject {
    AirQualityRequest,
    LastAnimeRequest,
    LeagueRankRequest,
    HistoricalEventRequest,
    BreakingNewsRequest,
    //AnimeStatsRequest,
}

#[derive(Clone)]
pub struct CacheObject {
    last_get_time: Instant,
    keep_alive: Duration,
    last_response: ResponseObject
}

pub async fn getApiText( debug_option: Option<&RequestObject>, 
                        request_cache: &mut HashMap<RequestObject, CacheObject>) -> anyhow::Result<ResponseObject> {
    
    let choice: &RequestObject;
    match debug_option {
        Some(requested_object) => choice = requested_object,
        None => choice = RequestObject::VARIANTS.choose(&mut rand::thread_rng()).unwrap()
    }
    // Check to see if the key exists in the cache
    let in_cache = request_cache.get(choice).clone();
    
    match in_cache {
        Some(cache_object) => {
            let elapsed: Duration = cache_object.last_get_time.elapsed();
            if elapsed.le(&cache_object.keep_alive) {
                return Ok(cache_object.last_response.clone());
            }
        },
        None => {
            println!("Didn't find object in cache for {:?}", choice);
        }
    }

    let new_inner_text: String;
    let new_help_text: Option<String>;
    let new_url: String;
    let client = reqwest::Client::new();
    let now = Instant::now();
    let response_keep_alive: Duration;

    match *choice {
        RequestObject::AirQualityRequest => {
            let response: models::AirQuality = client.get("https://api.api-ninjas.com/v1/airquality?city=Miami")
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
            response_keep_alive = Duration::new(HOUR, 0);
        },
        RequestObject::LastAnimeRequest => {
            let response: models::MalResponse = client.get("https://api.myanimelist.net/v2/users/unkownfire25/animelist?sort=list_updated_at&status=completed")
                .header("X-MAL-CLIENT-ID", environment::MAL_API_KEY)
                .send().await?
                .json().await?;

            let last_daum = response.data[0].node.clone();

            new_inner_text = format!("The last anime I watched was <a href='https://myanimelist.net/anime/{}' target='_blank' >{}<a>", last_daum.id, last_daum.title);
            new_help_text = Some(format!("The information here is derived from my MyAnimeList profile and might not be completely accurate"));
            new_url = format!("https://myanimelist.net/apiconfig/references/api/v2#operation/users_user_id_animelist_get");
            response_keep_alive = Duration::new(HOUR * 12, 0);
        },
        RequestObject::LeagueRankRequest => {
            
            let response_raw = client.get("https://na1.api.riotgames.com/lol/league/v4/entries/by-summoner/lQVQ5WysORkE3p0673gDfTeKTyJceQ886Imu5jI3HVqpGLs")
                .header("X-Riot-Token", environment::RIOT_API_KEY)
                .send().await?;

            let text_response = response_raw.text().await?;
            println!("League Response Text Body: {}", text_response);
            let response: Vec<models::LeagueRankResponse> = serde_json::from_str(&text_response)?;

            let ranked_flex_response = response.iter().find( |e| e.queue_type == "RANKED_FLEX_SR").context("Ranked Flex not found")?;
            let rank = ranked_flex_response.rank.clone().context("Rank not found in response")?;
            let tier = ranked_flex_response.tier.clone().context("Tier not found in response")?;

            new_inner_text = format!("I am currently {} {} in League of Legends", tier, rank);
            new_help_text = None;
            new_url = String::from("https://developer.riotgames.com/apis#league-v4/GET_getLeagueEntriesForSummoner");
            response_keep_alive = Duration::new(HOUR, 0);
        },
        RequestObject::HistoricalEventRequest => {
            let current_date = chrono::Utc::now();
            let current_month = current_date.month();
            let current_day = current_date.day();

            let response_raw: String = client.get(format!("https://api.api-ninjas.com/v1/historicalevents?day={}&month={}", current_day, current_month))
                .header("X-Api-Key", environment::NINJA_API_KEY)
                .send().await?
                .text().await?;

            let mut response: Vec<models::HistoricalEventResponse> = serde_json::from_str(&response_raw)?;

            let historical_option = response.pop();

            
            match historical_option {
                Some(historical_event) => 
                    new_inner_text = format!("Today {}/{}:{}", current_month, current_day, historical_event.event),
                None => 
                    new_inner_text = String::from("No historical events occurred today!")
            }
            
            new_help_text = None;
            new_url = String::from("https://api-ninjas.com/api/historicalevents");
            response_keep_alive = Duration::new(HOUR * 24, 0);
        },
        RequestObject::BreakingNewsRequest => {

            let response: models::BreakingNewsResponse = client.get(
                format!("https://newsdata.io/api/1/news?apikey={}&country=au,us", environment::NEWS_DATA_API_KEY))
                .send().await?
                .json().await?;

            let top_story_option = response.results.first();

            match top_story_option {
                Some(top_story) => {

                    new_inner_text = format!("Breaking News! <a href=\"{}\">{}</a>", top_story.link, top_story.title);
                    let video_block: String = match &top_story.video_url {
                        Some(val) => {
                            format!("<video controls><source src=\"{}\" type=\"video/mp4\"></video>", val)
                        }
                        None => String::new(),
                    };
                    new_help_text = Some(format!("{}</br>{}", top_story.description, video_block));
                },
                None => {
                    new_inner_text = "No news today I guess!".to_string();
                    new_help_text = None;
                }
            }

            new_url = String::from("https://newsdata.io/documentation");
            response_keep_alive = Duration::new(HOUR + MINUTE * 30, 0);
        }
    }

    let final_response = ResponseObject {
        inner_text: new_inner_text,
        help_text: new_help_text,
        url: new_url
    };

    let final_cache = CacheObject {
        last_get_time: now,
        keep_alive: response_keep_alive,
        last_response: final_response.clone()
    };

    request_cache.insert(*choice, final_cache);

    return Ok(final_response);
}

