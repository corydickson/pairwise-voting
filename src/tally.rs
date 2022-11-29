use crate::ballot::*;
use std::collections::HashMap;

type Pair = (Candidate, Candidate);
type Score = i32;

pub fn tally(ballots: Vec<Ballot>) -> HashMap <Pair, Score> {
    let mut scores = HashMap::new();

    for b in ballots {
        for (rank, candidate) in b.iter().enumerate() {
            for i in 0..b.len() {
                if candidate == &b[i] {
                    scores.insert((candidate.to_owned(), b[i].to_owned()), 0);
                }
                else {
                    let count = scores.entry((candidate.to_owned(), b[i].to_owned())).or_insert(0);
                    if rank < i {
                        *count += 1;
                    }
                    else {
                        *count -= 1;
                    }
                }
            }
        }
    }

    return scores;
}

pub fn sort_majorities(scores: &HashMap<Pair, Score>) -> Vec<(&Pair, &Score)> {
    let mut majorities: Vec<(&Pair, &Score)> = scores.iter().collect();
    majorities.retain(|m| m.1 > &0);
    majorities.sort_by(|a, b| b.1.cmp(a.1));

    return majorities;
}

fn reachable(graph: &HashMap<&String, Vec<&String>>, from: &String, to: &String) -> bool {
    let edges = graph.get(from).unwrap();
    for &e in edges {
        return &e == &to || reachable(graph, &e, to);
    }

    return false;
}

pub fn lock<'a>(candidates: &'a Vec<Candidate>, majorities: Vec<(&Pair, &Score)>) -> Vec<&'a Candidate> {
    let mut graph = HashMap::new();

    for c in candidates.iter() {
        let edges: Vec<&String> = Vec::new();
        graph.insert(c, edges);
    }

    for m in majorities.iter() {
        let candidate_x = &m.0.0;
        let candidate_y = &m.0.1;

        if !reachable(&graph, candidate_y, candidate_x) {
            let edges = graph.entry(&candidate_x).or_insert(vec![]);
            edges.push(candidate_y);
        }
    }

    let sources = candidates.into_iter().filter( |c| {
        return candidates.into_iter().all(|n| {
            let adj = graph.get(&n).unwrap();
            return !adj.contains(&c);
        });
    }).collect::<Vec<_>>();

    return sources;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tally_count() {
        let ballots = create_wikipedia_ballots();
        let scores = tally(ballots);

        assert_eq!(scores.get(&(String::from("w"), String::from("w"))).unwrap(), &0);
        assert_eq!(scores.get(&(String::from("w"), String::from("x"))).unwrap(), &9);
        assert_eq!(scores.get(&(String::from("w"), String::from("y"))).unwrap(), &1);
        assert_eq!(scores.get(&(String::from("w"), String::from("z"))).unwrap(), &-7);

        assert_eq!(scores.get(&(String::from("x"), String::from("w"))).unwrap(), &-9);
        assert_eq!(scores.get(&(String::from("x"), String::from("x"))).unwrap(), &0);
        assert_eq!(scores.get(&(String::from("x"), String::from("y"))).unwrap(), &5);
        assert_eq!(scores.get(&(String::from("x"), String::from("z"))).unwrap(), &11);

        assert_eq!(scores.get(&(String::from("y"), String::from("w"))).unwrap(), &-1);
        assert_eq!(scores.get(&(String::from("y"), String::from("x"))).unwrap(), &-5);
        assert_eq!(scores.get(&(String::from("y"), String::from("y"))).unwrap(), &0);
        assert_eq!(scores.get(&(String::from("y"), String::from("z"))).unwrap(), &3);

        assert_eq!(scores.get(&(String::from("z"), String::from("w"))).unwrap(), &7);
        assert_eq!(scores.get(&(String::from("z"), String::from("x"))).unwrap(), &-11);
        assert_eq!(scores.get(&(String::from("z"), String::from("y"))).unwrap(), &-3);
        assert_eq!(scores.get(&(String::from("z"), String::from("z"))).unwrap(), &0);
    }

    #[test]
    fn majorities_sorted_correctly() {
        let ballots = create_wikipedia_ballots();
        let scores = tally(ballots);
        let majorities = sort_majorities(&scores);
        let correct_ordered_scores = vec![11,9,7,5,3,1];

        for (i, m) in majorities.into_iter().enumerate() {
            let score = m.1;
            assert_eq!(score, correct_ordered_scores.get(i).unwrap())
        }
    }

    #[test]
    fn locked_graph_no_cycles() {
        let candidates = vec![String::from("w"),String::from("x"), String::from("z"), String::from("y")];
        let ballots = create_wikipedia_ballots();
        let results = tally(ballots);
        let majorities = sort_majorities(&results);
        let sources = lock(&candidates, majorities);
        assert_eq!(sources.get(0).unwrap(), &candidates.get(0).unwrap());
    }
}

