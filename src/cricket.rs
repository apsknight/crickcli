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
    pub match_score: MatchScore,
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
    pub status: String,
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
    pub id: i64,
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

pub async fn get_live_matches() -> Result<Vec<Match>, Box<dyn Error>> {
    let api_key = std::env::var("CRICKET_API_KEY")
        .expect("CRICKET_API_KEY environment variable not set");
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://cricbuzz-cricket.p.rapidapi.com/matches/v1/live")
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

pub fn format_matches(matches: &[Match]) -> String {
    let mut rows = Vec::new();
    
    for match_data in matches {
        let team1_score = match_data.match_score.team1_score.as_ref()
            .and_then(|s| s.innings1.as_ref())
            .map(|i| format!("{}: {}/{} ({})", 
                match_data.match_info.team1.team_s_name,
                i.runs,
                i.wickets,
                i.overs))
            .unwrap_or_default();
        
        let team2_score = match_data.match_score.team2_score.as_ref()
            .and_then(|s| s.innings1.as_ref())
            .map(|i| format!("{}: {}/{} ({})", 
                match_data.match_info.team2.team_s_name,
                i.runs,
                i.wickets,
                i.overs))
            .unwrap_or_default();
        
        let score = if !team1_score.is_empty() && !team2_score.is_empty() {
            format!("{}\n{}", team1_score, team2_score)
        } else if !team1_score.is_empty() {
            team1_score
        } else if !team2_score.is_empty() {
            team2_score
        } else {
            String::from("No score available")
        };
        
        let row = MatchRow {
            teams: format!("{} vs {}", 
                match_data.match_info.team1.team_name,
                match_data.match_info.team2.team_name),
            venue: format!("{}, {}", 
                match_data.match_info.venue_info.ground,
                match_data.match_info.venue_info.city),
            status: match_data.match_info.status.clone(),
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