#![forbid(unsafe_code)]
use crate::node::{Node, Balance};

pub struct AVLTreeMap<K, V> {
    head: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        if let Some(h) = &self.head {
            return h.size;
        }
        0
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn left_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut r_child = vertex.remove_right().unwrap();
        if let Some(rl_child) = r_child.remove_left() {
            vertex.set_right(rl_child);
        }
        let bf = r_child.balance_factor;
        (vertex.balance_factor, r_child.balance_factor) = if bf == 0 {
            (-1, 1)
        } else {
            (0, 0)
        };
        vertex.fix_size();
        r_child.set_left(vertex);
        r_child.fix_size();
        r_child
    }

    fn right_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut l_child = vertex.remove_left().unwrap();
        if let Some(lr_child) = l_child.remove_right() {
            vertex.set_left(lr_child);
        }
        let bf = l_child.balance_factor;
        (vertex.balance_factor, l_child.balance_factor) = if bf == 0 {
            (-1, 1)
        } else {
            (0, 0)
        };
        vertex.fix_size();
        l_child.set_right(vertex);
        l_child.fix_size();
        l_child
    }

    fn big_left_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut r_child = vertex.remove_right().unwrap();
        r_child = Self::right_rotate(r_child);
        vertex.set_right(r_child);
        Self::left_rotate(vertex)
    }

    fn big_right_rotate(mut vertex: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut l_child = vertex.remove_left().unwrap();
        l_child = Self::left_rotate(l_child);
        vertex.set_left(l_child);
        Self::right_rotate(vertex)
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

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut node = &self.head;
        while let Some(curr) = node {
            match curr.key.cmp(key) {
                std::cmp::Ordering::Equal => return Some(&curr.val),
                std::cmp::Ordering::Less => node = &curr.left,
                std::cmp::Ordering::Greater => node = &curr.right,
            }
        }
        None
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        let mut node = &self.head;
        while let Some(curr) = node {
            match curr.key.cmp(key) {
                std::cmp::Ordering::Equal => return Some((&curr.key, &curr.val)),
                std::cmp::Ordering::Less => node = &curr.left,
                std::cmp::Ordering::Greater => node = &curr.right,
            }
        }
        None
    }

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
        res
    }
    // fn nth_key_value(&self, k: usize) -> Option<(&K, &V)>
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let res = Self::remove_node(&mut self.head, key);
        if let Some((_, v)) = res {
            Some(v)
        } else {
            None
        }
    }
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let res = Self::remove_node(&mut self.head, key);
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

    pub fn nth_key_value(&self, mut n: usize) -> Option<(&K, &V)> {
        let mut node = &self.head;
        while let Some(curr) = node {
            let left_size = curr.left_size();
            match left_size.cmp(&n) {
                std::cmp::Ordering::Equal => return Some((&curr.key, &curr.val)),
                std::cmp::Ordering::Less => node = &curr.left,
                std::cmp::Ordering::Greater => {
                    n -= left_size + 1;
                    node = &curr.right;
                },
            }
        }
        None
    }
}
