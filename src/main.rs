use pairwise_voting::ballot::create_wikipedia_ballots;
use pairwise_voting::tally::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let candidates = vec![String::from("w"),String::from("x"), String::from("z"), String::from("y")];
    let ballots = create_wikipedia_ballots();
    // Execute ranked pairs procedure
    let results = tally(ballots);
    let majorities = sort_majorities(&results);
    let sources = lock(&candidates, majorities);

    if sources.len() == 1 {
        let winner = sources.get(0).unwrap();
        println!("The winner is {}", winner);
    } else {
        println!("There is a tie between candidates: {:?}", sources);
        println!("Randomly selecting winner: {:?}", sources.choose(&mut thread_rng()).unwrap());
    }
}
