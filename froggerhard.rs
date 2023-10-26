use std::io::stdin;

#[derive(Clone, Debug, PartialEq, Eq)]
enum VisitState {
    Unseen,
    InProgress,
    Seen,
}
use VisitState::*;

#[derive(Clone, Debug)]
struct Node {
    visited: VisitState,
    val: usize,
    sum: usize,
    term: bool,
    // Only one outgoing edge
    adj: Vec<usize>,
}

fn dfs(graph: &mut [Node], cycle: &mut Vec<usize>, start: usize) -> Option<usize> {
    graph[start].term = true;
    match graph[start].visited {
        InProgress => return Some(start),
        Seen => return None,
        _ => {}
    }

    graph[start].visited = InProgress;
    let mut val = None;

    for adj in 0..graph[start].adj.len() {
        let adj = graph[start].adj[adj];
        if let Some(end) = dfs(graph, cycle, adj) {
            cycle.push(start);
            if end == start {
                continue;
            }
            val = Some(end);
        } else {
            graph[start].val += graph[adj].val;
            graph[start].sum += graph[adj].sum;
            graph[adj].term = false;
        }
    }
    graph[start].sum += graph[start].val;

    graph[start].visited = Seen;
    val
}

fn main() {
    let board: Vec<_> = stdin()
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let mut graph: Vec<_> = std::iter::repeat(Node {
        visited: Unseen,
        val: 1,
        sum: 0,
        term: false,
        adj: Vec::new(),
    })
    .take(board.len())
    .collect();

    (0..board.len()).for_each(|i| {
        let next = i as i64 + board[i];
        if 0 <= next && next < graph.len() as i64 {
            graph[next as usize].adj.push(i);
        }
    });
    let mut winning_instances = 0;

    for i in 0..graph.len() {
        let mut cycle = Vec::new();
        if graph[i].visited != Unseen {
            continue;
        }
        dfs(&mut graph, &mut cycle, i);
        for &node in &cycle {
            graph[node].visited = Unseen;
            graph[node].term = false;
        }

        let mut connected = 0;
        for &node in &cycle {
            for adj in 0..graph[node].adj.len() {
                let adj = graph[node].adj[adj];
                if graph[adj].visited == Unseen {
                    graph[adj].visited = Seen;
                    continue;
                }
                graph[adj].term = true;
                connected += graph[adj].val;
            }
        }
        winning_instances += (connected + cycle.len()) * cycle.len();
    }

    for node in graph.iter() {
        if node.term {
            winning_instances += node.sum;
        }
    }
    println!("{}", winning_instances);
}
