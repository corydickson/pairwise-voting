pub type Candidate = String;
pub type Ballot = [Candidate; 4];

pub fn create_wikipedia_ballots() -> Vec<[String; 4]> {
    // Example election provided by: https://en.wikipedia.org/wiki/Ranked_pairs
    let mut ballots: Vec<Ballot> = Vec::new();

    for _i in 0..7 {
        let ballot: [String; 4] = [String::from("w"),String::from("x"), String::from("z"), String::from("y")];

        ballots.push(ballot);
    }
    assert_eq!(ballots.len(), 7);

    for _i in 0..2 {
        let ballot = [String::from("w"),String::from("y"), String::from("x"), String::from("z")];

        ballots.push(ballot);
    }
    assert_eq!(ballots.len(), 9);

    for _i in 0..4 {
        let ballot = [String::from("x"),String::from("y"), String::from("z"), String::from("w")];

        ballots.push(ballot);
    }
    assert_eq!(ballots.len(), 13);

    for _i in 0..5 {
        let ballot = [String::from("x"),String::from("z"), String::from("w"), String::from("y")];

        ballots.push(ballot);
    }
    assert_eq!(ballots.len(), 18);

    for _i in 0..1 {
        let ballot = [String::from("y"),String::from("w"), String::from("x"), String::from("z")];

        ballots.push(ballot);
    }
    assert_eq!(ballots.len(), 19);

    for _i in 0..8 {
        let ballot = [String::from("y"),String::from("z"), String::from("w"), String::from("x")];

        ballots.push(ballot);
    }
    assert_eq!(ballots.len(), 27);

    return ballots;
}

