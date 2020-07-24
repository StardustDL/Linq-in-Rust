use super::{m_builtin, m_method, m_order_by, m_select, m_distinct, m_union};
use m_builtin::{ConcateIterator, ReverseIterator, SelectIterator, WhereIterator};
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

    /// Concatenates two sequences.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// let x = 0..100;
    /// let y = 100..200;
    /// let e = x.concate(y);
    /// assert!((0..200).eq(e));
    /// ```
    fn concate<U>(self, other: U) -> ConcateIterator<Self, U>
    where
        Self: Sized,
        U: Enumerable<Item = Self::Item>,
    {
        m_builtin::concate(self, other)
    }

    /// Returns the first element of a sequence.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// assert!((0..0).first().is_none());
    /// assert_eq!((0..2).first(), Some(0));
    /// assert_eq!((0..1).first(), Some(0));
    /// ```
    fn first(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        m_builtin::first(self)
    }

    /// Returns the index-th element of the iterator.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// let a = [1, 2, 3];
    /// 
    /// assert_eq!(a.iter().element_at(0), Some(&1));
    /// assert_eq!(a.iter().element_at(1), Some(&2));
    /// assert_eq!(a.iter().element_at(2), Some(&3));
    /// assert_eq!(a.iter().element_at(3), None);
    /// ```
    fn element_at(self, index: usize) -> Option<Self::Item>
    where
        Self: Sized,
    {
        m_builtin::element_at(self, index)
    }

    /// Returns the only element of a sequence.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// assert!((0..0).single().is_none());
    /// assert!((0..2).single().is_none());
    /// assert_eq!((0..1).single(), Some(0));
    /// ```
    fn single(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        m_method::single(self)
    }

    /// Inverts the order of the elements in a sequence.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// let a = [1, 2, 3];
    /// let mut iter = a.iter().reverse();
    /// 
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// 
    /// assert_eq!(iter.next(), None);
    /// ```
    fn reverse(self) -> ReverseIterator<Self>
    where
        Self: Sized + DoubleEndedIterator,
    {
        m_builtin::reverse(self)
    }

    /// Determines whether a sequence contains a specified element by using the default equality comparer.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// let x = 0..10;
    /// assert!(x.clone().contains(&0));
    /// assert!(x.clone().contains(&5));
    /// assert!(!x.clone().contains(&10));
    /// ```
    fn contains(self, value: &Self::Item) -> bool
    where
        Self: Sized,
        Self::Item: Eq,
    {
        m_method::contains(self, value)
    }

    /// Applies an accumulator function over a sequence. The specified seed value is used as the initial accumulator value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linq::iter::Enumerable;
    /// 
    /// let x = 0..10;
    /// assert_eq!(x.clone().aggregate(1, |b, v| b * v), x.clone().product());
    /// ```
    fn aggregate<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        m_builtin::aggregate(self, init, f)
    }

    fn distinct(self) -> m_distinct::DistinctIterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + std::hash::Hash + Copy,
    {
        m_distinct::distinct(self)
    }

    fn union<U>(self, union_with : U) -> m_union::UnionIterator<Self, U>
    where
        Self: Sized,
        Self::Item: Eq + std::hash::Hash + Copy,
        U: Enumerable<Item = Self::Item>,
    {
        m_union::union(self, union_with)
    }
    
}

impl<I, T> Enumerable for I where I: Iterator<Item = T> {}
