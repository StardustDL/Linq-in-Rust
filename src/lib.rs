//! Linq query in Rust.

mod ops;
use ops::{
    OrderedIterator, SelectManyIterator, SelectManySingleIterator, SingleMapIterator, WhereIterator,
};

pub trait Queryable: Iterator {
    fn where_by<P>(self, predicate: P) -> WhereIterator<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool;

    fn select<TResult, F>(self, f: F) -> SingleMapIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> TResult;

    fn select_many_single<TResult, TCollection, F>(
        self,
        f: F,
    ) -> SelectManySingleIterator<Self, TCollection, F>
    where
        Self: Sized,
        TCollection: Queryable<Item = TResult>,
        F: FnMut(Self::Item) -> TCollection;

    fn select_many<TResult, TCollection, TItem, FC, FR>(
        self,
        fc: FC,
        fr: FR,
    ) -> SelectManyIterator<Self, TCollection, FC, FR>
    where
        Self: Sized,
        Self::Item: Clone,
        TCollection: Queryable<Item = TItem>,
        FC: FnMut(Self::Item) -> TCollection,
        FR: FnMut(Self::Item, TItem) -> TResult;

    fn order_by<TKey, F>(self, f: F) -> OrderedIterator<Self::Item>
    where
        Self: Sized,
        TKey: Ord,
        F: Fn(&Self::Item) -> TKey;

    fn order_by_descending<TKey, F>(self, f: F) -> OrderedIterator<Self::Item>
    where
        Self: Sized,
        TKey: Ord,
        F: Fn(&Self::Item) -> TKey;
}

impl<I, T> Queryable for I
where
    I: Iterator<Item = T>,
{
    fn where_by<P>(self, predicate: P) -> WhereIterator<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        ops::where_by(self, predicate)
    }

    fn select<TResult, F>(self, f: F) -> SingleMapIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> TResult,
    {
        ops::select(self, f)
    }

    fn select_many_single<TResult, TCollection, F>(
        self,
        f: F,
    ) -> SelectManySingleIterator<Self, TCollection, F>
    where
        Self: Sized,
        TCollection: Queryable<Item = TResult>,
        F: FnMut(Self::Item) -> TCollection,
    {
        ops::select_many_single(self, f)
    }

    fn select_many<TResult, TCollection, TItem, FC, FR>(
        self,
        fc: FC,
        fr: FR,
    ) -> SelectManyIterator<Self, TCollection, FC, FR>
    where
        Self: Sized,
        Self::Item: Clone,
        TCollection: Queryable<Item = TItem>,
        FC: FnMut(Self::Item) -> TCollection,
        FR: FnMut(Self::Item, TItem) -> TResult,
    {
        ops::select_many(self, fc, fr)
    }

    fn order_by<TKey, F>(self, f: F) -> OrderedIterator<Self::Item>
    where
        Self: Sized,
        TKey: Ord,
        F: Fn(&Self::Item) -> TKey,
    {
        ops::order_by(self, f, false)
    }

    fn order_by_descending<TKey, F>(self, f: F) -> OrderedIterator<Self::Item>
    where
        Self: Sized,
        TKey: Ord,
        F: Fn(&Self::Item) -> TKey,
    {
        ops::order_by(self, f, true)
    }
}

/// Create linq query
///
/// Use `,` to split each sub-statement.
///
/// See readme for more usage.
///
/// # Examples
///
/// ```ignore
/// use linq::linq;
///
/// let x = 1..100;
///
/// let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|t| t * 2).collect();
///
/// let e: Vec<i32> =
///     linq!(from p in x.clone(), where p <= &5, select p * 2).collect();
///
/// assert_eq!(e, y);
/// ```
#[macro_export]
macro_rules! linq {
    (from $v:ident in $c:expr, select $ms:expr) =>
    {
        $c.select(|$v| $ms)
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ select $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, orderby $mo:expr, select $ms:expr) =>
    {
        $c.order_by(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, orderby $mo:expr, descending, select $ms:expr) =>
    {
        $c.order_by_descending(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ orderby $mo:expr, select $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).order_by(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ orderby $mo:expr, descending, select $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).order_by_descending(|$v| $mo).select(|$v| $ms)
    };
    (from $v0:ident in $c0:expr, from $v:ident in $c:expr, select $ms:expr) =>
    {
        $c0.select_many_single(|$v0| $c).select(|$v| $ms)
    };
    (from $v0:ident in $c0:expr, zfrom $v:ident in $c:expr, select $ms:expr) =>
    {
        $c0.select_many(|$v0| $c, |$v0, $v| $ms)
    };
}

#[cfg(test)]
mod tests {
    use super::Queryable;

    #[test]
    fn select() {
        let x = 1..100;
        let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
        let e: Vec<i32> = linq!(from p in x.clone(), select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn select_many() {
        let x = 1..5;
        let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
        let e: Vec<i32> = linq!(from p in x.clone(), from t in 0..p, select t).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn select_many_nest() {
        let x = 1..5;
        let y = vec![0, 1, 0, 1, 2, 3];
        let e: Vec<i32> = linq!(
            from p in linq!(from y in x.clone(), where y % 2 == 0, select y), from t in 0..p, select t)
    .collect();
        assert_eq!(e, y);
    }

    #[test]
    fn select_many_zip() {
        let x = 1..5;
        let y = vec![
            (1, 0),
            (2, 0),
            (2, 1),
            (3, 0),
            (3, 1),
            (3, 2),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
        ];
        let e: Vec<_> = linq!(from p in x.clone(), zfrom t in 0..p, select (p,t)).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn where_by() {
        let x = 1..100;
        let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|p| p * 2).collect();
        let e: Vec<i32> = linq!(from p in x.clone(), where p <= &5, select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn order_by_descending() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().rev().map(|t| t * 2).collect();
        let e: Vec<i32> =
            linq!(from p in x.clone(), orderby -p, descending, select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn order_by() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
        let e: Vec<i32> = linq!(from p in x.clone(), orderby -p, select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn where_order() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().filter(|p| p <= &5).collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
        let e: Vec<i32> =
            linq!(from p in x.clone(), where p <= &5, orderby -p, select p * 2).collect();
        assert_eq!(e, y);
    }
}
