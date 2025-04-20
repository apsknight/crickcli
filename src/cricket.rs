use serde::{Deserialize, Serialize};
use std::error::Error;
use tabled::{Table, Tabled, settings::Style};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "typeMatches")]
    pub type_matches: Vec<TypeMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeMatch {
    #[serde(rename = "matchType")]
    pub match_type: String,
    #[serde(rename = "seriesMatches")]
    pub series_matches: Vec<SeriesMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesMatch {
    #[serde(rename = "seriesAdWrapper")]
    pub series_ad_wrapper: Option<SeriesAdWrapper>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesAdWrapper {
    #[serde(rename = "seriesId")]
    pub series_id: i64,
    #[serde(rename = "seriesName")]
    pub series_name: String,
    pub matches: Vec<Match>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "matchInfo")]
    pub match_info: MatchInfo,
    #[serde(rename = "matchScore")]
    pub match_score: Option<MatchScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfo {
    #[serde(rename = "matchId")]
    pub match_id: i64,
    #[serde(rename = "seriesId")]
    pub series_id: i64,
    #[serde(rename = "seriesName")]
    pub series_name: String,
    #[serde(rename = "matchDesc")]
    pub match_desc: String,
    #[serde(rename = "matchFormat")]
    pub match_format: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub state: String,
    pub status: Option<String>,
    pub team1: Team,
    pub team2: Team,
    #[serde(rename = "venueInfo")]
    pub venue_info: VenueInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "teamId")]
    pub team_id: i64,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "teamSName")]
    pub team_s_name: String,
    #[serde(rename = "imageId")]
    pub image_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VenueInfo {
    pub id: Option<i64>,
    pub ground: String,
    pub city: String,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchScore {
    #[serde(rename = "team1Score")]
    pub team1_score: Option<TeamScore>,
    #[serde(rename = "team2Score")]
    pub team2_score: Option<TeamScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamScore {
    #[serde(rename = "inngs1")]
    pub innings1: Option<Innings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Innings {
    #[serde(rename = "inningsId")]
    pub innings_id: i64,
    pub runs: i64,
    pub wickets: i64,
    pub overs: f64,
}

#[derive(Tabled)]
struct MatchRow {
    #[tabled(rename = "Teams")]
    teams: String,
    #[tabled(rename = "Venue")]
    venue: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Score")]
    score: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleResponse {
    #[serde(rename = "matchScheduleMap")]
    pub match_schedule_map: Vec<ScheduleMap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleMap {
    #[serde(rename = "scheduleAdWrapper")]
    pub schedule_ad_wrapper: Option<ScheduleAdWrapper>,
    #[serde(rename = "adDetail")]
    pub ad_detail: Option<AdDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleAdWrapper {
    pub date: String,
    #[serde(rename = "matchScheduleList")]
    pub match_schedule_list: Vec<MatchSchedule>,
    #[serde(rename = "longDate")]
    pub long_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchSchedule {
    #[serde(rename = "seriesName")]
    pub series_name: String,
    #[serde(rename = "matchInfo")]
    pub match_info: Vec<ScheduleMatchInfo>,
    #[serde(rename = "seriesId")]
    pub series_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleMatchInfo {
    #[serde(rename = "matchId")]
    pub match_id: i64,
    #[serde(rename = "seriesId")]
    pub series_id: i64,
    #[serde(rename = "matchDesc")]
    pub match_desc: String,
    #[serde(rename = "matchFormat")]
    pub match_format: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub team1: Team,
    pub team2: Team,
    #[serde(rename = "venueInfo")]
    pub venue_info: VenueInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdDetail {
    pub name: String,
    pub layout: String,
    pub position: i64,
}

async fn fetch_matches(endpoint: &str) -> Result<Vec<Match>, Box<dyn Error>> {
    let api_key = std::env::var("CRICKET_API_KEY")
        .expect("CRICKET_API_KEY environment variable not set");
    
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://cricbuzz-cricket.p.rapidapi.com/matches/v1/{}", endpoint))
        .header("X-RapidAPI-Key", api_key)
        .header("X-RapidAPI-Host", "cricbuzz-cricket.p.rapidapi.com")
        .send()
        .await?;

    let api_response: ApiResponse = response.json().await?;
    
    let mut all_matches = Vec::new();
    for type_match in api_response.type_matches {
        for series_match in type_match.series_matches {
            if let Some(series_ad_wrapper) = series_match.series_ad_wrapper {
                all_matches.extend(series_ad_wrapper.matches);
            }
        }
    }
    
    Ok(all_matches)
}

pub async fn get_live_matches() -> Result<Vec<Match>, Box<dyn Error>> {
    fetch_matches("live").await
}

pub async fn get_recent_matches() -> Result<Vec<Match>, Box<dyn Error>> {
    fetch_matches("recent").await
}

pub async fn get_upcoming_matches() -> Result<Vec<Match>, Box<dyn Error>> {
    fetch_matches("upcoming").await
}

pub fn format_matches(matches: &[Match]) -> String {
    let mut rows = Vec::new();
    
    for match_data in matches {
        let score = match &match_data.match_score {
            Some(match_score) => {
                let team1_score = match_score.team1_score.as_ref()
                    .and_then(|s| s.innings1.as_ref())
                    .map(|i| format!("{}: {}/{} ({})", 
                        match_data.match_info.team1.team_s_name,
                        i.runs,
                        i.wickets,
                        i.overs))
                    .unwrap_or_default();
                
                let team2_score = match_score.team2_score.as_ref()
                    .and_then(|s| s.innings1.as_ref())
                    .map(|i| format!("{}: {}/{} ({})", 
                        match_data.match_info.team2.team_s_name,
                        i.runs,
                        i.wickets,
                        i.overs))
                    .unwrap_or_default();
                
                if !team1_score.is_empty() && !team2_score.is_empty() {
                    format!("{}\n{}", team1_score, team2_score)
                } else if !team1_score.is_empty() {
                    team1_score
                } else if !team2_score.is_empty() {
                    team2_score
                } else {
                    String::from("No score available")
                }
            },
            None => String::from("Match not started"),
        };
        
        let row = MatchRow {
            teams: format!("{} vs {}", 
                match_data.match_info.team1.team_s_name,
                match_data.match_info.team2.team_s_name),
            venue: match_data.match_info.venue_info.city.clone(),
            status: match_data.match_info.status.clone().unwrap_or_else(|| String::from("Upcoming")),
            score,
        };
        
        rows.push(row);
    }
    
    if rows.is_empty() {
        return String::from("No matches found");
    }
    
    let table = Table::new(rows)
        .with(Style::modern())
        .to_string();
    
    format!("\n{}\n", table)
}

#[derive(Tabled)]
struct ScheduleRow {
    #[tabled(rename = "Teams")]
    teams: String,
    #[tabled(rename = "Venue")]
    venue: String,
    #[tabled(rename = "Schedule")]
    schedule: String,
}

pub fn format_schedule(matches: &[(ScheduleMatchInfo, String)]) -> String {
    let mut rows = Vec::new();
    
    for (match_data, _series_name) in matches {
        // Convert timestamp to readable date in local timezone
        let timestamp = match_data.start_date.parse::<i64>().unwrap_or(0);
        let datetime = chrono::DateTime::from_timestamp(timestamp / 1000, 0)
            .map(|dt| dt.with_timezone(&chrono::Local))
            .unwrap_or_default();
        let formatted_date = datetime.format("%Y-%m-%d %I:%M %p").to_string();
        
        let row = ScheduleRow {
            teams: format!("{} vs {}", 
                match_data.team1.team_s_name,
                match_data.team2.team_s_name),
            venue: match_data.venue_info.city.clone(),
            schedule: formatted_date,
        };
        
        rows.push(row);
    }
    
    if rows.is_empty() {
        return String::from("No matches found");
    }
    
    let table = Table::new(rows)
        .with(Style::modern())
        .to_string();
    
    format!("\n{}\n", table)
}

pub async fn get_schedule() -> Result<Vec<(ScheduleMatchInfo, String)>, Box<dyn Error>> {
    let api_key = std::env::var("CRICKET_API_KEY")
        .expect("CRICKET_API_KEY environment variable not set");
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://cricbuzz-cricket.p.rapidapi.com/schedule/v1/league")
        .header("X-RapidAPI-Key", api_key)
        .header("X-RapidAPI-Host", "cricbuzz-cricket.p.rapidapi.com")
        .send()
        .await?;

    let schedule_response: ScheduleResponse = response.json().await?;
    
    let mut all_matches = Vec::new();
    for schedule_map in schedule_response.match_schedule_map {
        if let Some(schedule_ad_wrapper) = schedule_map.schedule_ad_wrapper {
            for match_schedule in schedule_ad_wrapper.match_schedule_list {
                for match_info in match_schedule.match_info {
                    all_matches.push((match_info, match_schedule.series_name.clone()));
                }
            }
        }
    }
    
    Ok(all_matches)
} 