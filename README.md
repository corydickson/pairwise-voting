# Ranked Pairs Voting

## Definition

This is a Rust implementation of T.N. Tideman's Ranked Pairs algorithm, with test cases provided by the example given in the [Wikipedia article](https://en.wikipedia.org/wiki/Ranked_pairs). The goal of the ranked pair election/ballot is to select a winner amongst a list of preferences. It generates a sorted list of pairs and ensures that at least one candidate wins
amongst all the possible choices. The procedure first tallies and compares each pair of candidates (choices) and determines a winner. Then these majorities are sorted, in descending
order of magnitude from greatest to least. A "lock-in" graph is created, where the largest magnitude majority is considered first to create a DAG in which the source (i.e. the node with no incoming edges) is the winner. In the possiblity of a tie, where there is one or more source nodes, a winner is selected at random amongst them.

## Citations

- Tideman, T.N. Independence of clones as a criterion for voting rules. Soc Choice Welfare 4, 185–206 (1987). https://doi.org/10.1007/BF00433944
- Zavist, T.M., Tideman, T.N. Complete independence of clones in the ranked pairs rule. Soc Choice Welfare 6, 167–173 (1989). https://doi.org/10.1007/BF00303170

## Quickstart

`cargo build`
`cargo run -- examples/wiki.json` to see full working example with announced winner.

To run unit tests: `cargo test`
