use std::{collections::HashSet, io::stdin};

#[derive(Debug)]
struct Web {
    cycle: usize,
    chains: Vec<usize>,
    nodes: Vec<usize>,
}

impl Web {
    fn count(&self) -> usize {
        let total = self.cycle + self.chains.iter().clone().sum::<usize>();
        let m_in_cycle = self.cycle * total;

        let m_in_chain = self.chains.iter().map(count_chain).sum::<usize>();

        m_in_cycle + m_in_chain
    }

    fn add_chain(&mut self, v: impl Iterator<Item = usize>) {
        let mut count = 0;
        for node in v {
            let idx = match self.nodes.binary_search(&node) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
            self.nodes.insert(idx, node);
            count += 1;
        }
        self.chains.push(count);
    }

    fn contains(&self, start_idx: usize) -> bool {
        self.nodes.binary_search(&start_idx).is_ok()
    }
}

fn count_chain(length: &usize) -> usize {
    (length * (length + 1)) / 2
}

fn main() {
    let board: Vec<_> = stdin()
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut visited = Vec::with_capacity(board.len());
    let mut seen = HashSet::with_capacity(board.len());
    let size = f32::sqrt(board.len() as f32) as usize;
    let mut webs: Vec<Web> = Vec::with_capacity(size);
    let mut chains = Vec::with_capacity(size);

    // Construct/Update a web each iteration

    for i in 0..board.len() {
        if seen.contains(&(i as i32)) {
            continue;
        }

        let mut pos = i as i32;
        let new_visited = visited.len();

        loop {
            // Cycle

            if seen.contains(&pos) {
                if let Some(start_idx) = visited.iter().position(|val| *val == pos) {
                    // => Already added, find it and update with new chain

                    if start_idx < new_visited {
                        for existing in webs.iter_mut() {
                            if existing.contains(start_idx) {
                                existing.add_chain(
                                    visited[new_visited..].iter().map(|n| (*n) as usize),
                                );
                                break;
                            }
                        }
                    } else {
                        let cycle_len = visited.len() - start_idx;
                        let first_chain_len = start_idx - new_visited;
                        let mut nodes: Vec<_> = visited[new_visited..]
                            .iter()
                            .map(|n| (*n) as usize)
                            .collect();
                        nodes.sort_unstable();
                        webs.push(Web {
                            cycle: cycle_len,
                            chains: vec![first_chain_len],
                            nodes,
                        });
                    }
                    break;
                }
            }

            if 0 <= pos && pos < board.len() as i32 {
                visited.push(pos);
                seen.insert(pos);
            }

            match board.get(pos as usize) {
                Some(delta) => pos += delta,
                // We fell off => Chain & Not part of a Cycle
                None => {
                    let nodes = visited.len() - new_visited;
                    chains.push(nodes);
                    visited.truncate(visited.len() - nodes);
                    break;
                }
            }
        }
    }

    println!(
        "{}",
        webs.iter().map(Web::count).sum::<usize>() + chains.iter().map(count_chain).sum::<usize>()
    );
}
