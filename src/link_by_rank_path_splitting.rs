use std::cmp::Ordering;

use self::node::Node;

mod node {
    use std::cell::Cell;

    #[derive(Clone)]
    pub(crate) struct Node {
        //If root:
        //	-length of the longest branch that would be there
        //	if we would not continually flatten the tree.
        //Otherwise: id of parent.
        parent_or_neg_rank: Cell<isize>,
    }

    impl Node {
        pub(crate) fn new() -> Self {
            Self {
                parent_or_neg_rank: Cell::new(-1),
            }
        }

        // fn is_root(&self) -> bool {
        //     self.parent_or_neg_rank.get().is_negative()
        // }

        pub(crate) fn get_rank(&self) -> usize {
            // assert!(self.is_root());
            -self.parent_or_neg_rank.get() as usize
        }

        pub(crate) fn get_parent(&self) -> Option<usize> {
            let result = self.parent_or_neg_rank.get();
            if result.is_negative() {
                None
            } else {
                Some(result as usize)
            }
        }

        pub(crate) fn set_parent(&self, new: usize) {
            self.parent_or_neg_rank.set(new as isize);
        }

        pub(crate) fn increment_rank(&mut self) {
            // assert!(self.is_root());
            *self.parent_or_neg_rank.get_mut() -= 1;
        }
    }
}
pub struct DisjointSet {
    //Mutable because some functions update the internal structure,
    //but do not modify the represented subsets.
    nodes: Vec<Node>,
}

impl DisjointSet {
    //Returns the root of the subset containing i.
    //Along the way, splits the path.
    fn root_of(&self, mut child: usize) -> usize {
        if let Some(mut parent) = self.nodes[child].get_parent() {
            while let Some(grandparent) = self.nodes[parent].get_parent() {
                self.nodes[child].set_parent(grandparent);
                child = parent;
                parent = grandparent;
            }

            parent
        } else {
            child
        }
    }

    //Initializes the discrete set with given size.
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![Node::new(); size],
        }
    }

    //If i and j belong to different subsets, joins these two subsets, return true.
    //Otherwise, returns false.
    //Internally, appends the rankwise smallest to the rankwise biggest tree.
    pub fn join(&mut self, i: usize, j: usize) -> bool {
        //Immediate parent check.
        let pi = self.nodes[i].get_parent().unwrap_or(i);
        let pj = self.nodes[j].get_parent().unwrap_or(j);
        if pi == pj {
            return false;
        }

        let root_of_i = self.root_of(i);
        let root_of_j = self.root_of(j);

        if root_of_i == root_of_j {
            return false;
        }

        let j_cmp_i = self.nodes[root_of_j]
            .get_rank()
            .cmp(&self.nodes[root_of_i].get_rank());

        if j_cmp_i == Ordering::Greater {
            //root_of_j is the new root.

            self.nodes[root_of_i].set_parent(root_of_j);
        } else {
            //root_of_i is the new root.
            if j_cmp_i == Ordering::Equal {
                self.nodes[root_of_i].increment_rank();
            }
            self.nodes[root_of_j].set_parent(root_of_i);
        }

        true
    }

    pub fn are_in_same_set(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }
}
