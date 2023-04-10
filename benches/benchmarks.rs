#![feature(test)]

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    fn run_link_by_rank_path_halving(
        size: usize,
        edges: &[(usize, usize)],
    ) -> Vec<&(usize, usize)> {
        let mut result = Vec::new();
        let mut disjoint_set =
            disjoint_set_bench::link_by_rank_path_halving::DisjointSet::new(size);
        for edge in edges {
            if disjoint_set.join(edge.0, edge.1) == usize::MAX {
                result.push(edge);
            }
        }

        result
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let size = 1_000_000;
        let edges = {
            let mut edges = Vec::new();
            for i in 0..size {
                let mut j = i + 99;
                while j < size {
                    edges.push((i, j));
                    j += i + 99;
                }
            }
            edges
        };

        b.iter(|| run_link_by_rank_path_halving(black_box(size), black_box(&edges)));
    }
}
