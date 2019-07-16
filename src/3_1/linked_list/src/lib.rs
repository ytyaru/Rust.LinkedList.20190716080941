#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}
#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(item: T) -> Self { Self { item: item, next: None, prev: None } }
}
impl<T> LinkedList<T> {
    pub fn new() -> Self { Self { head: None, tail: None } }
    pub fn push(&mut self, item: T) { self.push_tail(item); }
    pub fn push_head(&mut self, item: T) {
        let new_node = Node::new(item);
        let old_head = std::mem::replace(&mut self.head, Some(Box::new(new_node)));
        std::mem::replace(&mut self.head.as_mut().unwrap().next, old_head);
    }
    pub fn push_tail(&mut self, item: T) {
        // 1. 新しい末尾ノードを指すポインタを返す(LinkedList.head or Node.next)
        fn get_tail_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n) => get_tail_node_ptr(&mut _n.next),
                _ => node
            }
        }
        // 2. 新しい末尾ノードを指すポインタを取得する
        let last = get_tail_node_ptr(&mut self.head);
        // 3. 新しい末尾ノードポインタの値として生成した新ノードを代入する
        *last = Some(Box::new(Node::new(item)));
    }
    pub fn push_from_index(&mut self, item: T, index: u32) {
        let index_ptr = self.search_from_index_ptr(index);
        let mut new_node = Some(Box::new(Node::new(item)));
        let old_idx_node = std::mem::replace(index_ptr, None);
        std::mem::replace(&mut new_node.as_mut().unwrap().next, old_idx_node);
        std::mem::replace(index_ptr, new_node);
    }
    fn search_from_index_ptr(&mut self, index: u32) -> &mut Option<Box<Node<T>>> {
        fn node_of_index<T>(node: &mut Option<Box<Node<T>>>, index: u32, count: u32) -> &mut Option<Box<Node<T>>> {
            if index == count { node }
            else {
                if let None = node { panic!("Out of index. index値が大きすぎます。index: {}, 許容範囲: 0...{}", index, count) }
                else { node_of_index(&mut node.as_mut().unwrap().next, index, count+1) }
            }
        }
        node_of_index(&mut self.head, index, 0)
    }
    pub fn remove(&mut self) { self.remove_tail(); }
    pub fn remove_head(&mut self) {
        if let Some(ref mut node) = self.head {
            let first = std::mem::replace(&mut self.head, None);
            let first = std::mem::replace(&mut self.head, first.unwrap().next); // panicked at 'called `Option::unwrap()`
//            if first.is_some() { std::mem::replace(&mut self.head, first.unwrap().next); }
        };
    }
    pub fn remove_tail(&mut self) {
        if self.head.is_none() { return; }
        // 1. 末尾ノードを指すポインタを返す(LinkedList.head or Node.next)
        fn get_booby_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n) if _n.next.is_some() => get_booby_node_ptr(&mut _n.next),
                _ => node
            }
        }
        let booby = get_booby_node_ptr(&mut self.head);
        if booby.is_some() { std::mem::replace(&mut *booby, None); }
    }
    pub fn remove_from_index(&mut self, index: u32) {
        if 0 == index { self.remove_head(); } // 先頭なら
        else {
            /*
            let pre_index_ptr = self.search_from_pre_index_ptr(index); // インデックスの一つ前を取得する
//            let mut target = std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
            if pre_index_ptr.as_ref().unwrap().next.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
//            if pre_index_ptr.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
//            if target.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
//                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target.as_mut().unwrap().next);
                std::mem::replace(
                    &mut pre_index_ptr.as_mut().unwrap().next,
                    pre_index_ptr.as_mut().unwrap().next.as_mut().unwrap().next);
//                    pre_index_ptr.as_ref().unwrap().next.as_ref().unwrap().next);
            }
            */
            let pre_index_ptr = self.search_from_pre_index_ptr(index); // インデックスの一つ前を取得する
            let mut target = std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
            if target.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
                let target_next = std::mem::replace(&mut target.as_mut().unwrap().next, None);
                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target_next);
//                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target.as_mut().unwrap().next);
            }
            /*
            let pre_index_ptr = self.search_from_pre_index_ptr(index); // インデックスの一つ前を取得する
            let mut target = std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
//            if pre_index_ptr.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
            if target.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target.as_mut().unwrap().next);
            }
            */
            /*
            if pre_index_ptr.as_ref().unwrap().next.is_none() { // 末尾なら
                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
            } else { // 中間なら
                let target = std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target.as_mut().unwrap().next);
            }
            let target = std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
            std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target.as_mut().unwrap().next);
            */
        }
        /*





        fn get_booby_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n) if _n.next.is_some() => get_booby_node_ptr(&mut _n.next),
                _ => node
            }
        }

        let index_ptr = self.search_from_index_ptr(index);
        if index_ptr.is_none() { panic!("指定した位置にノードが存在しません。 index: {}", index) }
        // 末尾なら
        if index_ptr.as_mut().unwrap().next.is_none() {
            fn get_booby_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
                match *node {
                    Some(ref mut _n) if _n.next.is_some() => get_booby_node_ptr(&mut _n.next),
                    _ => node
                }
            }
            let booby = get_booby_node_ptr(&mut self.head);
            if booby.is_some() { std::mem::replace(&mut *booby, None); }
        }
        // 中間なら（先頭でも末尾でもない）
        else {
            std::mem::(index_ptr.as_mut().unwrap().prev, index_ptr.as_mut().unwrap().next);
        }
        */
        /*
        if 0 == index { self.remove_head(); return; } // 先頭なら
        let index_ptr = self.search_from_index_ptr(index);
        if index_ptr.is_none() { panic!("指定した位置にノードが存在しません。 index: {}", index) }
        // 末尾なら
        if index_ptr.as_mut().unwrap().next.is_none() {
            fn get_booby_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
                match *node {
                    Some(ref mut _n) if _n.next.is_some() => get_booby_node_ptr(&mut _n.next),
                    _ => node
                }
            }
            let booby = get_booby_node_ptr(&mut self.head);
            if booby.is_some() { std::mem::replace(&mut *booby, None); }
        }
        // 中間なら（先頭でも末尾でもない）
        else {
            std::mem::(index_ptr.as_mut().unwrap().prev, index_ptr.as_mut().unwrap().next);
        }
        */
        /*
        let index_node = self.search_from_index_node(index);
        let index_node = std::mem::replace(&mut index_ptr, None);
        let first = std::mem::replace(&mut self.head, first.unwrap().next);
        */
        /*
        let index_ptr = self.search_from_index_ptr(index);
        let mut new_node = Some(Box::new(Node::new(item)));
        let old_idx_node = std::mem::replace(index_ptr, None);
        std::mem::replace(&mut new_node.as_mut().unwrap().next, old_idx_node);
        std::mem::replace(index_ptr, new_node);
        */
    }
    fn search_from_pre_index_ptr(&mut self, index: u32) -> &mut Option<Box<Node<T>>> {
        fn node_of_index<T>(node: &mut Option<Box<Node<T>>>, index: u32, count: u32) -> &mut Option<Box<Node<T>>> {
            if index-1 == count { node }
            else {
                if let None = node { panic!("Out of index. index値が大きすぎます。index: {}, 許容範囲: 0...{}", index, count) }
                else { node_of_index(&mut node.as_mut().unwrap().next, index, count+1) }
            }
        }
        node_of_index(&mut self.head, index, 0)
    }
    /*
    fn search_from_index_node(&mut self, index: u32) -> &mut Option<Box<Node<T>>> {
        fn node_of_index<T>(node: &mut Option<Box<Node<T>>>, index: u32, count: u32) -> &mut Option<Box<Node<T>>> {
            if index == count {
                match node {
                    Some(n) if n.next.is_some() => node
                    Some(n) if n.next.is_none() => node
                    _ => 
                }
            }
            else {
                if let None = node { panic!("Out of index. index値が大きすぎます。index: {}, 許容範囲: 0...{}", index, count) }
                else { node_of_index(&mut node.as_mut().unwrap().next, index, count+1) }
            }
        }
        node_of_index(&mut self.head, index, 0)
    }
    */
    pub fn remove_from_item(&mut self, item: T) {} // 指定要素に一致するノードを検索する必要がある
    fn search(&self, item: T) -> Result<&Option<Box<Node<T>>>, &'static str> { Err("Not found") }
    pub fn clear(&mut self) {
        // 末尾ノードから削除していく
       fn reverse_drop<T>(node: &mut Option<Box<Node<T>>>) {
            if let Some(ref mut _n) = *node {
                reverse_drop(&mut _n.next);
                std::mem::replace(&mut *node, None);
            }
        }
        reverse_drop(&mut self.head);
    }
    pub fn get(&mut self, index: u32) -> &mut T { return &mut self.head.as_mut().unwrap().item; }
    pub fn next(&mut self) -> &mut T { return &mut self.head.as_mut().unwrap().item; }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn Node_new() {
        let first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
    }
    #[test]
    fn Node_new_2() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let mut second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
        first.next = Some(Box::new(first));
    }
    #[test]
    fn Node_new_3() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let mut second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
        first.next = Some(Box::new(first));
        let mut third = Node::new(2);
        assert_eq!(third.item, 2);
        assert_eq!(third.next, None);
        second.next = Some(Box::new(third));
    }
    #[test]
    fn Node_new_string() {
        let first = Node::new(String::from("AA"));
        assert_eq!(first.item, String::from("AA"));
        assert_eq!(first.next, None);
        let second = Node::new(String::from("BB"));
        assert_eq!(second.item, String::from("BB"));
        assert_eq!(second.next, None);
    }
    #[test]
    fn LinkedList_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_push_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
    }
    #[test]
    fn LinkedList_push_2() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: Some(Box::new(Node { item: 1, next: None, prev: None })), prev: None })));
    }
    #[test]
    fn LinkedList_push_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 0, next: 
                Some(Box::new(Node { item: 1, next: 
                    Some(Box::new(Node { item:2, next: None, prev: None }))
                , prev: None
                }))
            , prev: None
            }))
        );
    }
    #[test]
    fn LikedList_push_head_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_head(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
        list.push_head(1);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 0, next: None, prev: None }))
            , prev: None
            }))
        );
        list.push_head(2);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 2, next: 
                Some(Box::new(Node { item: 1, next: 
                    Some(Box::new(Node { item:0, next: None, prev: None }))
                , prev: None
                }))
            , prev: None
            }))
        );
    }
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 1, 許容範囲: 0...0")]
    fn LikedList_push_from_index_out_of_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_from_index(0, 1);
    }
    #[test]
    fn LikedList_push_from_index_3_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_from_index(0, 0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
        list.push_from_index(1, 0);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 0, next: None, prev: None }))
            , prev: None
            }))
        );
        list.push_from_index(2, 0);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 2, next: 
                Some(Box::new(Node { item: 1, next: 
                    Some(Box::new(Node { item:0, next: None, prev: None }))
                , prev: None
                }))
            , prev: None
            }))
        );
    }
    #[test]
    fn LikedList_push_from_index_3_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_from_index(0, 0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
        list.push_from_index(1, 1);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 0, next: 
                Some(Box::new(Node { item: 1, next: None, prev: None }))
            , prev: None
            }))
        );
        list.push_from_index(2, 2);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 0, next: 
                Some(Box::new(Node { item: 1, next: 
                    Some(Box::new(Node { item:2, next: None, prev: None }))
                , prev: None
                }))
            , prev: None
            }))
        );
    }
    #[test]
    fn LikedList_push_from_index_4_middle() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_from_index(0, 0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
        list.push_from_index(1, 1);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 0, next: 
                Some(Box::new(Node { item: 1, next: None, prev: None }))
            , prev: None
            }))
        );
        list.push_from_index(2, 1);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 0, next: 
                Some(Box::new(Node { item: 2, next: 
                    Some(Box::new(Node { item:1, next: None, prev: None }))
                , prev: None
                }))
            , prev: None
            }))
        );
        list.push_from_index(3, 0);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 3, next: 
                Some(Box::new(Node { item: 0, next: 
                    Some(Box::new(Node { item: 2, next:
                        Some(Box::new(Node { item: 1, next: None, prev: None }))
                    , prev: None
                    }))
                , prev: None
                }))
            , prev: None
            }))
        );
    }
    #[test]
    fn LinkedList_new_string() {
        let list: LinkedList<String> = LinkedList::new();
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_push_3_string() {
        let mut list: LinkedList<String> = LinkedList::new();
        list.push(String::from("AA"));
        list.push(String::from("BB"));
        list.push(String::from("CC"));
        assert_eq!(list.head, 
            Some(Box::new(Node { item: String::from("AA"), next: 
                Some(Box::new(Node { item: String::from("BB"), next: 
                    Some(Box::new(Node { item: String::from("CC"), next: None, prev: None }))
                , prev: None }))
            , prev: None }))
        );
    }
    #[test]
    fn LinkedList_remove_head_0() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_head_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_head_2() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: None, prev: None }))
            
        , prev: None })));
        list.remove_head();
        assert_eq!(list.head, Some(Box::new(Node { item: 1, next: None, prev: None })));
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_head_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None, prev: None }))
            , prev: None }))
        , prev: None })));
        list.remove_head();
        assert_eq!(list.head, Some(Box::new(Node { item: 1, next: 
            Some(Box::new(Node { item: 2, next: None, prev: None }))
        , prev: None })));
        list.remove_head();
        assert_eq!(list.head, Some(Box::new(Node { item: 2, next: None, prev: None })));
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_tail_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove_tail();
        assert_eq!(list.head, None);
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None, prev: None  }))
            , prev: None }))
        , prev: None })));
        list.remove_tail();
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: None, prev: None  }))
        , prev: None })));
        list.remove_tail();
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None  })));
        list.remove_tail();
        assert_eq!(list.head, None);
    }
    
    // index=0のとき`remove_head`が実行される。Link.head=Noneのとき何もしない（エラー発さず）
    // remove_headのときはそれでいい。
    // remove_from_index(0)のときはどうすべきか？
    // 現状、ノードが一つもないとき、remove_from_index(0)はエラーなしで、remove_from_index(1)はエラーになる。挙動に一貫性がない。
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 1, 許容範囲: 0...0")]
    fn LinkedList_remove_from_index_out_of_index() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove_from_index(1);
//        list.remove_from_index(0);
    }
    #[test]
    fn LinkedList_remove_from_index_head() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None, prev: None  }))
            , prev: None }))
        , prev: None })));
        list.remove_from_index(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 1, next: 
            Some(Box::new(Node { item: 2, next: None, prev: None  }))
        , prev: None })));
        list.remove_from_index(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 2, next: None, prev: None  })));
        list.remove_from_index(0);
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_from_index_tail() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None, prev: None  }))
            , prev: None }))
        , prev: None })));
        list.remove_from_index(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: None, prev: None  }))
        , prev: None })));
        list.remove_from_index(1);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None  })));
        list.remove_from_index(0);
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_from_index_middle() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: 
                    Some(Box::new(Node { item: 3, next: 
                        Some(Box::new(Node { item: 4, next: None, prev: None  }))
                    , prev: None }))
                , prev: None }))
            , prev: None }))
        , prev: None })));
        list.remove_from_index(1);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 2, next: 
                Some(Box::new(Node { item: 3, next: 
                    Some(Box::new(Node { item: 4, next: None, prev: None  }))
                , prev: None }))
            , prev: None }))
        , prev: None })));
        list.remove_from_index(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 2, next: 
                Some(Box::new(Node { item: 4, next: None, prev: None  }))
            , prev: None }))
        , prev: None })));
    }

    #[test]
    fn LinkedList_clear_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.clear();
        assert_eq!(list.head, None);
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None, prev: None  }))
            , prev: None }))
        , prev: None })));
        list.clear();
        assert_eq!(list.head, None);
    }
}
