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
    //Returns a current root, and the same root if i and j are in the same set, or an elemnt of the other set otherwise.
    //Along the way, splits the path.
    fn diff(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        let mut pi = self.nodes[i].get_parent();
        let mut pj = self.nodes[j].get_parent();
        loop {
            match pi.cmp(&pj) {
                Ordering::Equal => return (pi, pj),
                Ordering::Less => {
                    if i == pi {
                        return (pi, pj);
                    }
                    let gpi = self.nodes[pi].get_parent();
                    self.nodes[i].set_parent(gpi);
                    i = pi;
                    pi = gpi;
                }
                Ordering::Greater => {
                    if j == pj {
                        return (pj, pi);
                    }
                    let gpj = self.nodes[pj].get_parent();
                    self.nodes[j].set_parent(gpj);
                    j = pj;
                    pj = gpj;
                }
            }
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
    //Internally, appends the rankwise smallest to the rankwise biggest tree.
    pub fn join(&mut self, i: usize, j: usize) -> bool {
        // //Immediate parent check.
        // let pi = self.nodes[i].get_parent().unwrap_or(i);
        // let pj = self.nodes[j].get_parent().unwrap_or(j);
        // if pi == pj {
        //     return false;
        // }

        let (root, new_parent_or_same) = self.diff(i, j);
        assert!(root <= new_parent_or_same);

        if root == new_parent_or_same {
            return false;
        }

        self.nodes[root].set_parent(new_parent_or_same);

        true
    }

    pub fn are_in_same_set(&self, i: usize, j: usize) -> bool {
        let (root, same_if_in_same_set) = self.diff(i, j);
        root == same_if_in_same_set
    }
}
