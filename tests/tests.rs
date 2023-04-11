#[cfg(test)]
mod tests {
    use disjoint_set_bench::test_utility::{
        graph_example_1, graph_example_1_reversed, run_extern_implementation_partitions,
        run_link_by_rank_path_halving, run_link_by_rank_path_halving_no_quick_union,
        run_link_by_rank_path_splitting, run_link_by_rank_path_splitting_no_quick_union,
        run_original_port, run_original_port_no_quick_union, run_rem_path_splitting,
        run_rem_path_splitting_no_quick_union,
    };

    #[test]
    fn test_correctness_graph_example_1() {
        let (size, edges) = graph_example_1();

        assert_eq!(size, 1_000_000);
        assert_eq!(edges.len(), 8_802_760);

        let result_extern_implementation_partitions =
            run_extern_implementation_partitions(size, &edges);
        let result_original_port = run_original_port(size, &edges);
        let result_original_port_no_quick_union = run_original_port_no_quick_union(size, &edges);
        let result_link_by_rank_path_halving = run_link_by_rank_path_halving(size, &edges);
        let result_link_by_rank_path_halving_no_quick_union =
            run_link_by_rank_path_halving_no_quick_union(size, &edges);
        let result_link_by_rank_path_splitting = run_link_by_rank_path_splitting(size, &edges);
        let result_link_by_rank_path_splitting_no_quick_union =
            run_link_by_rank_path_splitting_no_quick_union(size, &edges);
        let result_rem_path_splitting = run_rem_path_splitting(size, &edges);
        let result_rem_path_splitting_no_quick_union =
            run_rem_path_splitting_no_quick_union(size, &edges);

        assert_eq!(result_extern_implementation_partitions.len(), 999_999);
        assert_eq!(
            result_extern_implementation_partitions,
            result_original_port
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_original_port_no_quick_union
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_halving
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_halving_no_quick_union
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_splitting
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_splitting_no_quick_union
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_rem_path_splitting
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_rem_path_splitting_no_quick_union
        );
    }

    #[test]
    fn test_correctness_graph_example_1_reversed() {
        let (size, edges) = graph_example_1_reversed();

        assert_eq!(size, 1_000_000);
        assert_eq!(edges.len(), 8_802_760);

        let result_extern_implementation_partitions =
            run_extern_implementation_partitions(size, &edges);
        let result_original_port = run_original_port(size, &edges);
        let result_original_port_no_quick_union = run_original_port_no_quick_union(size, &edges);
        let result_link_by_rank_path_halving = run_link_by_rank_path_halving(size, &edges);
        let result_link_by_rank_path_halving_no_quick_union =
            run_link_by_rank_path_halving_no_quick_union(size, &edges);
        let result_link_by_rank_path_splitting = run_link_by_rank_path_splitting(size, &edges);
        let result_link_by_rank_path_splitting_no_quick_union =
            run_link_by_rank_path_splitting_no_quick_union(size, &edges);
        let result_rem_path_splitting = run_rem_path_splitting(size, &edges);
        let result_rem_path_splitting_no_quick_union =
            run_rem_path_splitting_no_quick_union(size, &edges);

        assert_eq!(result_extern_implementation_partitions.len(), 999_999);
        assert_eq!(
            result_extern_implementation_partitions,
            result_original_port
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_original_port_no_quick_union
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_halving
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_halving_no_quick_union
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_splitting
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_link_by_rank_path_splitting_no_quick_union
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_rem_path_splitting
        );
        assert_eq!(
            result_extern_implementation_partitions,
            result_rem_path_splitting_no_quick_union
        );
    }
}
