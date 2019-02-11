//! Linq query in Rust.

mod m_select;
pub use m_select::QSelect;

mod m_where;
pub use m_where::QWhere;

mod m_skip;
pub use m_skip::QSkip;

mod m_take;
pub use m_take::QTake;

mod m_skip_while;
pub use m_skip_while::QSkipWhile;

mod m_take_while;
pub use m_take_while::QTakeWhile;

mod m_concate;
use m_concate::QConcate;

pub trait Queryable {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn element_at(&mut self, index: usize) -> Option<Self::Item> {
        if index > 0 {
            for _ in 0..index - 1 {
                self.next();
            }
        }
        self.next()
    }

    fn count(mut self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        while self.next().is_some() {
            count += 1;
        }
        count
    }

    fn select<B, F>(self, selector: F) -> QSelect<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        QSelect::new(self, selector)
    }

    fn where_by<P>(self, predicate: P) -> QWhere<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        QWhere::new(self, predicate)
    }

    fn skip(self, count: usize) -> QSkip<Self>
    where
        Self: Sized,
    {
        QSkip::new(self, count)
    }

    fn skip_while<P>(self, predicate: P) -> QSkipWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        QSkipWhile::new(self, predicate)
    }

    fn take(self, count: usize) -> QTake<Self>
    where
        Self: Sized,
    {
        QTake::new(self, count)
    }

    fn take_while<P>(self, predicate: P) -> QTakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        QTakeWhile::new(self, predicate)
    }

    fn concate(self, other: Self) -> QConcate<Self>
    where
        Self: Sized,
    {
        QConcate::new(self, other)
    }
}

pub struct IteratorQueryable<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Queryable for IteratorQueryable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.iter.next()
    }
}

pub fn into_queryable<I: Iterator>(iter: I) -> IteratorQueryable<I> {
    IteratorQueryable { iter }
}

pub struct QueryableIterator<Q: Queryable> {
    query: Q,
}

impl<Q: Queryable> QueryableIterator<Q> {
    pub fn new(query: Q) -> Self {
        QueryableIterator { query }
    }
}

impl<Q: Queryable> Iterator for QueryableIterator<Q> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        self.query.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let sqr = into_queryable(1..10).where_by(|val| val <= &5);
        assert_eq!(5, sqr.count());
    }

    #[test]
    fn element_at() {
        let mut sqr = into_queryable(1..10).where_by(|val| val <= &5);
        assert_eq!(Some(3), sqr.element_at(3));
        assert_eq!(None, sqr.element_at(3));
    }
}

#[macro_export]
macro_rules! query {
  (from $v:ident in $c:ident select $ms:expr) =>
  { $c.map(|$v| $ms) };
  (from $v:ident in $c:ident $(where $mw:expr;)+ select $ms:expr) =>
  { $c.filter(|$v| ($(&& $mv)*) ).map(|$v| $ms) };
}