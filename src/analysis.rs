use crate::data::Game;
use std::collections::HashMap;

/// Analyzes bestseller stats grouped by a category field (ex: genre).
pub fn analyze_category<F>(games: &[Game], key_fn: F, label: &str)
where
    F: Fn(&Game) -> String,
{
    let mut totals: HashMap<String, (u32, u32)> = HashMap::new();

    for game in games.iter() {
        let key = key_fn(game);
        let is_bestseller = matches!(game.global_sales, Some(s) if s > 1.0);
        let counter = totals.entry(key).or_insert((0, 0));
        counter.0 += 1;
        if is_bestseller {
            counter.1 += 1;
        }
    }

    let mut sorted: Vec<_> = totals.iter().collect();
    sorted.sort_by(|a, b| b.1.1.cmp(&a.1.1));

    println!("Top 10 {}s by # of Bestsellers:", label);
    for (key, (total, hits)) in sorted.iter().take(10) {
        let percent = *hits as f64 / *total as f64 * 100.0;
        println!("{:<25} {:>4}/{} ({:>5.2}%)", key, hits, total, percent);
    }
    println!("Total unique {}s: {}\n", label, totals.len());
}

/// Computes and prints average sales per category (ex: genre or publisher).
pub fn average_sales_by_category<F>(games: &[Game], key_fn: F, label: &str)
where
    F: Fn(&Game) -> String,
{
    let mut totals: HashMap<String, (u32, f64)> = HashMap::new();

    for game in games.iter() {
        let key = key_fn(game);
        if let Some(sales) = game.global_sales {
            let counter = totals.entry(key).or_insert((0, 0.0));
            counter.0 += 1;
            counter.1 += sales;
        }
    }

    let mut sorted: Vec<_> = totals.iter().collect();
    sorted.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());

    println!("Top 10 {}s by Average Global Sales:", label);
    for (key, (count, total)) in sorted.iter().take(10) {
        let avg = *total / *count as f64;
        println!("{:<25} {:.2}M avg over {} games", key, avg, count);
    }
    println!("Total unique {}s: {}\n", label, totals.len());
}

/// Pedict all games of a given genre that are bestsellers.
pub fn genre_predictor(games: &[Game], genre: &str) {
    let mut tp = 0;
    let mut fp = 0;
    let mut tn = 0;
    let mut fn_ = 0;

    for game in games.iter() {
        let actual = matches!(game.global_sales, Some(s) if s > 1.0);
        let predicted = game.genre == genre;

        match (predicted, actual) {
            (true, true) => tp += 1,
            (true, false) => fp += 1,
            (false, true) => fn_ += 1,
            (false, false) => tn += 1,
        }
    }

    println!("Confusion Matrix for genre '{}':", genre);
    println!("TP: {}, FP: {}", tp, fp);
    println!("FN: {}, TN: {}", fn_, tn);

    let accuracy = (tp + tn) as f64 / (tp + fp + tn + fn_) as f64 * 100.0;
    println!("Accuracy: {:.2}%\n", accuracy);
}

/// Lists the top N best-selling games.
pub fn top_games_by_sales(games: &[Game], n: usize) {
    let mut filtered: Vec<_> = games
        .iter()
        .filter_map(|g| g.global_sales.map(|s| (g.name.clone(), s)))
        .collect();

    filtered.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (i, (name, sales)) in filtered.iter().take(n).enumerate() {
        println!("{}. {} - {:.2}M", i + 1, name, sales);
    }
}

/// Visual bar chart output for bestsellers by year.
pub fn yearly_bestseller_trend_with_bar(games: &[Game]) {
    let mut year_map: HashMap<u32, u32> = HashMap::new();

    for game in games.iter() {
        if let (Some(year), Some(sales)) = (game.year, game.global_sales) {
            if sales > 1.0 {
                *year_map.entry(year as u32).or_insert(0) += 1;
            }
        }
    }

    let mut sorted: Vec<_> = year_map.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    for (year, count) in sorted {
        let bar = "█".repeat((*count / 2).min(50) as usize);
        println!("{:<6}: {:>3} bestsellers | {}", year, count, bar);
    }
}
