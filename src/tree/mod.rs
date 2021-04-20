pub mod node;

pub use node::Node;

pub struct Tree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: PartialEq,
{
    pub fn new() -> Tree<T> {
        Tree::<T> { arena: vec![] }
    }
}
