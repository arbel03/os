struct MemoryArea {
    start: usize,
    size: usize,
}

type Link<T> = Option<Box<Node<T>>>

struct Node<T> {
    value: T,
    link: Link<T>,
}