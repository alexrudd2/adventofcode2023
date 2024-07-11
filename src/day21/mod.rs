#[path = "../util/util.rs"]
mod util;

use petgraph::algo::dijkstra;
use petgraph::graph::UnGraph;
use std::collections::HashMap;

pub fn main() {
    println!("Day 21: Step Counter");
    let plots = util::read_input_array("src/day21/input.txt");
    let rows = plots.shape()[0];
    let cols = plots.shape()[1];
    let mut start_row = 0;
    let mut start_col = 0;
    let max_steps = 64;

    let mut graph: UnGraph<i32, i32> = UnGraph::new_undirected();
    let mut nodes = HashMap::new();
    let mut indices = HashMap::new();

    for row in 0..rows {
        for col in 0..cols {
            if plots[[row, col]] == '#' {
                continue;
            }
            if plots[[row, col]] == 'S' {
                start_row = row;
                start_col = col;
            }
            let node = graph.add_node(0);
            nodes.insert((row, col), node);
            indices.insert(node, (row, col));

            if row > 0 && plots[[row - 1, col]] != '#' {
                let above_node = nodes.get(&(row - 1, col)).unwrap();
                graph.add_edge(node, *above_node, 1);
            }
            if col > 0 && plots[[row, col - 1]] != '#' {
                let left_node = nodes.get(&(row, col - 1)).unwrap();
                graph.add_edge(node, *left_node, 1);
            }
        }
    }

    let start_node = *nodes.get(&(start_row, start_col)).unwrap();
    let costs = dijkstra(&graph, start_node, None, |_| 1);

    // for row in 0..rows {
    //     for col in 0..cols {
    //         let node = nodes.get(&(row, col));
    //         if node.is_some()
    //             && costs.contains_key(node.unwrap())
    //             && *costs.get(node.unwrap()).unwrap() <= 6
    //         {
    //             print!("O");
    //         } else {
    //             let c = plots[[row, col]];
    //             print!("{c}");
    //         }
    //     }
    //     print!("\n");
    // }
    let total = costs.iter().fold(0, |acc, x| {
        if *x.1 <= max_steps && *x.1 % 2 == 0 {
            acc + 1
        } else {
            acc
        }
    });
    println!("With 64 steps, the elf can reach {total} different plots.");
}
