use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode<T:PartialEq+PartialOrd+Clone> {
    val: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

pub struct BinaryTree<T:PartialEq+PartialOrd+Clone> {
    size:usize,
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T:PartialEq+PartialOrd+Clone> TreeNode<T> {
    pub fn new(val:T) -> Self {
        TreeNode {
            val:val,
            left:None,
            right:None
        }
    }

    pub fn add_left(&mut self,val:T) {
        self.left = Some(
            Rc::new(
                RefCell::new(TreeNode::new(val))
            )
        )
    }

    pub fn add_right(&mut self,val:T) {
        self.right = Some(
            Rc::new(
                RefCell::new(TreeNode::new(val))
            )
        )
    }
}

impl<T:PartialEq+PartialOrd+Clone> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            size:0,
            root: None
        }
    }

    pub fn from()


}