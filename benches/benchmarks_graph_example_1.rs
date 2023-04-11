#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;

    use disjoint_set_bench::test_utility::{
        graph_example_1, run_extern_implementation_partitions, run_link_by_rank_path_halving,
        run_link_by_rank_path_halving_no_quick_union, run_link_by_rank_path_splitting,
        run_link_by_rank_path_splitting_no_quick_union, run_original_port,
        run_original_port_no_quick_union, run_rem_path_splitting,
        run_rem_path_splitting_no_quick_union, run_rem_splicing, run_rem_splicing_no_quick_union,
    };
    use test::{black_box, Bencher};

    #[bench]
    #[ignore]
    fn bench_extern_implementation_partitions(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_extern_implementation_partitions(black_box(size), black_box(&edges)));
    }

    #[bench]
    #[ignore]
    fn bench_original_port(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_original_port(black_box(size), black_box(&edges)));
    }

    #[bench]
    #[ignore]
    fn bench_original_port_no_quick_union(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_original_port_no_quick_union(black_box(size), black_box(&edges)));
    }

    #[bench]
    #[ignore]
    fn bench_link_by_rank_path_halving(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_link_by_rank_path_halving(black_box(size), black_box(&edges)));
    }

    #[bench]
    #[ignore]
    fn bench_link_by_rank_path_halving_no_quick_union(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_link_by_rank_path_halving_no_quick_union(black_box(size), black_box(&edges)));
    }

    #[bench]
    fn bench_link_by_rank_path_splitting(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_link_by_rank_path_splitting(black_box(size), black_box(&edges)));
    }

    #[bench]
    fn bench_link_by_rank_path_splitting_no_quick_union(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| {
            run_link_by_rank_path_splitting_no_quick_union(black_box(size), black_box(&edges))
        });
    }

    #[bench]
    fn bench_rem_path_splitting(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_rem_path_splitting(black_box(size), black_box(&edges)));
    }

    #[bench]
    fn bench_rem_path_splitting_no_quick_union(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_rem_path_splitting_no_quick_union(black_box(size), black_box(&edges)));
    }

    #[bench]
    fn bench_rem_splicing(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_rem_splicing(black_box(size), black_box(&edges)));
    }

    #[bench]
    fn bench_rem_splicing_no_quick_union(b: &mut Bencher) {
        let (size, edges) = graph_example_1();

        b.iter(|| run_rem_splicing_no_quick_union(black_box(size), black_box(&edges)));
    }
}
