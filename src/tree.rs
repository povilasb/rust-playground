use std::mem;
use std::cmp::max;
use std::fmt;

#[derive(Debug)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub value: u64,
    index: usize,
}

impl Node {
    fn new(value: u64, index: usize) -> Node {
        Node{left: None, right: None, value: value, index: index}
    }

    fn set_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    fn set_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }

    pub fn swap_children(&mut self) {
        mem::swap(&mut self.left, &mut self.right);
    }
}

pub struct Tree {
    root: Node,
    last_index: usize,
}

impl Tree {
    pub fn new() -> Tree {
        Tree{root: Node::new(1, 1), last_index: 1}
    }

    pub fn append(&mut self, left: Option<u64>, right: Option<u64>, at_index: usize) {
        let last_node = node_by_index(&mut self.root, at_index).unwrap();
        if left.is_some() {
            self.last_index += 1;
            last_node.set_left(Node::new(left.unwrap(), self.last_index));
        }
        if right.is_some() {
            self.last_index += 1;
            last_node.set_right(Node::new(right.unwrap(), self.last_index));
        }
    }

    pub fn nodes_at_depth(&mut self, depth: usize) -> Vec<&mut Node> {
        let mut res_nodes: Vec<&mut Node> = Vec::new();
        nodes_by_depth(&mut self.root, depth, 1, &mut res_nodes);
        res_nodes
    }

    pub fn depth(&self) -> usize {
        tree_depth(&self.root, 1)
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", inorder_traverse("".to_string(), &self.root))
    }
}

fn tree_depth(node: &Node, curr_depth: usize) -> usize {
    let left_depth = match node.left {
        Some(ref lnode) => tree_depth(lnode, curr_depth + 1),
        None => curr_depth,
    };
    let right_depth = match node.right {
        Some(ref rnode) => tree_depth(rnode, curr_depth + 1),
        None => curr_depth,
    };
    max(left_depth, right_depth)
}

fn nodes_by_depth<'a>(node: &'a mut Node, depth: usize, curr_depth: usize, res_nodes: &mut Vec<&'a mut Node>) {
    if curr_depth == depth {
        res_nodes.push(node);
        return;
    }
    if node.left.is_some() {
        nodes_by_depth(node.left.as_mut().unwrap(), depth, curr_depth + 1, res_nodes);
    }
    if node.right.is_some() {
        nodes_by_depth(node.right.as_mut().unwrap(), depth, curr_depth + 1, res_nodes);
    }
}

fn inorder_traverse(mut acc: String, node: &Node) -> String {
    if node.left.is_some() {
        acc = inorder_traverse(acc, node.left.as_ref().unwrap());
    }
    acc = acc + &node.value.to_string() + " ";
    if node.right.is_some() {
        acc = inorder_traverse(acc, node.right.as_ref().unwrap());
    }
    acc
}

fn node_by_index(node: &mut Node, index: usize) -> Option<&mut Node> {
    if node.index == index {
        return Some(node);
    }
    if node.left.is_some() {
        let node = node_by_index(node.left.as_mut().unwrap(), index);
        if node.is_some() {
            return node;
        }
    }
    if node.right.is_some() {
        let node = node_by_index(node.right.as_mut().unwrap(), index);
        if node.is_some() {
            return node;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    // Trying to emulate nesting what "stainless" lib does.
    mod node {
        use super::*;

        #[test]
        fn new_returns_tree_with_left_and_right_nodes_empty() {
            let node = Node::new(5, 1);

            assert_that!(node.left, is(none::<Box<Node>>()));
            assert_that!(node.right, is(none::<Box<Node>>()));
        }

        #[test]
        fn new_returns_tree_with_specified_index() {
            let node = Node::new(5, 1);

            assert_that!(node.index, is(equal_to(1)));
        }

        #[test]
        fn set_left_changes_left_node() {
            let mut node = Node::new(5, 1);

            node.set_left(Node::new(10, 2));

            let lnode = node.left.unwrap();
            assert_that!(lnode.value, is(equal_to(10)));
            assert_that!(lnode.index, is(equal_to(2)));
        }
    }

    mod tree {
        use super::*;

        #[test]
        fn append_when_only_root_node_exists_given_nodes_are_appended_to_it() {
            let mut t = Tree::new();
            t.append(Some(2), Some(3), 1);

            assert_that!(t.root.left.as_ref().unwrap().value, is(equal_to(2)));
            assert_that!(t.root.left.unwrap().index, is(equal_to(2)));
            assert_that!(t.root.right.as_ref().unwrap().value, is(equal_to(3)));
            assert_that!(t.root.right.unwrap().index, is(equal_to(3)));
        }

        #[test]
        fn append_appends_nodes_to_the_node_with_specified_index() {
            let mut t = Tree::new();
            t.append(Some(2), Some(3), 1);
            t.append(Some(4), None, 2);

            assert_that!(t.root.left.as_ref().unwrap().left.as_ref().unwrap().value, is(equal_to(4)));
        }

        #[test]
        fn nodes_at_depth_return_all_nodes_with_specified_depth() {
            let mut t = Tree::new();
            t.append(Some(2), Some(3), 1);
            t.append(Some(4), None, 2);
            t.append(Some(5), None, 3);

            let nodes = t.nodes_at_depth(3);
            assert_that!(nodes.len(), is(equal_to(2)));
            assert_that!(nodes[0].value, is(equal_to(4)));
            assert_that!(nodes[1].value, is(equal_to(5)));
        }

        #[test]
        fn depth_returns_max_tree_depth() {
            let mut t = Tree::new();
            t.append(Some(2), Some(3), 1);
            t.append(Some(4), None, 2);

            assert_that!(t.depth(), is(equal_to(3)));
        }
    }

    mod node_by_index {
        use super::*;

        #[test]
        fn when_index_is_the_root_node_index_same_node_is_returned() {
            let mut root = Node::new(10, 1);
            root.set_left(Node::new(20, 2));

            let n = node_by_index(&mut root, 1);
            assert_that!(n.unwrap().value, is(equal_to(10)));
        }

        #[test]
        fn it_returns_node_with_specified_index() {
            let mut root = Node::new(10, 1);
            let mut lnode = Node::new(20, 2);
            lnode.set_left(Node::new(40, 4));
            root.set_left(lnode);
            root.set_right(Node::new(30, 3));

            let n = node_by_index(&mut root, 4);
            assert_that!(n.unwrap().value, is(equal_to(40)));
        }
    }
}
