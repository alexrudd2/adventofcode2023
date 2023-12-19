#[path = "../util/util.rs"]
mod util;

use petgraph::graph::{ DiGraph, NodeIndex };
use petgraph::algo::dijkstra;
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

pub fn main() {
    println!("Day 17: Clumsy Crucible");
    let lava_map = util::read_input_array("src/day17/input.txt");
    // println!("{lava_map:?}");

    let directions = vec!['D', 'U', 'L', 'R', 'S']; // down up left right start
    fn dir_i(ch: char) -> usize {
        match ch {
            'D' => 0,
            'U' => 1,
            'L' => 2,
            'R' => 3,
            'S' => 4,
            _ => panic!()
        }
    }
    let rows = lava_map.shape()[0];
    let cols = lava_map.shape()[1];
    let dirs = 4 as usize;
    let seqs = 3 as usize;


    let mut graph = DiGraph::new();
    let mut indices: HashMap<(usize, usize, usize, usize), NodeIndex> = HashMap::new();
    let mut vertices: HashMap<NodeIndex, (usize, usize, usize, usize)> = HashMap::new();

    for row in 0..rows {
        for col in 0..cols {
            for dir in 0..dirs {
                for seq in 0..seqs {
                    let vertex = graph.add_node(lava_map[[row, col]].to_digit(10).unwrap() as i32);
                    indices.insert((row, col, dir, seq), vertex);
                    vertices.insert(vertex, (row, col, dir, seq));
                }
            }
        }
    }

    // Iterate over the vertices and add edges for adjacent points
    let mut visited: HashSet<NodeIndex> = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            for dir in 0..dirs {
                for seq in 0..seqs {
                    let current_index = *indices.get(&(row, col, dir, seq)).unwrap();
                    if row > 0 && directions[dir] != 'D' { // no backtracking the direction we came from
                        if directions[dir] == 'U' && seq < 2 { // no 4-in-a-row
                            let target = indices.get(&(row - 1, col, dir_i('U'), seq + 1));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        } else if directions[dir] != 'U' {
                            let target = indices.get(&(row - 1, col, dir_i('U'), 0));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        }
                    }
                    if row < rows - 1 && directions[dir] != 'U' {
                        if directions[dir] == 'D' && seq < 2 {  // no 4-in-a-row
                            let target = indices.get(&(row + 1, col, dir_i('D'), seq + 1));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        } else if directions[dir] != 'D'{
                            let target = indices.get(&(row + 1, col, dir_i('D'), 0));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        }
                    }
                    if col > 0 && directions[dir] != 'R' {
                        if directions[dir] == 'L' && seq < 2 { // no 4-in-a-row
                            let target = indices.get(&(row, col - 1, dir_i('L'), seq + 1));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        } else if directions[dir] != 'L' {
                            let target = indices.get(&(row, col - 1, dir_i('L'), 0));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        }
                    }
                    if col < cols - 1 && directions[dir] != 'L' {
                        if directions[dir] == 'R' && seq < 2 { // no 4-in-a-row
                            let target = indices.get(&(row, col + 1, dir_i('R'), seq + 1));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        } else if directions[dir] != 'R' {
                            let target = indices.get(&(row, col + 1, dir_i('R'), 0));
                            graph.add_edge(current_index, *target.unwrap(), ());
                        }
                    }
                    visited.insert(current_index);
                }
            }
        }
    }

    // for node in graph.node_indices() {
    //     print!("Node {} | ({}, {}, {}, {}x) | {} has ",
    //              node.index(),
    //              vertices.get(&node).unwrap().0,
    //              vertices.get(&node).unwrap().1,
    //              directions[vertices.get(&node).unwrap().2],
    //              vertices.get(&node).unwrap().3,
    //              *graph.node_weight(node).unwrap());

    //     let neighbors: Vec<_> = graph
    //         .neighbors(node)
    //         .map(|neighbor| (*graph.node_weight(neighbor).unwrap(), directions[vertices.get(&neighbor).unwrap().2]))
    //         .collect();
    //     println!("neighbors: {:?}", neighbors);
    // }

    let start_node = *indices.get(&(0, 0, dir_i('D'), 0 as usize)).unwrap();
    let costs = dijkstra(&graph, start_node, None,
                                        |edge| *graph.node_weight(edge.target()).unwrap());
    let end_node1 = *indices.get(&(rows - 1, cols - 1, dir_i('D'), 0 as usize)).unwrap();
    let end_node2 = *indices.get(&(rows - 1, cols - 1, dir_i('R'), 0 as usize)).unwrap();
    let lowest_cost1 = costs.get(&end_node1).unwrap();
    let lowest_cost2 = costs.get(&end_node2).unwrap();
    let lowest_cost = std::cmp::min(lowest_cost1, lowest_cost2);
    println!("The least heat loss possible is {lowest_cost}.");

}