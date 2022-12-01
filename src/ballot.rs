use::std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub type Candidate = String;
pub type Candidates = Vec<Candidate>;
pub type Ballots = Vec<Preference>;
pub type Preference = Candidates;

#[derive(Debug, Serialize, Deserialize)]
struct Ballot {
    preference: Preference,
    count: u8
}

#[derive(Serialize, Deserialize)]
struct Election {
    candidates: Candidates,
    ballots: Vec<Ballot>,
}

pub fn parse_election_json(json_str: &str) -> Result<(Candidates, Ballots)> {
    let election: Election = serde_json::from_str(json_str)?;
    let mut ballots = Vec::new();

    for b in election.ballots.iter() {
        if b.preference.len() != election.candidates.len() {
            panic!("Expected number of candidates on the ballot to be the same");
        }

        let mut unique = HashSet::new();
        if !b.preference.iter().all(|candidate| unique.insert(candidate) && election.candidates.contains(candidate)) {
            panic!("Expected ballot to only contain candidates defined: {:?}, received {:?}", election.candidates, b.preference);
        }

        for _ in 0..b.count {
            ballots.push(b.preference.to_owned());
        }
    }

    Ok((election.candidates, ballots))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fails_candidates_mismatch() {
        let data = r#"
        {
            "candidates": ["x", "y"],
            "ballots": [
                {
                    "preference": ["w", "x", "z", "y"],
                    "count": 7
                }
            ]
        }"#;

        let (_, _ballots) = parse_election_json(data).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_ballots_mismatch() {
        let data = r#"
        {
            "candidates": ["x", "y"],
            "ballots": [
                {
                    "preference": ["w", "z"],
                    "count": 7
                }
            ]
        }"#;

        let (_, _ballots) = parse_election_json(data).unwrap();
    }

    #[test]
    fn correct_num_ballots() {
        let data = r#"
        {
            "candidates": ["x", "y"],
            "ballots": [
                {
                    "preference": ["y", "x"],
                    "count": 7
                }
            ]
        }"#;

        let (_, ballots) = parse_election_json(data).unwrap();
        assert_eq!(ballots.len(), 7);
    }
}
