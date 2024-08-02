use std::cmp::{max, min};
use rand::Rng;
use rand::rngs::ThreadRng;
use term_table::row::Row;
use term_table::Table;
use crate::league_averages::get_league_averages;

pub struct Game {
    pub away_score: i32,
    pub home_score: i32,
    innings: Vec<Inning>
}

struct Inning {
    away_score: i32,
    home_score: i32,
}

impl Game {
    pub fn to_string(&self) -> String {
        format!("{} {}", self.away_score, self.home_score)
    }

    pub fn display_box_score(&self) {
        let mut innings = vec![];
        let mut away_scores = vec![];
        let mut home_scores = vec![];

        let mut i = 1;
        self.innings.iter().for_each(|inning| {
            innings.push(i.to_string());
            away_scores.push(inning.away_score);
            home_scores.push(inning.home_score);
            i += 1;
        });
        innings.push("R".to_string());
        away_scores.push(self.away_score);
        home_scores.push(self.home_score);
        println!("{}", Table::builder().rows(vec![Row::new(innings), Row::new(away_scores), Row::new(home_scores)]).build().render());
    }
}

fn simulate_inning_half(rng: &mut ThreadRng, run_prob_range: usize, total_prob_range: usize) -> i32 {
    let mut outs = 3;
    let mut runs = 0;
    while outs > 0 {
        let result: usize = rng.gen_range(0..total_prob_range);
        if result < run_prob_range {
            runs += 1;
        }
        else {
            outs -= 1;
        }
    }
    runs
}

pub(crate) fn simulate_simple_game(rng: &mut ThreadRng, run_prob_range: usize, total_prob_range: usize) -> Game {
    let mut away_score: i32 = 0;
    let mut home_score: i32 = 0;
    let mut innings = vec![];
    let mut inning = 0;
    while inning < 9 || away_score == home_score {
        let away_inning_score = simulate_inning_half(rng, run_prob_range, total_prob_range);
        let mut home_inning_score = 0;
        if inning < 8 || home_score <= away_score {
            home_inning_score = simulate_inning_half(rng, run_prob_range, total_prob_range);
        }
        innings.push(Inning {away_score: away_inning_score, home_score: home_inning_score});
        away_score += away_inning_score;
        home_score += home_inning_score;
        inning += 1;
    }
    Game {away_score, home_score, innings }
}

pub(crate) fn simple_games_simulation(year: u32, n_games: u64) -> reqwest::Result<Vec<Game>> {
    const TOTAL_PROB_RANGE: usize = 2625;

    let averages = get_league_averages(true)?;
    let mut rng = rand::thread_rng();
    let run_prob = (*averages.get(&year).unwrap().get("R").unwrap() * 100.0) as usize;
    let mut total_runs = 0;
    let mut games = vec![];

    for _ in 0..n_games {
        let game = simulate_simple_game(&mut rng, run_prob, TOTAL_PROB_RANGE + run_prob);
        total_runs += game.away_score + game.home_score;
        games.push(game);
    }
    println!("{:.4}", total_runs as f64 / n_games as f64);
    Ok(games)
}

pub(crate) fn is_comeback(game: &Game, threshold: i32) -> bool {
    let mut diff = 0;
    let mut away_deficit = 0;
    let mut home_deficit = 0;
    for inning in &game.innings {
        diff -= inning.away_score;
        home_deficit = min(home_deficit, diff);
        diff += inning.home_score;
        away_deficit = max(away_deficit, diff);
    }
    (away_deficit >= threshold && game.away_score > game.home_score) ||
    (-home_deficit >= threshold && game.away_score < game.home_score)
}

pub(crate) fn get_comeback_wins(games: Vec<Game>, threshold: i32) -> Vec<Game> {
    games.into_iter().filter(|game| is_comeback(game, threshold)).collect()
}