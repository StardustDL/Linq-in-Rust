use super::*;

pub struct QWhere<Q: Queryable, P> {
    query: Q,
    predicate: P,
}

impl<Q: Queryable, P> QWhere<Q, P> {
    pub fn new(query: Q, predicate: P) -> Self {
        QWhere { query, predicate }
    }
}

impl<Q: Queryable, P> Queryable for QWhere<Q, P>
where
    P: FnMut(&Q::Item) -> bool,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        while let Some(x) = self.query.next() {
            if (self.predicate)(&x) {
                return Some(x);
            }
        }
        None
    }
}

impl<Q: Queryable, P> IntoIterator for QWhere<Q, P>
where
    P: FnMut(&Q::Item) -> bool,
{
    type Item = Q::Item;
    type IntoIter = QueryableIterator<Self>;

    fn into_iter(self) -> Self::IntoIter {
        QueryableIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn where_by() {
        let sqr = into_queryable(1..10).where_by(|val| val <= &5);
        for x in sqr {
            assert!(x <= 5);
        }
    }
}
