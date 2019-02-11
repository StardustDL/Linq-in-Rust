use super::*;

pub struct QConcate<Q: Queryable> {
    query1: Q,
    query2: Q,
}

impl<Q: Queryable> QConcate<Q> {
    pub fn new(query1: Q,query2:Q) -> Self {
        QConcate { query1, query2 }
    }
}

impl<Q: Queryable> Queryable for QConcate<Q>
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        match self.query1.next() {
            Some(x) => Some(x),
            None => self.query2.next(),
        }
    }
}

impl<Q: Queryable> IntoIterator for QConcate<Q>
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
    fn concate() {
        let mut sqr = into_queryable(1..5).concate(into_queryable(5..10));
        for x in 1..10 {
            assert_eq!(x, sqr.next().unwrap());
        }
        assert_eq!(None,sqr.next());
    }
}
