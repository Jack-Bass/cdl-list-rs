mod cdl_list;

#[cfg(test)]
mod tests {
    use super::*;
    use cdl_list::CdlList;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

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

        println!("TEST: {:?}", list);
    }

    #[test]
    fn test_two_fronts() {
        let mut list : CdlList<u32> = CdlList::new();
        list.push_front(3);
        assert!(!list.is_empty());

        println!("TEST 1: {:?}", list);

        list.push_front(4);
        assert!(!list.is_empty());

        println!("TEST 2: {:?}", list);

        list.push_back(6);

        println!("TEST 3: {:?}", list);
    }
}
