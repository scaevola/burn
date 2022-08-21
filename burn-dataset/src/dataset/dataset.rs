use crate::DatasetIterator;

pub trait Dataset<I> {
    fn iter<'a>(&'a self) -> DatasetIterator<'a, I>;
    fn get(&self, index: usize) -> Option<I>;
    fn len(&self) -> usize;
}