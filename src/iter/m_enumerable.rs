use super::{m_builtin, m_order_by, m_select};
use m_builtin::{ConcateIterator,WhereIterator,SelectIterator};
use m_order_by::OrderedIterator;
use m_select::{SelectManyIterator, SelectManySingleIterator};

/// `Enumerable` is an extension of `Iterator`. It brings LINQ methods to `Iterator`.
///
/// Instead implementing `Enumerable` directly, implement `Iterator`, and `Enumerable` is auto-implemented.
pub trait Enumerable: Iterator {
    /// Filters a sequence of values based on a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use linq::iter::Enumerable;
    ///
    /// let x = 1..100;
    /// let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|p| p * 2).collect();
    /// let e: Vec<i32> = x.clone().where_by(|p| p <= &5).select(|p| p * 2).collect();
    ///
    /// assert_eq!(e, y);
    /// ```
    fn where_by<P>(self, predicate: P) -> WhereIterator<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        m_builtin::where_by(self, predicate)
    }

    /// Projects each element of a sequence into a new form.
    ///
    /// # Examples
    ///
    /// ```
    /// use linq::iter::Enumerable;
    ///
    /// let x = 1..100;
    /// let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
    /// let e: Vec<i32> = x.clone().select(|p| p * 2).collect();
    ///
    /// assert_eq!(e, y);
    /// ```
    fn select<TResult, F>(self, f: F) -> SelectIterator<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> TResult,
    {
        m_builtin::select(self, f)
    }

    /// Projects each element of a sequence to an Enumerable and flattens the resulting sequences into one sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use linq::iter::Enumerable;
    ///
    /// let x = 1..5;
    /// let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
    /// let e: Vec<i32> = x.clone().select_many_single(|p| 0..p).collect();
    ///
    /// assert_eq!(e, y);
    /// ```
    fn select_many_single<TResult, TCollection, F>(
        self,
        f: F,
    ) -> SelectManySingleIterator<Self, TCollection, F>
    where
        Self: Sized,
        TCollection: Enumerable<Item = TResult>,
        F: FnMut(Self::Item) -> TCollection,
    {
        m_select::select_many_single(self, f)
    }

    /// Projects each element of a sequence to an Enumerable, flattens the resulting sequences into one sequence, and invokes a result selector function on each element therein.
    ///
    /// # Examples
    ///
    /// ```
    /// use linq::iter::Enumerable;
    ///
    /// let x = 1..5;
    /// let y = vec![
    ///     (1, 0),
    ///     (2, 0),
    ///     (2, 1),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 2),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (4, 3),
    /// ];
    /// let e: Vec<_> = x.clone().select_many(|p| 0..p, |p, t| (p, t)).collect();
    ///
    /// assert_eq!(e, y);
    /// ```
    fn select_many<TResult, TCollection, TItem, FC, FR>(
        self,
        fc: FC,
        fr: FR,
    ) -> SelectManyIterator<Self, TCollection, FC, FR>
    where
        Self: Sized,
        Self::Item: Clone,
        TCollection: Enumerable<Item = TItem>,
        FC: FnMut(Self::Item) -> TCollection,
        FR: FnMut(Self::Item, TItem) -> TResult,
    {
        m_select::select_many(self, fc, fr)
    }

    /// Sorts the elements of a sequence in ascending order according to a key.
    ///
    /// # Examples
    ///
    /// ```
    /// use linq::iter::Enumerable;
    ///
    /// let x = 1..100;
    /// let mut y: Vec<i32> = x.clone().collect();
    /// y.sort_by_key(|t| -t);
    /// let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
    /// let e: Vec<i32> = x.clone().order_by(|p| -p).select(|p| p * 2).collect();
    ///
    /// assert_eq!(e, y);
    /// ```
    fn order_by<TKey, F>(self, f: F) -> OrderedIterator<Self::Item>
    where
        Self: Sized,
        TKey: Ord,
        F: Fn(&Self::Item) -> TKey,
    {
        m_order_by::order_by(self, f, false)
    }

    /// Sorts the elements of a sequence in descending order by using a specified comparer.
    ///
    /// # Examples
    ///
    /// ```
    /// use linq::iter::Enumerable;
    ///
    /// let x = 1..100;
    /// let mut y: Vec<i32> = x.clone().collect();
    /// y.sort_by_key(|t| -t);
    /// let y: Vec<i32> = y.into_iter().rev().map(|t| t * 2).collect();
    /// let e: Vec<i32> = x
    ///     .clone()
    ///     .order_by_descending(|p| -p)
    ///     .select(|p| p * 2)
    ///     .collect();
    ///
    /// assert_eq!(e, y);
    /// ```
    fn order_by_descending<TKey, F>(self, f: F) -> OrderedIterator<Self::Item>
    where
        Self: Sized,
        TKey: Ord,
        F: Fn(&Self::Item) -> TKey,
    {
        m_order_by::order_by(self, f, true)
    }

    fn concate<U>(self, other: U) -> ConcateIterator<Self, U>
    where
        Self: Sized,
        U: Enumerable<Item = Self::Item>,
    {
        m_builtin::concate(self, other)
    }
}

impl<I, T> Enumerable for I where I: Iterator<Item = T> {}
