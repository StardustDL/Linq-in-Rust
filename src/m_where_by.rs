type WhereIterator<I, P> = std::iter::Filter<I, P>;

pub fn where_by<I: Iterator, P>(iter: I, predicate: P) -> WhereIterator<I, P>
where
    I: Sized,
    P: FnMut(&I::Item) -> bool,
{
    iter.filter(predicate)
}
