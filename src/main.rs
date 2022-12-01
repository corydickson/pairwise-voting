use std::fs::read_to_string;
use std::env::args;
use pairwise_voting::ballot::parse_election_json;
use pairwise_voting::tally::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Please supply a filename for an election see: examples/wiki.json")
    }

    let file_path = &args[1];

    let json = read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (candidates, ballots) = parse_election_json(json.as_str()).unwrap();

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

