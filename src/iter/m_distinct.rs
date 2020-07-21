#[derive(Clone)]
pub struct DistinctIterator<I>
where
    I::Item : Eq + std::hash::Hash + Copy,
    I : Iterator
{
    source : I,
    hash_map : std::collections::HashSet<I::Item>
}

impl<I> Iterator for DistinctIterator<I>
where
    I : Iterator,
    I::Item : Eq + std::hash::Hash + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.source.next() {
                Some(item) => {
                     if self.hash_map.insert(item)
                    {
                        return Some(item);
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}


pub fn distinct<I>(
    iter: I
) -> DistinctIterator<I>
where
    I : Iterator,
    I::Item : Eq + std::hash::Hash + Copy,
{
    let hash_map = std::collections::HashSet::new();
    DistinctIterator {
        hash_map : hash_map,
        source : iter
    }
}
