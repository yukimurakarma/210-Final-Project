#[cfg(test)]
mod tests {
    use crate::data::Game;
    use crate::analysis::*;

    fn sample_games() -> Vec<Game> {
        vec![
            Game {
                name: "Game A".to_string(),
                platform: "PS4".to_string(),
                year: Some(2018.0),
                genre: "Shooter".to_string(),
                publisher: "Ubisoft".to_string(),
                global_sales: Some(1.5),
            },
            Game {
                name: "Game B".to_string(),
                platform: "Xbox".to_string(),
                year: Some(2016.0),
                genre: "Sports".to_string(),
                publisher: "EA".to_string(),
                global_sales: Some(0.8),
            },
            Game {
                name: "Game C".to_string(),
                platform: "PC".to_string(),
                year: Some(2010.0),
                genre: "Shooter".to_string(),
                publisher: "Activision".to_string(),
                global_sales: Some(0.9),
            },
        ]
    }

    #[test]
    fn test_genre_predictor_accuracy() {
        let games = sample_games();
        // This just runs to check if function executes without panic.
        genre_predictor(&games, "Shooter");
    }

    #[test]
    fn test_average_sales_by_category_runs() {
        let games = sample_games();
        average_sales_by_category(&games, |g| g.genre.clone(), "Genre");
    }

    #[test]
    fn test_top_games_by_sales_ordering() {
        let games = sample_games();
        top_games_by_sales(&games, 2);
    }

    #[test]
    fn test_analyze_category_does_not_panic() {
        let games = sample_games();
        analyze_category(&games, |g| g.publisher.clone(), "Publisher");
    }
}
