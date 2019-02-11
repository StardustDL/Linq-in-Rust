use super::*;

pub struct QSkipWhile<Q: Queryable, P> {
    query: Q,
    flag: bool,
    predicate: P,
}

impl<Q: Queryable, P> QSkipWhile<Q, P> {
    pub fn new(query: Q, predicate: P) -> Self {
        QSkipWhile {
            query,
            flag: true,
            predicate,
        }
    }
}

impl<Q: Queryable, P> Queryable for QSkipWhile<Q, P>
where
    P: FnMut(&Q::Item) -> bool,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        if !self.flag {
            return self.query.next();
        }
        while let Some(x) = self.query.next() {
            if !(self.predicate)(&x) {
                self.flag = false;
                return Some(x);
            }
        }
        None
    }
}

impl<Q: Queryable, P> IntoIterator for QSkipWhile<Q, P>
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
    fn skip_while() {
        let sqr = into_queryable(1..10).skip_while(|val| val <= &5);
        for x in sqr {
            assert!(x > 5);
        }
    }
}
