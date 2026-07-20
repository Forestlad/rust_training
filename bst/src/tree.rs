#![forbid(unsafe_code)]
use crate::node::{Node, Balance};

pub struct AVLTreeMap<K, V> {
    head: Option<Box<Node<K, V>>>,
    size: usize,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn left_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut r_child = vertex.remove_right().unwrap();
        if let Some(rl_child) = r_child.remove_left() {
            vertex.set_right(rl_child);
            vertex.correct_height();
        }
        r_child.set_left(vertex);
        r_child.correct_height();
        r_child
    }

    fn big_left_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut r_child = vertex.remove_right().unwrap();
        let mut rl_child = r_child.remove_left().unwrap();
        if let Some(rlr) = rl_child.remove_right() {
            r_child.set_left(rlr);
            r_child.correct_height();
        }
        if let Some(rll) = rl_child.remove_left() {
            vertex.set_right(rll);
            vertex.correct_height();
        }
        rl_child.set_left(vertex);
        rl_child.set_right(r_child);
        rl_child.correct_height();
        rl_child
    }

    fn right_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut l_child = vertex.remove_left().unwrap();
        if let Some(lr_child) = l_child.remove_right() {
            vertex.set_left(lr_child);
            vertex.correct_height();
        }
        l_child.set_right(vertex);
        l_child.correct_height();
        l_child
    }

    fn big_right_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut l_child = vertex.remove_left().unwrap();
        let mut lr_child = l_child.remove_right().unwrap();
        if let Some(lrl) = lr_child.remove_left() {
            l_child.set_right(lrl);
            l_child.correct_height();
        }
        if let Some(lrr) = lr_child.remove_right() {
            vertex.set_left(lrr);
            vertex.correct_height();
        }
        lr_child.set_left(l_child);
        lr_child.set_right(vertex);
        lr_child.correct_height();
        lr_child
    }

    fn search_for_insert(head: &mut Option<Box<Node<K, V>>>, key: K, value: V) -> Option<V> {
        if head.is_none() {
            *head = Some(Box::new(Node::new(key, value)));
            return None;
        }
        let mut vertex = head.take().unwrap();
        if (*vertex).key == key {
            return Some(vertex.set_val(value));
        }
        let res = if key < (*vertex).key {
            Self::search_for_insert(&mut vertex.left, key, value)
        } else {
            Self::search_for_insert(&mut vertex.right, key, value)
        };
        if res.is_none() {
            vertex.correct_height();
            let balance = vertex.check_balance();
            vertex = match balance {
                Balance::LeftRotate => Self::left_rotate(vertex),
                Balance::BigLeftRotate => Self::big_left_rotate(vertex),
                Balance::RightRotate => Self::right_rotate(vertex),
                Balance::BigRightRotate => Self::big_right_rotate(vertex),
                Balance::Balanced => vertex,
            }
        }
        *head = Some(vertex);
        res
    }

    // fn get(&self, key: ...) -> Option<&V>
    // fn get_key_value(&self, key: ...) -> Option<&V>
    pub fn contains_key(&self, key: &K) -> bool {
        let mut node = &self.head;
        while let Some(curr) = node {
            match curr.key.cmp(key) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => node = &curr.left,
                std::cmp::Ordering::Greater => node = &curr.right,
            }
        }
        false
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let res = Self::search_for_insert(&mut self.head, key, value);
        if res.is_none() {
            self.size += 1;
        }
        res
    }
    // fn nth_key_value(&self, k: usize) -> Option<(&K, &V)>
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let res = Self::remove_node(&mut self.head, key);
        if res.is_some() {
            self.size -= 1;
        }
        if let Some((_, v)) = res {
            Some(v)
        } else {
            None
        }
    }
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let res = Self::remove_node(&mut self.head, key);
        if res.is_some() {
            self.size -= 1;
        }
        res
    }

    fn remove_node(head: &mut Option<Box<Node<K, V>>>, key: &K) -> Option<(K, V)> {
        if head.is_none() {
            return None;
        }
        let mut vertex = head.take().unwrap();
        let res = match vertex.key.cmp(key) {
            std::cmp::Ordering::Greater => Self::remove_node(&mut vertex.right, key),
            std::cmp::Ordering::Less => Self::remove_node(&mut vertex.left, key),
            std::cmp::Ordering::Equal => {
                None
            },
        };
        if res.is_some() {
            vertex.correct_height();
            let balance = vertex.check_balance();
            vertex = match balance {
                Balance::LeftRotate => Self::left_rotate(vertex),
                Balance::BigLeftRotate => Self::big_left_rotate(vertex),
                Balance::RightRotate => Self::right_rotate(vertex),
                Balance::BigRightRotate => Self::big_right_rotate(vertex),
                Balance::Balanced => vertex,
            }
        }
        *head = Some(vertex);
        res
    }

    fn remove_help() {
        
    }
}
