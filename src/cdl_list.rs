//

use std::{cell::{RefCell, Ref}, rc::{Rc, Weak}, fmt::{Debug}};

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

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop(true)
    }

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
                        // 1. prev->next = head
                        // 2. head->prev = prev

                        // Isolate these two steps to avoid invalid runtime borrowing
                        {
                            // Upgrade prev to strong link
                            let strong_ref = Weak::upgrade(&wl).unwrap();
                            let mut strong_ref_mut = strong_ref.as_ref().borrow_mut();
                            let weak_head = Rc::downgrade(&self.head.as_ref().unwrap());

                            strong_ref_mut.next = Some(LinkType::WeakLink(weak_head));
                        }

                        {
                            let head_ref = Rc::clone(&self.head.as_ref().unwrap());
                            let mut head_ref_mut = head_ref.as_ref().borrow_mut();

                            head_ref_mut.prev = Some(LinkType::WeakLink(Weak::clone(&wl)));
                        }

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
}