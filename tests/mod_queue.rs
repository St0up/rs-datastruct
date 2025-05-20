#[cfg(test)]
use rs_datastruct::queue::Queue;

#[test]
fn queue_enqueue_peek() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    assert_eq!(*queue.peek().unwrap(), 1);
}

#[test]
fn queue_dequeue() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    assert_eq!(queue.dequeue().unwrap(), 1);
}

#[test]
fn queue_is_empty() {
    let mut queue = Queue::new();
    assert!(queue.is_empty());
    queue.enqueue(1);
    assert!(!queue.is_empty());
}

#[test]
fn queue_len() {
    let mut queue = Queue::new();
    assert_eq!(queue.len(), 0);
    queue.enqueue(1);
    assert_eq!(queue.len(), 1);
    queue.dequeue();
    assert_eq!(queue.len(), 0);
    queue.enqueue(2);
    assert_eq!(queue.len(), 1);
    queue.enqueue(3);
}

#[test]
fn queue_dequeue_should_return_none_if_empty() {
    let mut queue: Queue<i32> = Queue::new();
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn queue_should_be_fifo() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
}
