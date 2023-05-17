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

        assert!(val.is_some());
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
        assert!(val.is_some());
        assert_eq!(val, Some(4));
        assert_eq!(list.size(), 5);

        let val = list.pop_front();
        assert!(val.is_some());
        assert_eq!(val, Some(8));
        assert_eq!(list.size(), 4);

        let val = list.pop_front();
        assert!(val.is_some());
        assert_eq!(val, Some(15));
        assert_eq!(list.size(), 3);

        let val = list.pop_front();
        assert!(val.is_some());
        assert_eq!(val, Some(16));
        assert_eq!(list.size(), 2);

        let val = list.pop_front();
        assert!(val.is_some());
        assert_eq!(val, Some(23));
        assert_eq!(list.size(), 1);

        let val = list.pop_front();
        assert!(val.is_some());
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
        assert!(val.is_some());
        assert_eq!(val, Some(4));
        assert_eq!(list.size(), 5);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(8));
        assert_eq!(list.size(), 4);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(15));
        assert_eq!(list.size(), 3);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(16));
        assert_eq!(list.size(), 2);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(23));
        assert_eq!(list.size(), 1);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(42));
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn push_and_pop() {
        // Alternate between pushing and popping from different ends of 
        // the list to see if nodes are correctly linked after each call
        let mut list : CdlList<i32> = CdlList::new();

        list.push_front(1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

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
}
