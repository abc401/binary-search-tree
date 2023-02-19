use std::cmp::PartialOrd;
use std::fmt::Display;

pub struct ValueNotInTree;

pub enum BST<T: PartialOrd> {
    Null,
    NonNull {
        val: T,
        left: Box<Self>,
        right: Box<Self>
    }
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
        Self::NonNull {
            val,
            left: Box::new(Self::Null),
            right: Box::new(Self::Null)
        }
    }

    pub fn insert(&mut self, val: T) {
        if let Self::Null = self {
            *self = Self::new(val);
            return
        }

        let Self::NonNull { val: selfval, left, right } = self;

        if val == *selfval {
            return
        }
        if val > *selfval {
            left.insert(val)
        } else {
            right.insert(val)
        }
    }

    pub fn search(&self, val: T) -> &Self {
        if let Self::Null = self {
            return self
        }

        let Self::NonNull { val: selfval, left, right } = self;
        if val == *selfval {
            return self
        }

        if val > *selfval {
            right.search(val)
        } else {
            left.search(val)
        }

    }

    pub fn search_mut(&mut self, val: T) -> &mut Self {
        if let Self::Null = self {
            return self
        }

        let Self::NonNull { val: selfval, left, right } = self;
        if val == *selfval {
            return self
        }

        if val > *selfval {
            right.search_mut(val)
        } else {
            left.search_mut(val)
        }

    }

    // pub fn get_parent(&self, val: T) -> &Self {
    //     let parent = self;

    // }

    pub fn delete(&mut self, val: T) -> Result<(), ValueNotInTree> {
        let node = self.search_mut(val).ok_or(ValueNotInTree)?;
        match (&node.left, &node.right) {
            (None, None) => {

            },
            (Some(child), None) | (Some(child), None) => {

            },
            _ => {

            }
        }
        Ok(())
    }
}

impl<T: Display + PartialOrd> BST<T> {
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
