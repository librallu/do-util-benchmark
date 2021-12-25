use std::time::Instant;

use rand::Rng;
use rand::prelude::ThreadRng;

use do_util::pareto_pq::{
    ParetoElement, ParetoFront,
    util::CartesianParetoElement
};
use do_util::pareto_pq::list::ListParetoFront;
// use do_util::pareto_pq::naive_kd_tree::NaiveKDTreeFront;
use do_util::pareto_pq::kd_tree::KDTreeFront;

pub fn random_element<const NB_DIM:usize>(rng:&mut ThreadRng) -> CartesianParetoElement<NB_DIM> {
    let mut coords = [0;NB_DIM];
    loop {
        for i in coords.iter_mut().take(NB_DIM) {
            *i = rng.gen();
        }
        if coords.iter().map(|e| {
            (*e as f64 / u16::MAX as f64).powf(NB_DIM as f64)
        }).sum::<f64>() <= 1. {
            let mut new_coords = [u16::MAX ; NB_DIM];
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
Elt:'a + ParetoElement<T>+Eq+Clone,
Front:ParetoFront<T, Elt> {
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
    let mut dim:usize = 1;
    let start_pop = Instant::now();
    let mut last = front.peek_minimum_coordinate(dim).unwrap();
    let mut dim_vec = Vec::new();
    while let Some(e) = front.pop_minimum_element(dim) {
        nb_pops += 1;
        if last > e.kth(dim) {
            println!("error");
        }
        last = e.kth(dim);
        dim_vec.push(e.kth(dim));
        // println!("{:?}", e.kth(dim));
        // println!("{:?}", front.peek_minimum_coordinate(dim));
    }
    dim_vec.sort();
    println!("{:?}", dim_vec);
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
    let list_front:ListParetoFront<CartesianParetoElement<NB_DIM>> = ListParetoFront::default();
    perform_bench(&elements, list_front);
    // TEST KD-TREE
    println!("kd-tree structure:");
    let kdtree_front:KDTreeFront<u16, CartesianParetoElement<NB_DIM>, NB_DIM> = KDTreeFront::default();
    perform_bench(&elements, kdtree_front);
}


pub fn pareto_pq_bench() {
    bench_pareto::<3>(100);
}