#[cfg(test)]
use rs_datastruct::linked_list::LinkedList;

#[test]
fn test_new_list_is_empty() {
    let list: LinkedList<i32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_push_front() {
    let mut list = LinkedList::new();
    list.push_front(10);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
    assert_eq!(list.peek_front(), Some(&10));
}

#[test]
fn test_push_back() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek_back(), Some(&3));
}

#[test]
fn test_pop_front() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_pop_back() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_peek_front_back() {
    let mut list = LinkedList::new();
    list.push_back(42);
    assert_eq!(list.peek_front(), Some(&42));
    assert_eq!(list.peek_back(), Some(&42));
}

#[test]
fn test_get() {
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    assert_eq!(list.get(1), Some(&20));
    assert_eq!(list.get(3), None);
}

#[test]
fn test_get_mut() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    if let Some(val) = list.get_mut(1) {
        *val = 99;
    }
    assert_eq!(list.get(1), Some(&99));
}

#[test]
fn test_contains() {
    let mut list = LinkedList::new();
    list.push_back(5);
    list.push_back(10);
    assert!(list.contains(10));
    assert!(!list.contains(3));
}

#[test]
fn test_find() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    let found = list.find(|x| *x == 2);
    assert_eq!(found, Some(&2));
}

#[test]
fn test_insert() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(3);
    list.insert(1, 2);
    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&3));
}

#[test]
fn test_remove() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.remove(1), Some(2));
    assert_eq!(list.len(), 2);
    assert_eq!(list.get(1), Some(&3));
}

#[test]
fn test_reverse() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.reverse();
    assert_eq!(list.get(0), Some(&3));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&1));
}

#[test]
fn test_clear() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.clear();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

