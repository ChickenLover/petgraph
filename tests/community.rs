use itertools::Itertools;
use petgraph::algo::community::girvan_newman;
use petgraph::data::Build;
use petgraph::{prelude::*, Graph};
use std::collections::{HashSet, HashMap};
use std::fs;
use std::str::FromStr;


#[test]
fn girvan_newman_test() {
    let mut graph: Graph<(), ()> = Graph::new();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    let f = graph.add_node(());
    let g = graph.add_node(());
    let h = graph.add_node(());

    graph.extend_with_edges(&[
        (a, b),
        (b, c),
        (c, d),
        (d, a),
        (e, f),
        (b, e),
        (f, g),
        (g, h),
        (h, e),
    ]);
    // a ----- b ----- e ----- f
    // |       |       |       |
    // |       |       |       |
    // d ----- c       h ----- g

    let mut first_community = HashSet::new();
    first_community.insert(a);
    first_community.insert(b);
    first_community.insert(c);
    first_community.insert(d);

    let mut second_community = HashSet::new();
    second_community.insert(e);
    second_community.insert(f);
    second_community.insert(g);
    second_community.insert(h);

    let expected_res = (vec![first_community, second_community], vec![(b, e)]);
    let res = girvan_newman(&graph, 1);

    assert_eq!(res, expected_res);
}

#[test]
fn girvan_newman_big_test() {
    let graph_contents = String::from_utf8(fs::read("tests/res/communities.txt").unwrap()).unwrap();
    let mut graph: Graph<(), ()> = Graph::new();

    let mut node_map = HashMap::new();
    for line in graph_contents.split("\n") {
        let parts: Vec<&str> = line.split(" ").into_iter().collect();
        if parts.first().unwrap() == &"Node" {
            node_map.insert(
                u32::from_str(parts.get(1).unwrap()).unwrap(),
                graph.add_node(())
            );
        } else if parts.first().unwrap() == &"Edge" {
            let a = node_map.get(
                &u32::from_str(parts.get(1).unwrap()).unwrap()
            ).unwrap();
            let b = node_map.get(
                &u32::from_str(parts.get(2).unwrap()).unwrap()
            ).unwrap();
            graph.add_edge(*a, *b, ());
        }
    }

    let (communities, edges_removed) = girvan_newman(&graph, 2);
    let first_community = communities.first().unwrap();
    let mut second_community = HashSet::<NodeIndex>::new();
    for com in &communities[1..] {
        second_community.extend(com);
    }

    println!("{}, {}", first_community.len(), second_community.len());

    println!("{:?}", first_community);
    println!("{:?}", second_community);
}