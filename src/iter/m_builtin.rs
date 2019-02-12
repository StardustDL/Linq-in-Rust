pub type ConcateIterator<I, IT> = std::iter::Chain<I, IT>;

pub fn concate<I: Iterator, U>(
    iter: I,
    other: U,
) -> ConcateIterator<I, <U as IntoIterator>::IntoIter>
where
    U: IntoIterator<Item = I::Item>,
{
    iter.chain(other)
}

pub type WhereIterator<I, P> = std::iter::Filter<I, P>;

pub fn where_by<I: Iterator, P>(iter: I, predicate: P) -> WhereIterator<I, P>
where
    I: Sized,
    P: FnMut(&I::Item) -> bool,
{
    iter.filter(predicate)
}

pub type SelectIterator<I, F> = std::iter::Map<I, F>;

pub fn select<I: Iterator, B, F>(iter: I, f: F) -> SelectIterator<I, F>
where
    F: FnMut(I::Item) -> B,
{
    iter.map(f)
}

pub fn first<I: Iterator>(mut iter: I) -> Option<I::Item> {
    iter.next()
}

pub fn element_at<I: Iterator>(mut iter: I, index: usize) -> Option<I::Item> {
    iter.nth(index)
}

pub type ReverseIterator<I> = std::iter::Rev<I>;

pub fn reverse<I: DoubleEndedIterator>(iter: I) -> ReverseIterator<I> {
    iter.rev()
}

pub fn aggregate<I: Iterator, B, F>(iter: I, init: B, f: F) -> B
where
    F: FnMut(B, I::Item) -> B,
{
    iter.fold(init, f)
}
