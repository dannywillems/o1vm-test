use core::clone::Clone;

pub struct MerkleTree<T> {
    pub root: T,
}

impl<T> MerkleTree<T> {
    fn new(root: T) -> Self {
        MerkleTree { root }
    }
}

pub fn build<T: Clone>(data: [T; 8]) -> MerkleTree<T> {
    let root = data[0].clone();
    MerkleTree::new(root)
}
