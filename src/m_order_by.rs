pub enum OrderedIterator<T> {
    Inscending(std::vec::IntoIter<T>),
    Descending(std::iter::Rev<std::vec::IntoIter<T>>),
}

impl<T> Iterator for OrderedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OrderedIterator::Inscending(ins) => ins.next(),
            OrderedIterator::Descending(ins) => ins.next(),
        }
    }
}

pub fn order_by<I: Iterator, K: Ord, F>(
    iter: I,
    func: F,
    descending: bool,
) -> OrderedIterator<I::Item>
where
    F: FnMut(&I::Item) -> K,
{
    let mut temp: Vec<_> = iter.collect();
    temp.sort_by_key(func);

    if descending {
        OrderedIterator::Descending(temp.into_iter().rev())
    } else {
        OrderedIterator::Inscending(temp.into_iter())
    }
}
