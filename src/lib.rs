pub mod link_by_rank_path_halving;
pub mod original_port;
pub mod rem_path_splitting;
pub mod rem_splicing;

pub mod test_utility {
    use rand::{seq::SliceRandom, SeedableRng};
    use rand_chacha::ChaChaRng;

    use crate::{link_by_rank_path_halving, original_port, rem_path_splitting, rem_splicing};

    pub fn graph_example_1() -> (usize, Vec<(usize, usize)>) {
        let size = 1_000_000;
        let mut edges = Vec::new();
        for i in 0..size {
            let mut j = i + 99;
            while j < size {
                edges.push((i, j));
                j += i + 99;
            }
        }

        let seed = [0; 32];
        let mut rng = ChaChaRng::from_seed(seed);
        edges.shuffle(&mut rng);

        (size, edges)
    }

    pub fn graph_example_1_reversed() -> (usize, Vec<(usize, usize)>) {
        let (size, mut edges) = graph_example_1();
        edges.reverse();

        (size, edges)
    }

    pub fn run_extern_implementation_partitions(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = partitions::partition_vec![(); size];
        for edge in edges {
            if disjoint_set.other_sets(edge.0, edge.1) {
                disjoint_set.union(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }

    pub fn run_extern_implementation_disjoint_set_no_quick_union(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = disjoint::DisjointSet::with_len(size);
        for edge in edges {
            if !disjoint_set.is_joined(edge.0, edge.1) {
                disjoint_set.join(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }

    pub fn run_extern_implementation_disjoint_set(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = disjoint::DisjointSet::with_len(size);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) {
                result.push(edge);
            }
        }

        result
    }

    pub fn run_extern_implementation_disjoint_set_vec_no_quick_union(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = disjoint::DisjointSetVec::from(vec![(); size]);
        for edge in edges {
            if !disjoint_set.is_joined(edge.0, edge.1) {
                disjoint_set.join(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }

    pub fn run_extern_implementation_disjoint_set_vec(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = disjoint::DisjointSetVec::from(vec![(); size]);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) {
                result.push(edge);
            }
        }

        result
    }

    pub fn run_original_port(size: usize, edges: &[(usize, usize)]) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = original_port::DisjointSet::new(size);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) != usize::MAX {
                result.push(edge);
            }
        }

        result
    }

    pub fn run_original_port_no_quick_union(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = original_port::DisjointSet::new(size);
        for edge in edges {
            if disjoint_set.find(edge.0) != disjoint_set.find(edge.1) {
                disjoint_set.join(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }

    pub fn run_link_by_rank_path_halving(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = link_by_rank_path_halving::DisjointSet::new(size);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) {
                result.push(edge);
            }
        }

        result
    }

    pub fn run_link_by_rank_path_halving_no_quick_union(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = link_by_rank_path_halving::DisjointSet::new(size);
        for edge in edges {
            if !disjoint_set.are_in_same_set(edge.0, edge.1) {
                disjoint_set.join(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }

    pub fn run_rem_path_splitting(size: usize, edges: &[(usize, usize)]) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = rem_path_splitting::DisjointSet::new(size);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) {
                result.push(edge);
            }
        }

        result
    }

    pub fn run_rem_path_splitting_no_quick_union(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = rem_path_splitting::DisjointSet::new(size);
        for edge in edges {
            if !disjoint_set.are_in_same_set(edge.0, edge.1) {
                disjoint_set.join(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }

    pub fn run_rem_splicing(size: usize, edges: &[(usize, usize)]) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = rem_splicing::DisjointSet::new(size);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) {
                result.push(edge);
            }
        }

        result
    }

    pub fn run_rem_splicing_no_quick_union(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set = rem_splicing::DisjointSet::new(size);
        for edge in edges {
            if !disjoint_set.are_in_same_set(edge.0, edge.1) {
                disjoint_set.join(edge.0, edge.1);
                result.push(edge);
            }
        }

        result
    }
}
