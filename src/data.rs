use std::error::Error;
use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "Year", deserialize_with = "deserialize_optional_f64")]
    pub year: Option<f64>,
    #[serde(rename = "Genre")]
    pub genre: String,
    #[serde(rename = "Publisher")]
    pub publisher: String,
    #[serde(rename = "Global_Sales")]
    pub global_sales: Option<f64>,
}

fn deserialize_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        match s.trim().parse::<f64>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn load_games(path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_path(path)?;

    let mut games = Vec::new();
    for result in rdr.deserialize() {
        let game: Game = result?;
        games.push(game);
    }
    Ok(games)
}
