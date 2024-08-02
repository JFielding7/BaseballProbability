#![allow(non_snake_case)]

use crate::game_simulation::is_comeback;
use crate::league_averages::get_league_averages;

mod league_averages;
mod game_simulation;

fn main() {
    // let games = game_simulation::simple_games_simulation(2024, 1000000).unwrap();
    // let mut i = 0;
    // for game in game_simulation::get_comeback_wins(games, 1) {
    //     // game.box_score();
    //     i += 1;
    //     // break;
    // }
    // println!("{i}")
    const TOTAL_PROB_RANGE: usize = 2625;

    let averages = get_league_averages(true).unwrap();
    let mut rng = rand::thread_rng();
    let run_prob = (*averages.get(&2024).unwrap().get("R").unwrap() * 100.0) as usize;

    let mut i = 0;
    loop {
        i += 1;
        let game = game_simulation::simulate_simple_game(&mut rng, run_prob, run_prob + TOTAL_PROB_RANGE);
        if is_comeback(&game, 13) {
            game.display_box_score();
            break;
        }
    }
    println!("{i}");
}