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
