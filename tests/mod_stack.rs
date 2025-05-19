#[cfg(test)]
use rs_datastruct::stack::Stack;

#[test]
fn stack_push_peek() {
    let mut stack = Stack::new();
    stack.push(1);
    assert_eq!(stack.peek().unwrap(), 1);
}

#[test]
fn stack_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    assert_eq!(stack.pop().unwrap(), 1);
    assert_eq!(stack.len(), 0);
}

#[test]
fn stack_is_empty() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());
    stack.push(1);
    assert!(!stack.is_empty());
}

#[test]
fn stack_len() {
    let mut stack = Stack::new();
    assert_eq!(stack.len(), 0);
    stack.push(1);
    assert_eq!(stack.len(), 1);
    stack.pop();
    assert_eq!(stack.len(), 0);
    stack.push(2);
    assert_eq!(stack.len(), 1);
    stack.push(3);
    assert_eq!(stack.len(), 2);
}

#[test]
fn stack_pop_should_return_none_if_empty() {
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.pop(), None);
}

#[test]
fn stack_should_be_lifo() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
}
