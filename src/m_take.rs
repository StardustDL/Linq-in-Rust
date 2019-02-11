use super::*;

pub struct QTake<Q: Queryable> {
    query: Q,
    count: usize,
}

impl<Q: Queryable> QTake<Q> {
    pub fn new(query: Q, count: usize) -> Self {
        QTake { query, count }
    }
}

impl<Q: Queryable> Queryable for QTake<Q> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        if self.count == 0 {
            None
        } else {
            self.count-=1;
            self.query.next()
        }
    }
}

impl<Q: Queryable> IntoIterator for QTake<Q> {
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
    fn take() {
        let sqr: Vec<i32> = into_queryable(1..10).take(4).into_iter().collect();
        assert_eq!(4, sqr.len());
    }
}
