extern crate rand;

use rand::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::usize;

fn contract(graph: &mut Vec<(String, String)>) -> Vec<(String, String)> {
    let (u, v) = choose_edge(graph);
    let mut aux = vec![];
    let w = format!("{},{}", u, v);
    for (x, y) in graph.iter() {
        let mut rx = x;
        let mut ry = y;
        if (u == *x) || (v == *x) {
            rx = &w;
        }
        if (u == *y) || (v == *y) {
            ry = &w;
        }
        if *rx < *ry {
            aux.push((rx.clone(), ry.clone()));
        }
        if *rx > *ry {
            aux.push((ry.clone(), rx.clone()));
        }
    }
    aux
}

fn choose_edge(graph: &mut Vec<(String, String)>) -> (String, String) {
    let mut rng = thread_rng();
    let graph_len = graph.len();
    let mut edge_idx = 0;
    if graph_len > 0 {
        edge_idx = rng.gen_range(0, graph.len());
    }
    graph.iter().nth(edge_idx).unwrap().clone()
}

fn mincut(graph: &Vec<(String, String)>, n: i32) -> ((String,String), usize){
    let mut cost = usize::MAX;
    let mut components = (String::new(), String::new());
    for _ in 0..n * n {
        let mut aux = graph.clone();
        while HashSet::<(String, String)>::from_iter(aux.iter().cloned()).len() > 1 {
            aux = contract(&mut aux);
            let aux_len = aux.len();
            if aux_len < cost {
                components = aux[0].clone();
                cost = aux_len;
            }
        }
    }
    (components, cost)
}

fn main() {
    let graph = vec![
        (String::from("A1"), String::from("A2")),
        (String::from("A1"), String::from("A3")),
        (String::from("A1"), String::from("A4")),
        (String::from("A2"), String::from("A3")),
        (String::from("A2"), String::from("A5")),
        (String::from("A3"), String::from("A4")),
        (String::from("A4"), String::from("A5")),
    ];
    let (components, cost) = mincut(&graph, 10);
    println!("best cut: {}", cost);
    println!("component#1:{}", components.0);
    println!("component#2:{}", components.1);
}
