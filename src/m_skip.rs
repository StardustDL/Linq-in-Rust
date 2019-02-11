use super::*;

pub struct QSkip<Q: Queryable> {
    query: Q,
    count: usize,
}

impl<Q: Queryable> QSkip<Q> {
    pub fn new(query: Q, count: usize) -> Self {
        QSkip { query, count }
    }
}

impl<Q: Queryable> Queryable for QSkip<Q> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        if self.count == 0 {
            self.query.next()
        } else {
            let old_count = self.count;
            self.count = 0;
            self.element_at(old_count + 1)
        }
    }

    
}

impl<Q: Queryable> IntoIterator for QSkip<Q> {
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
    fn skip() {
        let sqr: Vec<i32> = into_queryable(1..10).skip(4).into_iter().collect();
        assert_eq!(5, sqr.len());
    }
}
