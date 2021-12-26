use std::time::Instant;

use do_util::priority_queue::PriorityQueue;
use rand::Rng;
use rand::prelude::ThreadRng;

use do_util::priority_queue::{
    ParetoElement,
    GuidedElement,
    util::CartesianParetoElement
};
use do_util::priority_queue::pareto_pq_list::ListParetoFront;
// use do_util::pareto_pq::naive_kd_tree::NaiveKDTreeFront;
// use do_util::pareto_pq::kd_tree::KDTreeFront;

pub fn random_element<const NB_DIM:usize>(rng:&mut ThreadRng) -> CartesianParetoElement<NB_DIM> {
    let mut coords = [0;NB_DIM];
    loop {
        for i in coords.iter_mut().take(NB_DIM) {
            *i = rng.gen::<u32>() % 10000;
        }
        if coords.iter().map(|e| {
            (*e as f64 / u32::MAX as f64).powf(NB_DIM as f64)
        }).sum::<f64>() <= 1. {
            let mut new_coords = [10000 ; NB_DIM];
            for i in 0..NB_DIM {
                new_coords[i] -= coords[i];
            }
            return CartesianParetoElement::new(new_coords);
        }
    }
    
}


pub fn perform_bench<'a, T, Elt:ParetoElement<T>, Front>
(elements:&[Elt], mut front:Front) where
T:Ord+std::fmt::Debug,
Elt:'a + ParetoElement<T>+GuidedElement<T>+Eq+Clone,
Front:PriorityQueue<T, Elt> {
    // inserts elements
    let start_insert = Instant::now();
    for e in elements {
        front.insert(e.clone());
    }
    let time_insert = start_insert.elapsed().as_secs_f32();
    println!("\t{:<5} inserts in {} seconds ({} inserts/s)",
        elements.len(), time_insert, elements.len() as f32/time_insert
    );
    // pop them until no more elements in the front
    let mut nb_pops:usize = 0;
    let start_pop = Instant::now();
    let mut last = front.peek_minimum().unwrap().guide();
    while let Some(e) = front.pop_minimum() {
        nb_pops += 1;
        if last > e.guide() {
            panic!("error: priority queue did not extract a minimum element.");
        }
        last = e.guide();
    }
    let time_pop = start_pop.elapsed().as_secs_f32();
    println!("\t{:<5} pops in {} seconds ({} pops/s)", nb_pops, time_pop, nb_pops as f32/time_pop);
}


pub fn bench_pareto<const NB_DIM:usize>(nb_elts:usize) {
    let mut rng = rand::thread_rng();
    let mut elements:Vec<CartesianParetoElement<NB_DIM>> = Vec::new();
    for _ in 0..nb_elts {
        elements.push(random_element(&mut rng));
    }
    // TEST LIST PARETO
    println!("List structure:");
    let list_front:ListParetoFront<u32,CartesianParetoElement<NB_DIM>> = ListParetoFront::default();
    perform_bench(&elements, list_front);
    // // TEST KD-TREE
    // println!("kd-tree structure:");
    // let kdtree_front:KDTreeFront<u16, CartesianParetoElement<NB_DIM>, NB_DIM> = KDTreeFront::default();
    // perform_bench(&elements, kdtree_front);
}


pub fn pareto_pq_bench() {
    println!("running pareto priority queue benchmark...");
    bench_pareto::<9>(100000);
}