# cdl-list-rs

This crate implements a Circular Doubly Linked List in Rust using `Rc<T>` and `RefCell<T>`.

Use this crate if you really like interior mutability or dislike the many unsafe blocks used in other implementations!  Of course, interior mutability introduces potential 
runtime panics of its own, but those should not be present currently.

## Usage

Create a list using [`cdl_list::CdlList::new()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.new): 

```rust
let mut list : CdlList<u32> = CdlList::new();
```

The list must be mutable to add any elements to it.  Elements may be added to 
the head or tail of the list using [`cdl_list::CdlList::push_front()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.push_front) or 
[`cdl_list::CdlList::push_back()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.push_back).

```rust
// Where A <══> B = A ⇄ B
list.push_front(1); // list = ╔══> 1 <══╗
                    //        ╚═════════╝

list.push_back(2);  // list = ╔══> 1 <══> 2 <══╗
                    //        ╚════════════════╝

list.push_front(3); // list = ╔══> 3 <══> 1 <══> 2 <══╗
                    //        ╚═══════════════════════╝
```

Additionally, you may use [`cdl_list::CdlList::insert_at()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.insert_at) to insert an element into the list at a specific index.

```rust
                      // list = ╔══> 3 <══> 1 <══> 2 <══╗
                      //        ╚═══════════════════════╝

list.insert_at(2, 4); // list = ╔══> 3 <══> 1 <══> 4 <══> 2 <══╗
                      //        ╚══════════════════════════════╝

assert_eq!(list.size(), 4);
assert_eq!(list.pop_back(), Some(2));
assert_eq!(list.pop_back(), Some(4));
assert_eq!(list.pop_back(), Some(1));
assert_eq!(list.pop_back(), Some(3));
```

To see which item is at the head or tail of the list, use 
[`cdl_list::CdlList::peek_front()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.peek_front) or [`cdl_list::CdlList::peek_back()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.peek_back).  This optionally returns a `Ref<T>`, which can be dereferenced using \* or `clone()`.  This 
creates a copy of the value and cannot modify the list's contents!

```rust
list.push_front(1);
list.push_back(2);
list.push_front(3);
let head_val = *list.peek_front().unwrap();        // head_val = 3
let tail_val = list.peek_back().unwrap().clone();  // tail_val = 2
```

To remove an item from the list, you can currently use 
[`cdl_list::CdlList::pop_front()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.pop_front) or [`cdl_list::CdlList::pop_back()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.pop_back).  This 
gives you ownership of the value at the head or tail of the list respectively and 
removes it from the list, adjusting the list's pointers appropriately.  Like 
peek, this returns `None` if the list is empty.

```rust
let head = list.pop_front(); // head = Some(3)
                             // list = ╔══> 1 <══> 2 <══╗
                             //        ╚════════════════╝

let tail = list.pop_back();  // tail = Some(2)
                             // list = ╔══> 1 <══╗
                             //        ╚═════════╝

let last = list.pop_front(); // last = Some(1)
                             // list is empty

let empty = list.pop_back(); // empty = None
```

Additionally, you may use [`cdl_list::CdlList::remove_at()`](https://docs.rs/cdl-list-rs/0.1.0/cdl_list_rs/cdl_list/struct.CdlList.html#method.remove_at) to remove an element from the list at a specific index.

```rust
list.push_front(1);
list.push_back(2);
list.push_front(3);

// List:  3, 1, 2
// Index: 0, 1, 2

assert_eq!(list.remove_at(1), Some(1));
```

## References

Some authors have some choice words to say about implementing linked lists 
in Rust.  Particularly, [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) warns *against* using `Rc<T>` and `RefCell<T>` 
to implement a doubly linked list!  Still, I learned a lot from their implementation, 
so even if I stuck my fingers in my ears when reading the libel printed about my 
beloved linked lists, I am thankful for their hard work, and it is a great read for anyone interested.

(Disclaimer: I never actually read all of it.  I can't borrow *all* their code, now can I?)
