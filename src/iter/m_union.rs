#[derive(Clone)]
pub struct UnionIterator<I, U>
where
    I::Item : Eq + std::hash::Hash + Copy,
    I : Iterator,
    U : Iterator<Item=I::Item>,
{
    first_source : I,
    second_source : U,
    was_first_source_consumed : bool,
    hash_map : std::collections::HashSet<I::Item>
}

impl<I, U> Iterator for UnionIterator<I, U>
where
    I : Iterator,
    U : Iterator<Item=I::Item>,
    I::Item : Eq + std::hash::Hash + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if !self.was_first_source_consumed
            {
                match self.first_source.next() {
                    Some(item) => {
                         if self.hash_map.insert(item)
                        {
                            return Some(item);
                        }
                    }
                    None => {
                        self.was_first_source_consumed = true;
                    }
                }
            }
            match self.second_source.next() {
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


pub fn union<I, U>(
    iter: I,
    iter_union : U
) -> UnionIterator<I, U>
where
    I : Iterator,
    U : Iterator<Item=I::Item>,
    I::Item : Eq + std::hash::Hash + Copy,
{
    let hash_map = std::collections::HashSet::new();
    UnionIterator {
        hash_map : hash_map,
        first_source : iter,
        second_source : iter_union,
        was_first_source_consumed : false
    }
}
