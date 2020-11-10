use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node<T:PartialEq+Copy> {
    val: T,
    pre: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}
#[derive(Debug)]
pub struct DoubleLinkList<T:PartialEq+Copy> {
    root : Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize
}

pub struct DoubleLinkListIter<'a,T:PartialEq+Copy> {
    head: Option<&'a Rc<RefCell<Node<T>>>>,
    tail: Option<&'a Rc<RefCell<Node<T>>>>,
    len: usize
}

impl<'a,T:PartialEq+Copy> Iterator for DoubleLinkListIter<'a,T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next.as_ref();
                &node.val
            })
        }
    }

    #[inline]
    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

impl<'a, T:PartialEq+Copy> DoubleEndedIterator for DoubleLinkListIter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &*node.as_ptr();
                self.len -= 1;
                self.tail = node.pre.as_ref();
                &node.val
            })
        }
    }
}

impl<T:PartialEq+Copy> Node<T> {
    pub fn new(val:T) -> Self {
        Node {
            val:val,
            pre:None,
            next:None
        }
    }
}

impl<T:PartialEq+Copy> DoubleLinkList<T> {
    pub fn new() -> Self {
        DoubleLinkList {
            len:0,
            root:None,
            tail:None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self,idx:usize,val:T) {
        assert!(idx <= self.len,"Index Out Bound");
        let mut node = Node::new(val);
        if self.root.is_none() && self.tail.is_none() {
            let node = 
                Rc::new(
                    RefCell::new(
                        node
                    )
                );
            self.tail = Some(node.clone());
            self.root = Some(node);
        } else if idx == 0 {
            let mut old_root = self.root.take();
            let new_node_rf =              Rc::new(
                RefCell::new(
                    node
                )
            );
            // 处理old_root
            old_root.as_mut().unwrap().borrow_mut().pre = Some(new_node_rf.clone());
            // 处理new root
            new_node_rf.borrow_mut().next = old_root;
            // 处理root
            self.root = Some(
                new_node_rf
            );
        } else if idx == self.len {
            let mut old_tail = self.tail.take();
            node.pre = Some(old_tail.as_ref().unwrap().clone());
            let new_tail = Rc::new(
                RefCell::new(node)
            );
            old_tail.as_mut().unwrap().borrow_mut().next = Some(
                new_tail.clone()
            );
            self.tail = Some(new_tail);
        } else {
            let mut tmp = self.root.as_ref().unwrap().clone();
            for _ in 0..(idx-1) {
                let tmp1 = tmp.borrow().next.as_ref().unwrap().clone();
                tmp = tmp1;
            }
            // 处理ｎｏｄｅ
            node.pre = Some(tmp.clone());
            node.next = tmp.borrow_mut().next.take();
            let rf_node = Rc::new(
                RefCell::new(node)
            );

            // 处理tmp.next
            rf_node.borrow_mut().next.as_mut().unwrap().borrow_mut().pre = Some(
                rf_node.clone()
            );

            // 处理tmp
            tmp.borrow_mut().next = Some(
                rf_node
            );
        }
        self.len += 1;
    }

    pub fn get(&self,idx:usize) -> &T {
        assert!(idx < self.len(),"Index Out Bound");
        if idx == 0 {
            unsafe {&(*(*Rc::as_ptr(self.root.as_ref().unwrap())).as_ptr()).val} 
        } else if idx == self.len - 1 {
            unsafe {&(*(*Rc::as_ptr(self.tail.as_ref().unwrap())).as_ptr()).val} 
        } else {
            let mut tmp = self.root.as_ref().unwrap().clone();
            for _ in 0..idx {
                let tmp1 = tmp.borrow().next.as_ref().unwrap().clone();
                tmp = tmp1;
            }
            unsafe {&(*(*Rc::as_ptr(&tmp)).as_ptr()).val}  
        }
    }

    pub fn remove(&mut self,idx:usize) -> T {
        assert!(idx < self.len(),"Index Out Bound");
        if idx == 0 {
            self.len -= 1;
            let tmp = self.root.as_ref().unwrap().clone();
            // 处理tmp -> next
            tmp.borrow_mut().next.as_mut().unwrap().borrow_mut().pre = None;
            // 处理root
            self.root = Some(
                tmp.borrow().next.as_ref().unwrap().clone()
            );
            // 处理ｔｍｐ
            unsafe { (*tmp.as_ptr()).val }
        } else if idx == self.len - 1 {
            self.len -= 1;
            let tmp = self.tail.as_ref().unwrap().clone();
            // 处理tmp -> pre
            tmp.borrow_mut().pre.as_mut().unwrap().borrow_mut().next = None;
            // 处理tail
            self.tail = Some(
                tmp.borrow().pre.as_ref().unwrap().clone()
            );
            // 处理tmp
            unsafe { (*tmp.as_ptr()).val }
        } else {
            self.len -= 1;
            let mut tmp = self.root.as_ref().unwrap().clone();
            for _ in 0..idx {
                let tmp1 = tmp.borrow().next.as_ref().unwrap().clone();
                tmp = tmp1;
            }
    
            let tmp_pre =  tmp.borrow_mut().pre.as_mut().unwrap().clone();
    
            // 处理tmp -> next
            tmp.borrow_mut().next.as_mut().unwrap().borrow_mut().pre = Some(tmp_pre.clone());
            //处理tmp -> pre
            tmp_pre.borrow_mut().next = tmp.borrow_mut().next.take();
            // 处理tmp
            unsafe { (*tmp.as_ptr()).val }
        }
        
    }

    pub fn iter(&self) -> DoubleLinkListIter <'_,T> {
        DoubleLinkListIter {
            head: self.root.as_ref(),
            tail: self.tail.as_ref(),
            len: self.len
        }
    }

    pub fn contains(&self,val:&T) -> bool {
        self.iter().any(|e| e == val)
    }

    pub fn push_back(&mut self,val: T) {
        self.insert(self.len, val);
    }

    pub fn pop_back(&mut self) -> T {
        self.remove(self.len - 1)
    }

    pub fn back(&self) -> &T {
        self.get(self.len-1)
    }

    pub fn push_front(&mut self,val:T) {
        self.insert(0, val);
    }

    pub fn pop_front(&mut self) -> T {
        self.remove(0)
    }

    pub fn front(&self) -> &T {
        self.get(0)
    }
}

#[test]
fn ok() {
    let mut l = DoubleLinkList::<i32>::new();
    l.insert(0, 2);
    l.insert(1, 3);
    l.insert(2, 4);
    l.insert(3, 5);
    l.insert(4, 6);
    l.push_back(7);
    l.push_back(8);
    l.push_front(0);
    l.push_front(-1);
    l.pop_back();
    l.pop_front();
    println!("len:{:?}",l.len());
    println!("idx=4:{:?}",l.get(4));
    println!("contain val 7 {:?}",l.contains(&7));
    let dl = l.iter().rev();
    let dl = dl.collect::<Vec<&i32>>();
    println!("{:?}",dl);
}