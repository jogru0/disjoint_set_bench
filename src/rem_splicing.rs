use std::cmp::Ordering;

use self::node::Node;

mod node {
    use std::cell::Cell;

    #[derive(Clone)]
    pub(crate) struct Node {
        //id of parent, (id to self if root)
        parent: Cell<usize>,
    }

    impl Node {
        pub(crate) fn new(id: usize) -> Self {
            Self {
                parent: Cell::new(id),
            }
        }

        pub(crate) fn get_parent(&self) -> usize {
            self.parent.get()
        }

        pub(crate) fn set_parent(&self, new: usize) {
            self.parent.set(new);
        }
    }
}
pub struct DisjointSet {
    //Mutable because some functions update the internal structure,
    //but do not modify the represented subsets.
    nodes: Vec<Node>,
}

impl DisjointSet {
    //Traditional root_of with path splitting.
    fn root_of(&self, mut child: usize) -> usize {
        let mut parent = self.nodes[child].get_parent();

        if child == parent {
            return child;
        };

        loop {
            let grandparent = self.nodes[parent].get_parent();

            if parent == grandparent {
                return parent;
            }

            self.nodes[child].set_parent(grandparent);
            child = parent;
            parent = grandparent;
        }
    }

    //Initializes the discrete set with given size.
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).map(Node::new).collect(),
        }
    }

    //If i and j belong to different subsets, joins these two subsets, return true.
    //Otherwise, returns false.
    //Internally, uses splicing
    pub fn join(&mut self, mut i: usize, mut j: usize) -> bool {
        let mut pi = self.nodes[i].get_parent();
        let mut pj = self.nodes[j].get_parent();
        loop {
            match pi.cmp(&pj) {
                Ordering::Equal => return false,
                Ordering::Less => {
                    if i == pi {
                        self.nodes[i].set_parent(pj);
                        return true;
                    }
                    self.nodes[i].set_parent(pj);
                    i = pi;
                    pi = self.nodes[i].get_parent();
                }
                Ordering::Greater => {
                    if j == pj {
                        self.nodes[j].set_parent(pi);
                        return true;
                    }
                    self.nodes[j].set_parent(pi);
                    j = pj;
                    pj = self.nodes[j].get_parent();
                }
            }
        }
    }

    pub fn are_in_same_set(&self, i: usize, j: usize) -> bool {
        self.root_of(i) == self.root_of(j)
    }
}
