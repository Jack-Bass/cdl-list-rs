mod cdl_list;

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

        // println!("TEST: {:?}", list);
    }

    #[test]
    fn test_two_fronts() {
        let mut list : CdlList<u32> = CdlList::new();
        list.push_front(3);
        assert!(!list.is_empty());

        // println!("TEST 1: {:?}", list);

        list.push_front(4);
        assert!(!list.is_empty());

        // println!("TEST 2: {:?}", list);

        list.push_back(6);

        // println!("TEST 3: {:?}", list);
    }

    #[test]
    fn test_empty_pop() {
        let mut list : CdlList<u32> = CdlList::new();
        assert_eq!(list.pop_front(), None);
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
        println!("\n\n\nTEST POP FRONT\n\n\n");

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
        // println!("VAL: {:?}", val);
        assert_eq!(list.size(), 5);

        let val = list.pop_front();
        assert!(val.is_some());
        // println!("VAL: {:?}", val);
        assert_eq!(val, Some(8));
        assert_eq!(list.size(), 4);

        let val = list.pop_front();
        assert!(val.is_some());
        // println!("VAL: {:?}", val);
        assert_eq!(val, Some(15));
        assert_eq!(list.size(), 3);

        let val = list.pop_front();
        assert!(val.is_some());
        // println!("VAL: {:?}", val);
        assert_eq!(val, Some(16));
        assert_eq!(list.size(), 2);

        let val = list.pop_front();
        assert!(val.is_some());
        // println!("VAL: {:?}", val);
        assert_eq!(val, Some(23));
        assert_eq!(list.size(), 1);

        let val = list.pop_front();
        assert!(val.is_some());
        // println!("VAL: {:?}", val);
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
        println!("VAL 2: {:?}", val);
        assert_eq!(list.size(), 5);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(8));
        println!("VAL 2: {:?}", val);
        assert_eq!(list.size(), 4);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(15));
        println!("VAL 2: {:?}", val);
        assert_eq!(list.size(), 3);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(16));
        println!("VAL 2: {:?}", val);
        assert_eq!(list.size(), 2);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(23));
        println!("VAL 2: {:?}", val);
        assert_eq!(list.size(), 1);

        let val = list.pop_back();
        assert!(val.is_some());
        assert_eq!(val, Some(42));
        println!("VAL 2: {:?}", val);
        assert_eq!(list.size(), 0);
    }
}
