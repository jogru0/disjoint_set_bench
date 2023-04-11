use std::{cell::Cell, cmp::Ordering};

struct Node {
    //If root:
    //	length of the longest branch that would be there
    //	if we would not continually flatten the tree.
    //Otherwise: nullid.
    rank: usize,
    //If root: identifier of the subset.
    //Otherwise: parent in the internal structure.
    identifier_or_parent: Cell<usize>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            rank: 0,
            identifier_or_parent: Cell::new(id),
        }
    }

    fn is_root_node(&self) -> bool {
        self.rank != usize::MAX
    }

    fn surely_rank(&self) -> usize {
        debug_assert!(self.is_root_node());
        self.rank
    }

    fn surely_parent(&self) -> usize {
        debug_assert!(!self.is_root_node());
        self.identifier_or_parent.get()
    }

    fn surely_identifier(&self) -> usize {
        debug_assert!(self.is_root_node());
        self.identifier_or_parent.get()
    }

    fn demote_root_to_child_of(&mut self, parent: usize) {
        debug_assert!(self.is_root_node());
        self.rank = usize::MAX;
        self.identifier_or_parent.set(parent);
    }

    fn update_parent(&self, new_parent: usize) {
        debug_assert!(!self.is_root_node());
        self.identifier_or_parent.set(new_parent);
    }

    fn update_identifier(&mut self, new_identifier: usize) {
        debug_assert!(self.is_root_node());
        self.identifier_or_parent.set(new_identifier);
    }

    fn increase_rank(&mut self) {
        debug_assert!(self.is_root_node());
        self.rank += 1;
    }
}

pub struct DisjointSet {
    //Mutable because some functions update the internal structure,
    //but do not modify the represented subsets.
    nodes: Vec<Node>,
}

impl DisjointSet {
    //Returns the root of the subset containing i.
    //Along the way, halves the path.
    fn root_of(&self, mut i: usize) -> usize {
        while !self.nodes[i].is_root_node() {
            let child = i;
            i = self.nodes[i].surely_parent();
            if self.nodes[i].is_root_node() {
                break;
            }
            i = self.nodes[i].surely_parent();
            self.nodes[child].update_parent(i);
        }
        i
    }

    //Initializes the discrete set with given size.
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).map(Node::new).collect(),
        }
    }

    //If i and j belong to different subsets,
    //	appends the subset containing j to the subset containing i, keeping the identifier of i
    //	returns the identifier of the old, now subsumed subset of j.
    //Otherwise, returns nullid.
    //Internally, appends the rankwise smallest to the rankwise biggest tree.
    pub fn join(&mut self, i: usize, j: usize) -> usize {
        let root_of_i = self.root_of(i);
        let root_of_j = self.root_of(j);

        if root_of_i == root_of_j {
            return usize::MAX;
        }

        let identifier_j = self.nodes[root_of_j].surely_identifier();

        let j_cmp_i = self.nodes[root_of_j]
            .surely_rank()
            .cmp(&self.nodes[root_of_i].surely_rank());

        if j_cmp_i == Ordering::Greater {
            //root_of_j is the new root.
            let identifier_i = self.nodes[root_of_i].surely_identifier();

            self.nodes[root_of_j].update_identifier(identifier_i);
            self.nodes[root_of_i].demote_root_to_child_of(root_of_j);
        } else {
            //root_of_i is the new root.
            if j_cmp_i == Ordering::Equal {
                self.nodes[root_of_i].increase_rank();
            }
            self.nodes[root_of_j].demote_root_to_child_of(root_of_i);
        }

        identifier_j
    }

    //Returns the identifier of the subset containing i.
    pub fn find(&self, i: usize) -> usize {
        self.nodes[self.root_of(i)].surely_identifier()
    }
}
