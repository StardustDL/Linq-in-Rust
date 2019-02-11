use super::*;

pub struct QSelect<Q: Queryable, F> {
    query: Q,
    selector: F,
}

impl<Q: Queryable, F> QSelect<Q, F> {
    pub fn new(query: Q, selector: F) -> Self {
        QSelect { query, selector }
    }
}

impl<B, Q: Queryable, F> Queryable for QSelect<Q, F>
where
    F: FnMut(Q::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        match self.query.next() {
            Some(x) => Some((self.selector)(x)),
            None => None,
        }
    }
}

impl<B, Q: Queryable, F> IntoIterator for QSelect<Q, F>
where
    F: FnMut(Q::Item) -> B,
{
    type Item = B;
    type IntoIter = QueryableIterator<Self>;

    fn into_iter(self) -> Self::IntoIter {
        QueryableIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn select() {
        let mut sqr = into_queryable(1..10).select(|val| val * val);
        for x in 1..10 {
            assert_eq!(x * x, sqr.next().unwrap())
        }
    }
}
