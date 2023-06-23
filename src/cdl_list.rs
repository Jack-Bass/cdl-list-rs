//! A circular doubly linked list is a series of nodes where each node has a reference to 
//! the next and previous node in the list.  By being circular, the head and tail of 
//! the list point to each other, making the graph cyclic.  Below is an example of a small list: 
//! 
//! ```text
//!       head                tail
//!        │                   │
//!        V                   V
//! ┌┄┄┄>┌───┐ ──> ┌───┐ ──> ┌───┐ ┄┄┄┐
//! ┆    │ 1 │     │ 2 │     │ 3 │    ┆
//! ┆  ┌ └───┘ <┄┄ └───┘ <┄┄ └───┘ <┐ ┆
//! ┆  └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘ ┆
//! └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘
//! ```
//! 
//! This implementation makes use of `Rc<T>` and `RefCell<T>`.  To avoid creating 
//! reference cycles, I make the distinction between strong links in the graph 
//! (represented above with a solid line) and weak links (represented with a dashed 
//! line).  Generally, the next pointer is always a strong link, except for tail->next, 
//! which is always a weak pointer to the head, so no reference cycle is created.  For 
//! more on `Rc<T>`, `RefCell<T>`, and reference cycles, see [the Rust book](https://doc.rust-lang.org/book/ch15-04-rc.html).

use std::{cell::{RefCell, Ref}, rc::{Rc, Weak}, fmt::{Debug, self}};

#[derive(Debug)]
enum LinkType<T> {
    StrongLink(Rc<RefCell<T>>), 
    WeakLink(Weak<RefCell<T>>)
}

impl<T: Debug> std::clone::Clone for LinkType<Node<T>> {
    fn clone(&self) -> Self {
        match self {
            Self::StrongLink(sl) => Self::StrongLink(sl.clone()),
            Self::WeakLink(wl) => Self::WeakLink(wl.clone()),
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

impl<T: Debug> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp = format!("{:?}", &self.data);
        f.write_str(&temp)
    }
}

/// A circular doubly linked list as defined in the [module-level documentation](`crate::cdl_list`).
#[derive(Debug)]
pub struct CdlList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}

impl<T: Debug> std::ops::Drop for CdlList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Debug> fmt::Display for CdlList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "None");
        }

        write!(f, "... <=> ")?;

        // get pointer to head
        let mut node = Rc::clone(&self.head.as_ref().unwrap());

        // print each data point (by using Debug)
        let mut count: usize = 0;
        while count < self.size() {
            {
                // access reference to data
                let node_ref = node.borrow();
                write!(f, "{:?} <=> ", node_ref.data)?;
            }

            let next = node.as_ref().borrow().next.clone().unwrap();
            match next {
                LinkType::StrongLink(sl) => {
                    node = sl;
                }, 
                _ => () // on last iteration, next is a weak link
            }

            count += 1;
        }

        write!(f, "...")
    }
}

impl<T: Debug> CdlList<T> {
    /// Returns a new CdlList without any values.  List should be defined as mutable 
    /// to add elements to it.
    /// 
    /// ```rust
    /// use cdl_list_rs::cdl_list::CdlList;
    /// 
    /// let mut list : CdlList<u32> = CdlList::new();
    /// ```
    pub fn new() -> CdlList<T> {
        CdlList { head: None, tail: None, size: 0 }
    }

    /// Returns whether or not the list is empty.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let list : CdlList<u32> = CdlList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns how many elements are in the list.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// assert_eq!(list.size(), 0);
    /// 
    /// list.push_back(1);
    /// assert_eq!(list.size(), 1);
    /// 
    /// list.push_back(2);
    /// assert_eq!(list.size(), 2);
    /// 
    /// list.pop_back();
    /// assert_eq!(list.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Pushes an element to the front of the list, making it the new head and 
    /// incrementing the size of the list.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// 
    /// list.push_front(3); // list = ╔══> 3 <══╗
    ///                     //        ╚═════════╝
    /// assert_eq!(list.size(), 1);
    /// 
    /// list.push_front(2); // list = ╔══> 2 <══> 3 <══╗
    ///                     //        ╚════════════════╝
    /// assert_eq!(list.size(), 2);
    /// 
    /// list.push_front(1); // list = ╔══> 1 <══> 2 <══> 3 <══╗
    ///                     //        ╚═══════════════════════╝
    /// assert_eq!(list.size(), 3);
    /// ```
    pub fn push_front(&mut self, t: T) {
        self.push(t, true);
    }

    /// Pushes an element to the back of the list, making it the new tail and 
    /// incrementing the size of the list.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// 
    /// list.push_back(1);  // list = ╔══> 1 <══╗
    ///                     //        ╚═════════╝
    /// assert_eq!(list.size(), 1);
    /// 
    /// list.push_back(2);  // list = ╔══> 1 <══> 2 <══╗
    ///                     //        ╚════════════════╝
    /// assert_eq!(list.size(), 2);
    /// 
    /// list.push_back(3);  // list = ╔══> 1 <══> 2 <══> 3 <══╗
    ///                     //        ╚═══════════════════════╝
    /// assert_eq!(list.size(), 3);
    /// ```
    pub fn push_back(&mut self, t: T) {
        self.push(t, false);
    }

    fn push(&mut self, t : T, insert_front: bool) {
        let n = Node::new(t);
        let ref_n = Rc::new(RefCell::new(n));
        
        let ref_n_clone = Rc::clone(&ref_n);
        let mut ref_n_mut = ref_n_clone.as_ref().borrow_mut();

        if self.is_empty() {
            // node's next and prev links point to self
            // use weak links to avoid reference cycle
            let weak_n = Rc::downgrade(&ref_n);
            let next = LinkType::WeakLink(weak_n);
            let prev = next.clone();

            ref_n_mut.next = Some(next);
            ref_n_mut.prev = Some(prev);

            // adjust list head/tail
            self.head = Some(Rc::clone(&ref_n));
            self.tail = Some(ref_n);
        } else {
            let head_ref = Rc::clone(&self.head.as_ref().unwrap());
            let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());

            // node->prev = current tail always
            let weak_tail = Rc::downgrade(&tail_ref);
            ref_n_mut.prev = Some(LinkType::WeakLink(weak_tail));

            // Break and add node links based on insertion point: 
            // 1. node->next is either a strong or weak link to current head
            // 2. modify head->prev or tail->next
            // 3. change head/tail to point to new node
            if insert_front {
                // node is before head, so node->next is a strong link to head
                ref_n_mut.next = Some(LinkType::StrongLink(head_ref.clone()));

                // adjust head->prev to point to node
                let mut head_ref_mut = head_ref.as_ref().borrow_mut();
                let weak_n = Rc::downgrade(&ref_n);
                head_ref_mut.prev = Some(LinkType::WeakLink(weak_n));
                
                // special case: head->next is not accurate for size==1
                if self.size() == 1 {
                    // Fix: head->next points to self right now
                    let weak_n = Rc::downgrade(&ref_n);
                    head_ref_mut.next = Some(LinkType::WeakLink(weak_n));
                }

                // adjust head pointer
                self.head = Some(ref_n);
            } else {
                // node is after tail, so node->next is a weak link to head 
                // to avoid creating a reference cycle
                let weak_head = Rc::downgrade(&head_ref);
                ref_n_mut.next = Some(LinkType::WeakLink(weak_head));

                // adjust tail->next to point to node
                let mut tail_ref_mut = tail_ref.as_ref().borrow_mut();
                tail_ref_mut.next = Some(LinkType::StrongLink(Rc::clone(&ref_n)));

                // special case: tail->prev is not accurate for size==1
                if self.size == 1 {
                    //tail->prev = tail, which is wrong
                    let weak_n = Rc::downgrade(&ref_n);
                    tail_ref_mut.prev = Some(LinkType::WeakLink(weak_n));
                }

                // adjust tail pointer
                self.tail = Some(ref_n);
            }
        }

        self.size += 1;
    }

    /// Removes an element N from the front of the list, making the new head `N->next` and 
    /// decrementing the size of the list.  Returns the popped element if the list is 
    /// not empty.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// 
    /// list.push_front(3);
    /// list.push_front(2);
    /// list.push_front(1); // list = ╔══> 1 <══> 2 <══> 3 <══╗
    ///                     //        ╚═══════════════════════╝
    /// assert_eq!(list.size(), 3);
    /// 
    /// let v = list.pop_front(); // list = ╔══> 2 <══> 3 <══╗
    ///                           //        ╚════════════════╝
    /// assert_eq!(v, Some(1));
    /// assert_eq!(list.size(), 2);
    /// ```
    /// 
    /// If the list is empty, then `None` is returned.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// let v = list.pop_front();
    /// assert!(v.is_none());
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop(true)
    }

    /// Removes an element N from the back of the list, making the new tail `N->prev` and 
    /// decrementing the size of the list.  Returns the popped element if the list is 
    /// not empty.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// 
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);  // list = ╔══> 1 <══> 2 <══> 3 <══╗
    ///                     //        ╚═══════════════════════╝
    /// assert_eq!(list.size(), 3);
    /// 
    /// let v = list.pop_back();  // list = ╔══> 1 <══> 2 <══╗
    ///                           //        ╚════════════════╝
    /// assert_eq!(v, Some(3));
    /// assert_eq!(list.size(), 2);
    /// ```
    /// 
    /// If the list is empty, then `None` is returned.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// let v = list.pop_back();
    /// assert!(v.is_none());
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop(false)
    }

    fn pop(&mut self, pop_front: bool) -> Option<T> {
        // nothing to pop if the list is empty!
        if self.is_empty() {
            return None;
        }

        // decrement list size before value is returned
        self.size -= 1;

        // AKA: True list size is 1
        if self.is_empty() {
            // For only this scenario, strong count for the node is 2 (head and tail are strong links).
            // In order to take ownership of inner value, must drop one link
            // No links need to be changed, since list is freed.
            let head = self.head.take().unwrap();
            drop(head);
            let tail = self.tail.take().unwrap();
            let val = Rc::try_unwrap(tail).ok().unwrap().into_inner().data;

            return Some(val);
        } else {
            if pop_front {
                //pop head

                // By design, strong count for head is always 1, 
                // so we can take ownership of its node data
                let head = self.head.take().unwrap();
                let node = Rc::try_unwrap(head).ok().unwrap().into_inner();
                let next = node.next.unwrap();

                match next {
                    // By design, for (true) size > 1: 
                    // head->next is always a strong link
                    LinkType::StrongLink(sl) => {
                        // Fix links: 
                        // 1. node->next->prev = tail
                        // 2. tail->next = node->next

                        // Isolate these two steps to avoid invalid runtime borrowing
                        {
                            let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                            let mut next_ref_mut = sl.as_ref().borrow_mut();
                            let weak_tail = Rc::downgrade(&tail_ref);

                            next_ref_mut.prev = Some(LinkType::WeakLink(weak_tail));
                        }

                        {
                            let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                            let mut tail_ref_mut = tail_ref.as_ref().borrow_mut();
                            let weak_ref = Rc::downgrade(&sl);

                            tail_ref_mut.next = Some(LinkType::WeakLink(weak_ref));
                        }
                        
                        // adjust head pointer
                        self.head = Some(sl);
                    }, 
                    _ => unreachable!("head->next is always a strong link for list size > 1")
                }

                return Some(node.data);
            } else {
                //pop tail

                // By design, tail will have more strong links than head
                //
                // ==> [H] <==> ... <==> [T] <==
                // ||                         ||
                // =============================
                //
                // Tail->next = head is weak, so Rc::strong_count(&head) = 1
                // Strong tail links are tail and tail->prev->next       = 2
                //
                // Therefore, must break tail->prev->next link before consuming tail
                {
                    let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                    let tail_ref_immut = tail_ref.as_ref().borrow();
                    let prev = tail_ref_immut.prev.clone().unwrap();

                    match prev {
                        LinkType::WeakLink(wl) => {
                            let up = Weak::upgrade(&wl).unwrap();
                            let mut up_ref_mut = up.as_ref().borrow_mut();
                            let weak_head = Rc::downgrade(&self.head.as_ref().unwrap());

                            // tail->prev->next = (weak link to) head
                            up_ref_mut.next = Some(LinkType::WeakLink(weak_head));
                        }, 
                        _ => unreachable!("All prev links are weak links")
                    };
                }

                // Now, we have guaranteed Rc::strong_count(&tail) = 1, 
                // so we can take ownership of inner data node
                let tail = self.tail.take().unwrap();
                let node = Rc::try_unwrap(tail).ok().unwrap().into_inner();
                let prev = node.prev.unwrap();

                match prev {
                    LinkType::WeakLink(wl) => {
                        // fix links: 
                        // 1. head->prev = prev
                        let head_ref = Rc::clone(&self.head.as_ref().unwrap());
                        let mut head_ref_mut = head_ref.as_ref().borrow_mut();

                        head_ref_mut.prev = Some(LinkType::WeakLink(Weak::clone(&wl)));

                        // adjust tail pointer
                        let strong_ref = Weak::upgrade(&wl).unwrap();
                        self.tail = Some(strong_ref);
                    }, 
                    _ => unreachable!("All prev links are weak links")
                }

                return Some(node.data);
            }
        }
    }

    /// Optionally returns a [`std::cell::Ref<T>`], which is an immutable reference to a 
    /// value inside a [`std::cell::RefCell<T>`].  In this case, this immutably borrows 
    /// the head node's data.  Thus, it cannot change the list's data.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// 
    /// let h = *list.peek_front().unwrap();
    /// assert_eq!(h, 1);
    /// ```
    /// 
    /// Note that we can only dereference the Ref here is because u32 implements the Copy 
    /// trait.  When using a type that does not implement Copy, you would have to 
    /// use the Clone trait instead.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// # use std::cell::Ref;
    /// let mut list : CdlList<String> = CdlList::new();
    /// list.push_front(String::from("hello"));
    /// 
    /// let h : Ref<String> = list.peek_front().unwrap();
    /// assert_eq!(h.clone(), String::from("hello"));
    /// ```
    /// 
    /// Of course, if the list is empty, there is nothing to peek!
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let list : CdlList<String> = CdlList::new();
    /// assert!(list.peek_front().is_none());
    /// ```
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.peek(true)
    }

    /// Optionally returns a [`std::cell::Ref<T>`], which is an immutable reference to a 
    /// value inside a [`std::cell::RefCell<T>`].  In this case, this immutably borrows 
    /// the tail node's data.  Thus, it cannot change the list's data.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// 
    /// let t = *list.peek_back().unwrap();
    /// assert_eq!(t, 2);
    /// ```
    /// 
    /// Note that we can only dereference the Ref here is because u32 implements the Copy 
    /// trait.  When using a type that does not implement Copy, you would have to 
    /// use the Clone trait instead.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// # use std::cell::Ref;
    /// let mut list : CdlList<String> = CdlList::new();
    /// list.push_back(String::from("hello"));
    /// 
    /// let t : Ref<String> = list.peek_back().unwrap();
    /// assert_eq!(t.clone(), String::from("hello"));
    /// ```
    /// 
    /// Of course, if the list is empty, there is nothing to peek!
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let list : CdlList<String> = CdlList::new();
    /// assert!(list.peek_back().is_none());
    /// ```
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.peek(false)
    }

    fn peek(&self, peek_front: bool) -> Option<Ref<T>> {
        if self.is_empty() {
            return None;
        }

        // Graciously borrowed from the "Too Many Linked Lists" book
        if peek_front {
            return self.head.as_ref().map(|node| {
                Ref::map(node.borrow(), |node| &node.data)
            });
        } else {
            return self.tail.as_ref().map(|node| {
                Ref::map(node.borrow(), |node| &node.data)
            });
        }
    }

    /// Inserts an element in the specified position, adjusting the existing 
    /// links and incrementing the size of the list.  Insertion point starts 
    /// from 0, so `insert_at(0, T)` inserts `T` at the start of the list, 
    /// `insert_at(3, T)` inserts `T` as the fourth element in the list, etc.
    /// 
    /// ```rust
    /// # use cdl_list_rs::cdl_list::CdlList;
    /// let mut list : CdlList<u32> = CdlList::new();
    /// 
    /// list.push_back(1); // index 0
    /// list.push_back(2); // index 1
    /// list.push_back(4); // index 2
    ///                    // list = ╔══> 1 <══> 2 <══> 4 <══╗
    ///                    //        ╚═══════════════════════╝
    /// 
    /// // insert 3 at index 2
    /// list.insert_at(2, 3); // list = ╔══> 1 <══> 2 <══> 3 <══> 4 <══╗
    ///                       //        ╚══════════════════════════════╝
    /// 
    /// assert_eq!(list.size(), 4);
    /// assert_eq!(list.pop_back().unwrap(), 4);
    /// assert_eq!(list.pop_back().unwrap(), 3);
    /// ```
    pub fn insert_at(&mut self, index: usize, val : T) {
        if index == 0 {
            self.push_front(val);
            return;
        }
        if index == self.size() {
            self.push_back(val);
            return;
        }
        if index > self.size() {
            //Should probably throw an error
            return;
        }

        //create new node
        let n = Node::new(val);
        let ref_n = Rc::new(RefCell::new(n));
        let mut ref_n_mut = ref_n.as_ref().borrow_mut();

        //TODO: change starting point based on insertion point
        //      i.e. if insertion point at back, shouldn't start iterating at head
        let mut node_ref = Rc::clone(&self.head.as_ref().unwrap());
        let mut count: usize = 0;

        //get the node before insertion point
        while count < index-1 {
            let next = node_ref.borrow().next.clone().unwrap();
            match next {
                LinkType::StrongLink(sl) => {
                    node_ref = sl;
                }, 
                _ => unreachable!("All intermediary nodes have strong links to next.")
            }

            count += 1;
        }

        //need to modify node_ref->next
        let node_ref_next = node_ref.borrow().next.clone().unwrap();

        //by design, node_ref->next = n, and node_ref_next->prev = n
        match node_ref_next {
            // since n not inserted at head or tail, node_ref_next is always strong
            LinkType::StrongLink(sl) => {
                let mut node_ref_mut = node_ref.as_ref().borrow_mut();
                let mut node_ref_next_mut = sl.as_ref().borrow_mut();

                // change old links
                node_ref_mut.next = Some(LinkType::StrongLink(Rc::clone(&ref_n)));
                node_ref_next_mut.prev = Some(LinkType::WeakLink(Rc::downgrade(&ref_n)));

                // set new links
                ref_n_mut.next = Some(LinkType::StrongLink(Rc::clone(&sl)));
                ref_n_mut.prev = Some(LinkType::WeakLink(Rc::downgrade(&node_ref)));
            }, 
            _ => unreachable!("All intermediary nodes have strong links to next.")
        }

        // adjust size of the list
        self.size += 1;
    }
}