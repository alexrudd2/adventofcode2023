#[path = "../util/util.rs"]
mod util;

use petgraph::graph::Graph;
use petgraph::algo::dijkstra;
use petgraph::visit::EdgeRef;
use std::collections::{HashSet, HashMap};
use binary_heap_plus::BinaryHeap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Dummy,
    R,
    D,
    L,
    U,
}

pub fn main() {
    println!("Day 17: Clumsy Crucible");
    let lava_map = util::read_input_array("src/day17/input.txt");
    println!("{lava_map:?}");

    let rows = lava_map.shape()[0];
    let cols = lava_map.shape()[1];


    let mut graph = Graph::<i32, Direction>::new();
    let mut vertices = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let vertex = graph.add_node(lava_map[[i,j]].to_digit(10).unwrap() as i32);
            vertices.push(vertex);
        }
    }

    // Iterate over the vertices and add edges for adjacent points
    let mut visited = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let current_vertex = vertices[i * cols + j];

            // if i > 0 && !visited.contains(&vertices[(i - 1) * cols + j]) {  // up
            if i > 0 {
                graph.add_edge(current_vertex, vertices[(i - 1) * cols + j], Direction::U);
            }
            // if i < rows - 1 && !visited.contains(&vertices[(i + 1) * cols + j]) { // down
            if i < rows - 1 {
                graph.add_edge(current_vertex, vertices[(i + 1) * cols + j], Direction::D);
            }
            // if j > 0 && !visited.contains(&vertices[i * cols + j - 1]) { // left
            if j > 0 {
                graph.add_edge(current_vertex, vertices[i * cols + j - 1], Direction::L);
            }
            // if j < cols - 1 && !visited.contains(&vertices[i * cols + j + 1]) { // right
            if j < cols - 1 {
                graph.add_edge(current_vertex, vertices[i * cols + j + 1], Direction::R);
            }

            // Mark the current vertex as visited
            visited.push(current_vertex);
        }
    }

    // println!("Graph: {:?}", graph);

    // for node in graph.node_indices() {
    //     println!("Node {}: Weight = {}", node.index(), *graph.node_weight(node).unwrap());

    //     let neighbors: Vec<_> = graph
    //         .neighbors(node)
    //         .map(|neighbor| *graph.node_weight(neighbor).unwrap())
    //         .collect();
    //     println!("  Neighbors: {:?}", neighbors);
    // }

    let start_node = graph.node_indices().next().unwrap();
    let end_node = graph.node_indices().max().unwrap();

    // Use Dijkstra's algorithm to find the lowest traversal cost
    let distances = dijkstra(&graph,
                                                      start_node,
                                                 Some(end_node),  
                                            |edge| *graph.node_weight(edge.target()).unwrap());

    let lowest_cost = distances.get(&end_node).unwrap();
    println!("lowest cost: {lowest_cost}");

    
    let distances = custom_dijkstra(&graph, start_node, Some(end_node));

    let mut sorted = Vec::new();
    for d in &distances {
        sorted.push(d);
    }
    sorted.sort_by(|(a, _), (b, _)| a.0.index().cmp(&b.0.index()));
    // Print distances and traversed edges for each node
    for (node, &distance) in &sorted {
        println!("Node {}({:?}x{}): Distance: {}", node.0.index(), node.1, node.2, distance);
    }
    // let lowest_cost = distances.last.unwrap();
    // println!("lowest cost: {lowest_cost}");
    // for (node, distance) in &sorted {
    //     println!("Node {}: Distance = {}", node.index(), distance);
    // }

    // 928 is too high

}
fn custom_dijkstra<G>(
    graph: G,
    start: G::NodeId,
    goal: Option<G::NodeId>,
) -> HashMap<(G::NodeId, Direction, i8), i32>
where
    G: petgraph::visit::IntoEdges + petgraph::visit::Visitable + petgraph::data::DataMap,
    <G as petgraph::visit::Data>::NodeWeight: Into<i32> + Copy,
    <G as petgraph::visit::Data>::EdgeWeight: Into<Direction> + Copy,
    G::NodeId: Eq + std::hash::Hash + core::fmt::Debug + Ord,
    G::EdgeId: core::fmt::Debug + std::cmp::Eq + std::hash::Hash,
    G::EdgeRef: core::fmt::Debug
{
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new_min();

    distances.insert((start, Direction::Dummy, 0), 0);
    queue.push(((start, Direction::Dummy, 0), 0));

    while let Some((node, dist)) = queue.pop() {
        // if visited.contains(&node) {
        //     continue;
        // }

        if Some(node.0) == goal {
            break;
        }
        visited.insert(node);

        // println!("Now at {node:?} with cost {dist}");
        let prev_dir = node.1;
        let prev_seq = node.2;
        for edge in graph.edges(node.0) {
            let direction = edge.weight().clone().into();
            let mut target = (edge.target(), direction, prev_seq + 1);

            // let w = graph.node_weight(edge.target()).unwrap().clone().into();

            // let mut existing_cost = 1000;
            // if distances.contains_key(&target) {
            //     existing_cost = *distances.get(&target).unwrap();
            // }
            // println!("Check {target:?} ({direction:?}) with cost {existing_cost}");

            // no 4 in a row
            if direction == prev_dir {
                if prev_seq == 3 {
                    // println!(" 4 in a row, don't push!");
                    continue;
                }
            } else {
                target.2 = 1;
                // no backtrack
                if direction == Direction::U && prev_dir == Direction::D
                    || direction == Direction::D && prev_dir == Direction::U
                    || direction == Direction::L && prev_dir == Direction::R
                    || direction == Direction::R && prev_dir == Direction::L {
                    // println!("Backtrack, don't push!");
                    continue;
                }
            }
            let cost = dist + graph.node_weight(edge.target()).unwrap().clone().into();
            if !distances.contains_key(&target) || cost < *distances.get(&target).unwrap() {
                distances.insert(target, cost);
                queue.push((target, cost));
                // println!("PUSH {target:?}, {cost}");
            }
        }
        // print!("\n");
    }

    distances
}