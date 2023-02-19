use std::cmp::PartialOrd;
use std::fmt::Display;

pub struct BST<T> {
    pub val: T,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

impl<T: PartialOrd> From<Vec<T>> for BST<T> {
    fn from(vector: Vec<T>) -> Self {
        let mut iter = vector.into_iter();
        let mut head = Self::new(iter.next().unwrap());
        for val in iter {
            head.insert(val)
        }
        head
    }
}

impl<T: PartialOrd> BST<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        if val == self.val {
            return;
        }

        let node;
        if val > self.val {
            node = &mut self.right;
        } else {
            node = &mut self.left;
        }

        if let Some(ref mut node) = node {
            node.insert(val);
            return;
        }
        *node = Some(Box::new(Self::new(val)));
    }

    pub fn search(&self, val: T) -> Option<&Self> {

        if val == self.val {
            return Some(self)
        }

        let node;
        if val > self.val {
            node = &self.right;
        } else {
            node = &self.left;
        }

        if let Some(ref node) = node {
            return node.search(val);
        }
        return None;

    }
}

impl<T: Display> BST<T> {
    pub fn display(&self) {
        print!("{} -> left(", self.val);
        if let Some(ref left) = self.left {
            print!("{})", left.val)
        } else {
            print!("None)")
        }
        println!();
        print!("{} -> right(", self.val);
        if let Some(ref right) = self.right {
            print!("{})", right.val)
        } else {
            print!("None)")
        }
        println!();

        if let Some(ref left) = self.left {
            left.display();
        }
        if let Some(ref right) = self.right {
            right.display();
        }
    }
}

// pub struct BST<T: PartialOrd> {
//     head: Option<Box<BST<T>>>,
// }

// impl<T: PartialOrd> BST<T> {
//     pub fn new() -> Self {
//         Self { head: None }
//     }

//     pub fn insert(&mut self, val: T) {
//         if let Some(ref mut node) = self.head {
//             node.insert(val);
//         } else {
//             self.head = Some(Box::new(BST::new(val)));
//         }
//     }

//     pub fn search(&self, val: T) -> Option<Self> {
//         if let Some(ref node) = self.head {
//             node.search(val)
//         } else {
//             None
//         }
//     }
// }

// impl<T: Display + PartialOrd> BST<T> {
//     pub fn display(&self) {
//         if let Some(ref head) = self.head {
//             head.display();
//         } else {
//             println!("Tree is empty!");
//         }
//     }
// }
