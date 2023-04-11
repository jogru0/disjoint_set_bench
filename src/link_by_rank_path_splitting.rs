use std::{cell::Cell, cmp::Ordering};

pub struct DisjointSet {
    //Mutable because some functions update the internal structure,
    //but do not modify the represented subsets.
    parents: Vec<Cell<usize>>,
    ranks: Vec<u8>,
}

impl DisjointSet {
    #[inline(always)]
    fn get_parent(&self, id: usize) -> usize {
        self.parents[id].get()
    }

    #[inline(always)]
    fn set_parent(&self, id: usize, new: usize) {
        self.parents[id].set(new);
    }

    #[inline(always)]
    fn get_mut_rank(&mut self, id: usize) -> &mut u8 {
        &mut self.ranks[id]
    }

    //Returns the root of the subset containing i.
    //Along the way, splits the path.
    fn root_of(&self, mut child: usize) -> usize {
        let mut parent = self.get_parent(child);
        if child == parent {
            return child;
        };

        loop {
            let grandparent = self.get_parent(parent);
            if parent == grandparent {
                return parent;
            }

            self.set_parent(child, grandparent);
            child = parent;
            parent = grandparent;
        }
    }

    //Initializes the discrete set with given size.
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).map(Cell::new).collect(),
            ranks: vec![0; size],
        }
    }

    //If i and j belong to different subsets, joins these two subsets, return true.
    //Otherwise, returns false.
    //Internally, appends the rankwise smallest to the rankwise biggest tree.
    pub fn join(&mut self, i: usize, j: usize) -> bool {
        //Immediate parent check.
        if self.get_parent(i) == self.get_parent(j) {
            return false;
        }

        let root_of_i = self.root_of(i);
        let root_of_j = self.root_of(j);

        if root_of_i == root_of_j {
            return false;
        }

        let rank_j = *self.get_mut_rank(root_of_j);
        let rank_i = self.get_mut_rank(root_of_i);

        let j_cmp_i = rank_j.cmp(rank_i);

        if j_cmp_i == Ordering::Greater {
            //root_of_j is the new root.

            self.set_parent(root_of_i, root_of_j);
        } else {
            //root_of_i is the new root.
            if j_cmp_i == Ordering::Equal {
                *rank_i += 1;
            }
            self.set_parent(root_of_j, root_of_i);
        }

        true
    }

    pub fn are_in_same_set(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }
}
