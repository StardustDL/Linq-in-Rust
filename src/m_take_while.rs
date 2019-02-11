use super::*;

pub struct QTakeWhile<Q: Queryable, P> {
    query: Q,
    flag: bool,
    predicate: P,
}

impl<Q: Queryable, P> QTakeWhile<Q, P> {
    pub fn new(query: Q, predicate: P) -> Self {
        QTakeWhile {
            query,
            flag: true,
            predicate,
        }
    }
}

impl<Q: Queryable, P> Queryable for QTakeWhile<Q, P>
where
    P: FnMut(&Q::Item) -> bool,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        if !self.flag {
            return None;
        }
        while let Some(x) = self.query.next() {
            if (self.predicate)(&x) {
                return Some(x);
            } else {
                self.flag = false;
                break;
            }
        }
        None
    }
}

impl<Q: Queryable, P> IntoIterator for QTakeWhile<Q, P>
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
    fn take_while() {
        let sqr = into_queryable(1..10).take_while(|val| val <= &5);
        for x in sqr {
            assert!(x <= 5);
        }
    }
}
