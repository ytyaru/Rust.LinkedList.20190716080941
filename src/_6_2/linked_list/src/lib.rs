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
//impl<T> LinkedList<T> where T: Clone {
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
            let pre_index_ptr = self.search_from_pre_index_ptr(index); // インデックスの一つ前を取得する
            if pre_index_ptr.is_none() { return } 
            let mut target = std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, None);
            if target.is_none() { return } 
            if target.as_ref().unwrap().next.is_some() { // 末尾でなく中間なら
                let target_next = std::mem::replace(&mut target.as_mut().unwrap().next, None);
                std::mem::replace(&mut pre_index_ptr.as_mut().unwrap().next, target_next);
            }
        }
    }
    // 指定インデックスの一つ前のノードへの可変参照を返す。
    // * インデックスは1以上の想定
    // * インデックスが1でノードがひとつしかなければそれを返す
    fn search_from_pre_index_ptr(&mut self, index: u32) -> &mut Option<Box<Node<T>>> {
        fn node_of_index<T>(node: &mut Option<Box<Node<T>>>, index: u32, count: u32) -> &mut Option<Box<Node<T>>> {
            if let None = node { node }
            else {
                if index-1 == count { node }
                else { node_of_index(&mut node.as_mut().unwrap().next, index, count+1) }
            }
        }
        node_of_index(&mut self.head, index, 0)
    }
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
//    pub fn iter<T: Clone>(&self) -> Iter<'_, T> {
//    pub fn iter<T>(&self) where T: Clone -> Iter<'_, T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}
impl<T> std::iter::Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut first = std::mem::replace(&mut self.head, None);
        if first.is_none() { return None; }
        else {
            // 2番目ノードが存在するならそれを先頭にセット
            if first.as_mut().unwrap().next.is_some() {
                let second = std::mem::replace(&mut first.as_mut().unwrap().next, None);
                std::mem::replace(&mut self.head, second);
            }
            Some(first.unwrap().item)
        }
    }
}
impl<T> std::ops::Index<usize> for LinkedList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        fn get_ptr_from_idx<T>(ptr: &Option<Box<Node<T>>>, index: usize, count: usize) -> &Option<Box<Node<T>>> {
            if count < index {
                if ptr.is_some() { return get_ptr_from_idx(&ptr.as_ref().unwrap().next, index, count+1); }
                else { return ptr; }
            } else { return ptr; }
        }
        if let Some(ref _n) = get_ptr_from_idx(&self.head, index, 0) { return &(_n.item) }
        else { panic!("Out of index. index値が大きすぎます。index: {:?}", index); }
    }
}
impl<T> std::ops::IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        fn get_ptr_from_idx<T>(ptr: &mut Option<Box<Node<T>>>, index: usize, count: usize) -> &mut Option<Box<Node<T>>> {
            if count < index {
                if ptr.is_some() { return get_ptr_from_idx(&mut ptr.as_mut().unwrap().next, index, count+1); }
                else { return ptr; }
            } else { return ptr; }
        }
        if let Some(ref mut _n) = get_ptr_from_idx(&mut self.head, index, 0) { return &mut (_n.item) }
        else { panic!("Out of index. index値が大きすぎます。index: {:?}", index); }
    }
}
pub struct Iter<'a, T> { next: Option<&'a Node<T>> }
pub struct IterMut<'a, T> { next: Option<&'a mut Node<T>> }
impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}
impl<'a, T> std::iter::Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.item
        })
    }
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
    fn LikedList_push_from_index_out_of_index_0() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_from_index(0, 0); // エラーにならないこと
    }
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 1, 許容範囲: 0...0")]
    fn LikedList_push_from_index_out_of_index_1() {
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
    // remove_headはheadに要素がないとき、エラーにならず何もせずにスルーする。
    // これと同じように、remove_from_indexは指定したインデックスにノードが存在しないとき、エラーにならず何もせずにスルーする。
     #[test]
    fn LinkedList_remove_from_index_out_of_index_blank_0() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove_from_index(0); // エラーにならずスルー
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_from_index_out_of_index_blank_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove_from_index(1); // エラーにならずスルー
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_from_index_out_of_index_blank_2() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove_from_index(2); // エラーにならずスルー
        assert_eq!(list.head, None);
    }
    #[test]
    fn LinkedList_remove_from_index_out_of_index_one_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.remove_from_index(1); // エラーにならずスルー
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None, prev: None })));
    }
    #[test]
    fn LinkedList_remove_from_index_out_of_index_two_2() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.remove_from_index(2); // エラーにならずスルー
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: None, prev: None }))
        , prev: None })));
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
    #[test]
    fn LinkedList_next() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let a = list.next();
        assert_eq!(list.next(), None);

        list.push(0);
        assert_eq!(list.next(), Some(0));
        assert_eq!(list.next(), None);

        list.push(0);
        list.push(1);
        assert_eq!(list.next(), Some(0));
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), None);

        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.next(), Some(0));
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), None);

        list.push(0);
        list.push(1);
        list.push(2);
        let expecteds = vec![0,1,2];
        for expected in expecteds.iter() {
            assert_eq!(list.next(), Some(*expected));
        }
        list.push(0);
        list.push(1);
        list.push(2);
        for (i, value) in expecteds.iter().enumerate() {
            assert_eq!(list.next(), Some(expecteds[i]));
        }
        list.push(0);
        list.push(1);
        list.push(2);
        for (i, value) in list.into_iter().enumerate() {
            assert_eq!(value, expecteds[i]);
        }
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        let mut iter = list.into_iter();
        for (i, actual) in iter.by_ref().enumerate() {
            assert_eq!(actual, expecteds[i]);
            println!("{} {}", actual, expecteds[i]);
        }
        for (i, actual) in iter.by_ref().enumerate() {
            assert_eq!(actual, expecteds[i]);
            println!("{} {}", actual, expecteds[i]);
        }
    }
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 0")]
    fn LinkedList_index_out_of_index_0() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list[0];
    }
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 1")]
    fn LinkedList_index_out_of_index_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        list[1];
    }
    #[test]
    fn LinkedList_index_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        assert_eq!(list[0], 10);
    }
    #[test]
    fn LinkedList_index_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        list.push(20);
        list.push(30);
        assert_eq!(list[0], 10);
        assert_eq!(list[1], 20);
        assert_eq!(list[2], 30);
    }
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 0")]
    fn LinkedList_index_mut_out_of_index_mut_0() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list[0] = 10;
    }
    #[test]
    #[should_panic(expected = "Out of index. index値が大きすぎます。index: 1")]
    fn LinkedList_index_mut_out_of_index_mut_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        list[1] = 10;
    }
    #[test]
    fn LinkedList_index_mut_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        assert_eq!(list[0], 10);
        list[0] = 11;
        assert_eq!(list[0], 11);
        list[0] += 1;
        assert_eq!(list[0], 12);
    }
    #[test]
    fn LinkedList_index_mut_3() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        list.push(20);
        list.push(30);
        assert_eq!(list[0], 10);
        assert_eq!(list[1], 20);
        assert_eq!(list[2], 30);
        list[0] = 11;
        list[1] = 21;
        list[2] = 31;
        assert_eq!(list[0], 11);
        assert_eq!(list[1], 21);
        assert_eq!(list[2], 31);
    }
    #[test]
    fn LinkedList_iter() {
        let mut list: LinkedList<i32> = LinkedList::new();

        let expecteds = vec![10,20,30];
        for i in expecteds.iter() { list.push(*i); }
        for (i, value) in list.iter().enumerate() { assert_eq!(*value, expecteds[i]) }
        for (i, value) in list.iter().enumerate() { assert_eq!(*value, expecteds[i]) }
        list.clear();

        let datas = vec![1,2,3];
        let expecteds = vec![1,3];
        for i in datas.iter() { list.push(*i); }
        for (i, value) in list.iter().filter(|item| **item%2==1).enumerate() { assert_eq!(*value, expecteds[i]) }

//        for (i, value) in list.iter().rev().enumerate() { assert_eq!(*value, expecteds[i]) }
        for (i, value) in list.iter().cycle().enumerate() { assert_eq!(*value, expecteds[i]) }
    }
    #[test]
    fn LinkedList_iter_mut() {
        let mut list: LinkedList<i32> = LinkedList::new();

        let expecteds = vec![10,20,30];
        for i in expecteds.iter() { list.push(*i); }
        for (i, value) in list.iter_mut().enumerate() { assert_eq!(*value, expecteds[i]) }
        for (i, value) in list.iter_mut().enumerate() { assert_eq!(*value, expecteds[i]) }
        list.clear();

        let datas = vec![1,2,3];
        let expecteds = vec![1,3];
        for i in datas.iter() { list.push(*i); }
        for (i, value) in list.iter_mut().filter(|item| **item%2==1).enumerate() { assert_eq!(*value, expecteds[i]) }

        let expecteds = vec![2,3,4];
        for (i, value) in list.iter_mut().enumerate() { *value += 1; }
        for (i, value) in list.iter_mut().enumerate() { assert_eq!(*value, expecteds[i]) }
    }
}
