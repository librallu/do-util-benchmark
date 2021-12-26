mod set_store_bench;
mod pareto_pq_bench;

fn main() {
    println!("running benchmarks...");
    pareto_pq_bench::pareto_pq_bench();
    // set_store_bench::set_store_benchmark(100,10000);
}

