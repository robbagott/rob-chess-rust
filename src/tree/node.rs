pub struct Node<T>
where
    T: PartialEq,
{
    id: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(id: usize, val: T) -> Self {
        Self {
            id,
            val,
            parent: None,
            children: vec![],
        }
    }

    fn eq(self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
