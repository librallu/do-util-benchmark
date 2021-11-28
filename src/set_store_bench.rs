use std::time::Instant;
use rand::Rng;

use do_util::set_store::{
    SetStore,   
    set_trie::TrieSetStore,
    list::ListSetStore,
};


/// computes a benchmark for a given data-structure
pub fn run_set_store_benchmark<Store>(bench:&[Vec<usize>], mut store:Store)
where Store:SetStore<usize> {
    // insertion
    let start_time = Instant::now();
    for set in bench {
        store.insert(set);
    }
    let t = start_time.elapsed().as_secs_f32();
    println!("\t{:<5} inserts in {} seconds ({} inserts/s)",
        bench.len(), t, bench.len() as f32/t
    );
    // find sub-sets for all sets
    let start_time = Instant::now();
    let mut nb_subsets = 0;
    for set in bench {
        nb_subsets += store.find_subsets(set).count();
    }
    let t = start_time.elapsed().as_secs_f32();
    println!("\t{:<5} sub-set found in {} seconds ({} queries/s)",
    nb_subsets, t, nb_subsets as f32/t
    );
    // find super-sets for all sets
    let start_time = Instant::now();
    let mut nb_supersets = 0;
    for set in bench {
        nb_supersets += store.find_supersets(set).count();
    }
    let t = start_time.elapsed().as_secs_f32();
    println!("\t{:<5} super-set found in {} seconds ({} queries/s)",
    nb_supersets, t, nb_supersets as f32/t
    );
}



/// n: number of elements
/// m: number of sets
pub fn set_store_benchmark(n:usize, m:usize) {
    // 1. generate the benchmark
    let mut rng = rand::thread_rng();
    let mut sets:Vec<Vec<usize>> = Vec::new();
    for _ in 0..m {
        let mut s = Vec::new();
        for i in 0..n {
            if rng.gen_range(0..100) <= 10 {
                s.push(i);
            }
        }
        sets.push(s);
    }
    println!("#### TESTING LIST SET_QUERY");
    run_set_store_benchmark(&sets, ListSetStore::default());
    println!("#### TESTING TRIE SET_QUERY");
    run_set_store_benchmark(&sets, TrieSetStore::default());
}