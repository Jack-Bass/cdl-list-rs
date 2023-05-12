//

use std::{cell::{RefCell, Ref}, rc::{Rc, Weak}, fmt::{Debug, format}, borrow::{BorrowMut, Borrow}, ops::Deref};

#[derive(Debug)]
enum LinkType<T> {
    StrongLink(Rc<RefCell<T>>), 
    WeakLink(Weak<RefCell<T>>)
}

impl<T: Debug> std::clone::Clone for LinkType<Node<T>> {
    fn clone(&self) -> Self {
        match self {
            Self::StrongLink(arg0) => Self::StrongLink(arg0.clone()),
            Self::WeakLink(arg0) => Self::WeakLink(arg0.clone()),
        }
    }
}

#[derive(Clone, Debug)]
struct Node<T: Debug> {
    next: Option<LinkType<Node<T>>>, 
    prev: Option<LinkType<Node<T>>>,
    data: T
}

impl<T: Debug> Node<T> {
    fn new(t: T) -> Node<T> {
        Self {
            next: None, 
            prev: None, 
            data: t
        }
    }
}

impl<T: Debug> std::ops::Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping: {:?}", self);
    }
}

#[derive(Debug)]
pub struct CdlList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}

impl<T: Debug> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp = format!("{:?}", &self.data);
        f.write_str(&temp)
    }
}

impl<T: Debug> CdlList<T> {
    pub fn new() -> CdlList<T> {
        CdlList { head: None, tail: None, size: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, t: T) {
        self.push(t, true);
    }

    pub fn push_back(&mut self, t: T) {
        self.push(t, false);
    }

    fn push(&mut self, t : T, insert_front: bool) {
        let n = Node::new(t);
        let ref_n = Rc::new(RefCell::new(n));
        
        let ref_n_clone = Rc::clone(&ref_n);
        let mut ref_n_mut = ref_n_clone.as_ref().borrow_mut();

        if self.is_empty() {
            // pointer magic
            let weak_n = Rc::downgrade(&ref_n);
            let next = LinkType::WeakLink(weak_n);
            let prev = next.clone();

            ref_n_mut.next = Some(next);
            ref_n_mut.prev = Some(prev);

            // adjust list head/tail
            self.head = Some(Rc::clone(&ref_n));
            self.tail = Some(Rc::clone(&ref_n));
        } else {
            // node->prev = current tail always
            let tail_ref = Rc::downgrade(&self.tail.as_ref().unwrap());
            ref_n_mut.prev = Some(LinkType::WeakLink(tail_ref));

            // Break and add node links based on insertion point: 
            // 1. node->next is either a strong or weak link to current head
            // 2. modify head->prev or tail->next
            // 3. change head/tail to point to new node
            match insert_front {
                true => {
                    let head_ref = Rc::clone(&self.head.as_ref().unwrap());
                    ref_n_mut.next = Some(LinkType::StrongLink(head_ref));

                    let head_ref = Rc::clone(&self.head.as_ref().unwrap());
                    let mut head_ref_mut = head_ref.as_ref().borrow_mut();
                    let weak_n = Rc::downgrade(&ref_n);
                    head_ref_mut.prev = Some(LinkType::WeakLink(weak_n));

                    self.head = Some(ref_n);
                }, 
                false => {
                    let head_ref = Rc::downgrade(&self.head.as_ref().unwrap());
                    ref_n_mut.next = Some(LinkType::WeakLink(head_ref));

                    let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                    let mut tail_ref_mut = tail_ref.as_ref().borrow_mut();
                    let weak_n = Rc::downgrade(&ref_n);
                    tail_ref_mut.next = Some(LinkType::WeakLink(weak_n));

                    self.tail = Some(ref_n);
                }
            };
        }

        self.size += 1;
    }
}