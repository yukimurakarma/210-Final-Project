mod data;
mod analysis;

fn main() {
    let games = data::load_games("vgsales.csv").expect("Failed to load CSV");

    println!("\n=== Bestseller Stats by Genre ===");
    analysis::analyze_category(&games, |g| g.genre.clone(), "Genre");

    println!("\n=== Bestseller Stats by Platform ===");
    analysis::analyze_category(&games, |g| g.platform.clone(), "Platform");

    println!("\n=== Bestseller Stats by Publisher ===");
    analysis::analyze_category(&games, |g| g.publisher.clone(), "Publisher");

    println!("\n=== Bestseller Stats by Year ===");
    analysis::analyze_category(&games, |g| match g.year {
        Some(y) => y.to_string(),
        None => "Unknown".to_string(),
    }, "Year");

    println!("\n=== Predict all Shooter games are bestsellers ===");
    analysis::genre_predictor(&games, "Shooter");

    println!("\n=== Predict all Sports games are bestsellers ===");
    analysis::genre_predictor(&games, "Sports");

    println!("\n=== Yearly Bestseller Trend (games over 1M) ===");
    analysis::yearly_bestseller_trend_with_bar(&games);

    println!("\n=== Top 10 Game Titles by Sales ===");
    analysis::top_games_by_sales(&games, 10);

    println!("\n=== Average Sales by Genre ===");
    analysis::average_sales_by_category(&games, |g| g.genre.clone(), "Genre");

    println!("\n=== Top Publishers by Average Global Sales ===");
    analysis::average_sales_by_category(&games, |g| g.publisher.clone(), "Publisher");
}
