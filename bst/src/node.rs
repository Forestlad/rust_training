#![forbid(unsafe_code)]

pub struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) val: V,
    pub(crate) left: Option<Box<Node<K, V>>>,
    pub(crate) right: Option<Box<Node<K, V>>>,
    pub(crate) balance_factor: i8,
    pub(crate) size: usize,
}

pub enum Balance {
        Balanced,
        LeftRotate,
        RightRotate,
        BigLeftRotate,
        BigRightRotate,
    }

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Self { key, val, left: None, right: None, balance_factor: 0, size: 1 }
    }
    pub fn set_val(&mut self, val: V) -> V {
        std::mem::replace(&mut self.val, val)
    }
    pub fn set_left(&mut self, node: Box<Self>) {
        debug_assert!(self.left.is_none(), "Left child already exists!");
        self.left = Some(node);
    }
    pub fn set_right(&mut self, node: Box<Self>) {
        debug_assert!(self.right.is_none(), "Right child already exists!");
        self.right = Some(node);
    }
    // fn get_left_right_height(&self) -> (usize, usize) {
    //     let left_h = self.left.as_ref().map_or(0, |x| x.height);
    //     let right_h = self.right.as_ref().map_or(0, |x| x.height);
    //     (left_h, right_h)
    // }
    // pub fn correct_height(&mut self) {
    //     let (left_h, right_h) = self.get_left_right_height();
    //     self.height = left_h.max(right_h) + 1;
    // }
    pub fn left_size(&self) -> usize {
        if let Some(x) = &self.left {
            x.size
        } else {
            0
        }
    }

    pub fn right_size(&self) -> usize {
        if let Some(x) = &self.right {
            x.size
        } else {
            0
        }
    }

    pub fn fix_size(&mut self) {
        self.size = self.left_size() + self.right_size() + 1;
    }

    pub fn check_balance(&self) -> Balance {
        if self.balance_factor.abs() < 2 {
            Balance::Balanced
        } else {
            if self.balance_factor < 0 {
                let v = self.right.as_deref().unwrap().balance_factor;
                if v == 1  {
                    Balance::BigLeftRotate
                } else {
                    Balance::LeftRotate
                }
            } else {
                let v = self.left.as_deref().unwrap().balance_factor;
                if v == 1 {
                    Balance::BigRightRotate
                } else {
                    Balance::RightRotate
                }
            }
        }
    }
    pub fn remove_left(&mut self) -> Option<Box<Self>> {
        self.left.take()
    }
    pub fn remove_right(&mut self) -> Option<Box<Self>> {
        self.right.take()
    }
}
