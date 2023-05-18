//! This crate implements a Circular Doubly Linked List in Rust using `Rc<T>` and `RefCell<T>`.
//! 
//! Use this crate if you really like interior mutability or dislike the many unsafe blocks used in other implementations!  Of course, interior mutability introduces potential 
//! runtime panics of its own, but those should not be present currently.
//! 
//! # Usage
//! 
//! Create a list using [`cdl_list::CdlList::new()`]: 
//! 
//! ```rust
//! let mut list : CdlList<u32> = CdlList::new();
//! ```
//! 
//! The list must be mutable to add any elements to it.  Elements may be added to 
//! the head or tail of the list using [`cdl_list::CdlList::push_front()`] or 
//! [`cdl_list::CdlList::push_back()`].
//! 
//! ```rust
//! // Where A <══> B = A ⇄ B
//! list.push_front(1); // list = ╔══> 1 <══╗
//!                     //        ╚═════════╝
//! 
//! list.push_back(2);  // list = ╔══> 1 <══> 2 <══╗
//!                     //        ╚════════════════╝
//! 
//! list.push_front(3); // list = ╔══> 3 <══> 1 <══> 2 <══╗
//!                     //        ╚═══════════════════════╝
//! ```
//! 
//! To see which item is at the head or tail of the list, use 
//! [`cdl_list::CdlList::peek_front()`] or [`cdl_list::CdlList::peek_back()`].  This optionally returns a `Ref<T>`, which can be dereferenced using \*.
//! 
//! ```rust
//! let head_val = *list.peek_front().unwrap(); // head_val = 3
//! let tail_val = *list.peek_back().unwrap();  // head_val = 2
//! ```
//! 
//! To remove an item from the list, you can currently use 
//! [`cdl_list::CdlList::pop_front()`] or [`cdl_list::CdlList::pop_back()`].  This 
//! gives you ownership of the value at the head or tail of the list respectively and 
//! removes it from the list, adjusting the list's pointers appropriately.  Like 
//! peek, this returns `None` if the list is empty.
//! 
//! ```rust
//! let head = list.pop_front(); // head = Some(3)
//!                              // list = ╔══> 1 <══> 2 <══╗
//!                              //        ╚════════════════╝
//! 
//! let tail = list.pop_back();  // tail = Some(2)
//!                              // list = ╔══> 1 <══╗
//!                              //        ╚═════════╝
//! 
//! let last = list.pop_front(); // last = Some(1)
//!                              // list is empty
//! 
//! let empty = list.pop_back(); // empty = None
//! ```
//! 
//! # References
//! 
//! Some authors have some choice words to say about implementing linked lists 
//! in Rust.  Particularly, [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) warns *against* using `Rc<T>` and `RefCell<T>` 
//! to implement a doubly linked list!  Still, I learned a lot from their implementation, 
//! so even if I stuck my fingers in my ears when reading the libel printed about my 
//! beloved linked lists, I am thankful for their hard work, and it is a great read for anyone interested.
//! 
//! (Disclaimer: I never actually read all of it.  I can't borrow *all* their code, now can I?)

/// TODO: Doc
pub mod cdl_list;

#[cfg(test)]
mod tests {
    use super::*;
    use cdl_list::CdlList;

    #[test]
    fn test_cdl_empty() {
        let list : CdlList<u32> = CdlList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_one_push() {
        let mut list : CdlList<u32> = CdlList::new();
        list.push_front(3);
        assert!(!list.is_empty());

        let mut list : CdlList<u32> = CdlList::new();
        list.push_back(3);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_empty_pop() {
        let mut list : CdlList<u32> = CdlList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_one_pop() {
        let mut list : CdlList<u32> = CdlList::new();
        list.push_front(42);
        let val = list.pop_front();

        assert_eq!(val, Some(42));
        assert!(list.is_empty());
    }

    #[test]
    fn test_pop_front() {
        let mut list : CdlList<u32> = CdlList::new();
        list.push_back(4);
        list.push_back(8);
        list.push_back(15);
        list.push_back(16);
        list.push_back(23);
        list.push_back(42);

        let val = list.pop_front();
        assert_eq!(val, Some(4));
        assert_eq!(list.size(), 5);

        let val = list.pop_front();        
        assert_eq!(val, Some(8));
        assert_eq!(list.size(), 4);

        let val = list.pop_front();        
        assert_eq!(val, Some(15));
        assert_eq!(list.size(), 3);

        let val = list.pop_front();        
        assert_eq!(val, Some(16));
        assert_eq!(list.size(), 2);

        let val = list.pop_front();        
        assert_eq!(val, Some(23));
        assert_eq!(list.size(), 1);

        let val = list.pop_front();        
        assert_eq!(val, Some(42));
        assert!(list.is_empty());
    }

    #[test]
    fn test_pop_back() {
        let mut list : CdlList<u32> = CdlList::new();
        list.push_front(4);
        list.push_front(8);
        list.push_front(15);
        list.push_front(16);
        list.push_front(23);
        list.push_front(42);

        let val = list.pop_back();        
        assert_eq!(val, Some(4));
        assert_eq!(list.size(), 5);

        let val = list.pop_back();        
        assert_eq!(val, Some(8));
        assert_eq!(list.size(), 4);

        let val = list.pop_back();        
        assert_eq!(val, Some(15));
        assert_eq!(list.size(), 3);

        let val = list.pop_back();        
        assert_eq!(val, Some(16));
        assert_eq!(list.size(), 2);

        let val = list.pop_back();        
        assert_eq!(val, Some(23));
        assert_eq!(list.size(), 1);

        let val = list.pop_back();        
        assert_eq!(val, Some(42));
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        // Alternate between pushing and popping from different ends of 
        // the list to see if nodes are correctly linked after each call
        let mut list : CdlList<i32> = CdlList::new();

        list.push_front(1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.pop_front().is_none());

        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        // List = 3 <=> 2 <=> 4
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(3));
        
        list.push_front(5);
        list.push_back(6);
        list.push_front(7);
        list.push_back(8);
        list.push_front(9);

        // List = 9 <=> 7 <=> 5 <=> 2 <=> 6 <=> 8
        assert_eq!(list.pop_back(), Some(8));
        assert_eq!(list.pop_back(), Some(6));

        assert_eq!(list.pop_front(), Some(9));
        assert_eq!(list.pop_front(), Some(7));

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(5));
    }

    #[test]
    fn test_peek_empty() {
        let list : CdlList<i32> = CdlList::new();

        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
    }

    #[test]
    fn test_peek_one() {
        let mut list : CdlList<i32> = CdlList::new();
        list.push_front(1);

        let val = *list.peek_front().unwrap();
        assert_eq!(val, 1);
    }

    #[test]
    fn test_peek_front() {
        let mut list : CdlList<i32> = CdlList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        let val = *list.peek_front().unwrap();
        assert_eq!(val, 1);
    }

    #[test]
    fn test_peek_back() {
        let mut list : CdlList<i32> = CdlList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let val = *list.peek_back().unwrap();
        assert_eq!(val, 3);
    }
}
