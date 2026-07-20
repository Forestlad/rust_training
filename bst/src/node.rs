#![forbid(unsafe_code)]

pub struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) val: V,
    pub(crate) left: Option<Box<Node<K, V>>>,
    pub(crate) right: Option<Box<Node<K, V>>>,
    pub(crate) height: usize,
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
        Self { key, val, left: None, right: None, height: 0 }
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
    fn get_left_right_height(&self) -> (usize, usize) {
        let left_h = self.left.as_ref().map_or(0, |x| x.height);
        let right_h = self.right.as_ref().map_or(0, |x| x.height);
        (left_h, right_h)
    }
    pub fn correct_height(&mut self) {
        let (left_h, right_h) = self.get_left_right_height();
        self.height = left_h.max(right_h) + 1;
    }
    pub fn check_balance(&self) -> Balance {
        let (left_h, right_h) = self.get_left_right_height();
        let del = left_h.abs_diff(right_h);
        if del < 2 {
            Balance::Balanced
        } else {
            if left_h < right_h {
                let v = self.right.as_deref().unwrap();
                let (lh, rh) = v.get_left_right_height();
                if lh <= rh {
                    Balance::LeftRotate
                } else {
                    Balance::BigLeftRotate
                }
            } else {
                let v = self.left.as_deref().unwrap();
                let (lh, rh) = v.get_left_right_height();
                if rh <= lh {
                    Balance::RightRotate
                } else {
                    Balance::BigRightRotate
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
