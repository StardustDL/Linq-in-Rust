//! Linq query in Rust.

mod m_expansion;
pub use m_expansion::expansion as linq_multi_from;

mod m_order_by;
pub use m_order_by::order_by as linq_order_by;

/// Create linq query
///
/// Use `;` to split each sub-statement.
///
/// # Examples
///
/// ```
/// use linq::linq;
///
/// let x = 1..100;
///
/// let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|t| t * 2).collect();
///
/// let e: Vec<i32> =
///     linq!(from p in x.clone(); where p <= &5; select p * 2).collect();
///
/// assert_eq!(e, y);
/// ```
#[macro_export]
macro_rules! linq {
    (from $v:ident in $c:expr; select $ms:expr) =>
    {
        $c.map(|$v| $ms)
    };
    (from $v:ident in $c:expr; $(where $mw:expr;)+ select $ms:expr) =>
    {
        $c.filter(|$v| true $(&& $mw)+ ).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; orderby $mo:expr; select $ms:expr) =>
    {
        crate::linq_order_by($c, |$v| $mo, false).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; orderby $mo:expr; descending; select $ms:expr) =>
    {
        crate::linq_order_by($c, |$v| $mo, true).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; $(where $mw:expr;)+ orderby $mo:expr; select $ms:expr) =>
    {
        crate::linq_order_by($c.filter(|$v| true $(&& $mw)+ ), |$v| $mo, false).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; $(where $mw:expr;)+ orderby $mo:expr; descending; select $ms:expr) =>
    {
        crate::linq_order_by($c.filter(|$v| true $(&& $mw)+ ), |$v| $mo, true).map(|$v| $ms)
    };
    (from $v0:ident in $c0:expr; from $v:ident in $c:expr; select $ms:expr) =>
    {
        crate::linq_multi_from($c0, |$v0| $c).map(|$v| $ms)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn select() {
        let x = 1..100;
        let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
        let e: Vec<i32> = linq!(from p in x.clone(); select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn select_many() {
        let x = 1..5;
        let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
        let e: Vec<i32> = linq!(
            from p in x.clone(); 
            from t in 0..p; select t).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn select_many_nest() {
        let x = 1..5;
        let y = vec![0, 1, 0, 1, 2, 3];
        let e: Vec<i32> = linq!(
            from p in linq!(from y in x.clone(); where y % 2 == 0; select y); 
            from t in 0..p; select t).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn where_by() {
        let x = 1..100;
        let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|p| p * 2).collect();
        let e: Vec<i32> = linq!(from p in x.clone(); where p <= &5; select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn order_by_descending() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().rev().map(|t| t * 2).collect();
        let e: Vec<i32> =
            linq!(from p in x.clone(); orderby -p; descending; select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn order_by() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
        let e: Vec<i32> = linq!(from p in x.clone(); orderby -p; select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn where_order() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().filter(|p| p <= &5).collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
        let e: Vec<i32> =
            linq!(from p in x.clone(); where p <= &5; orderby -p; select p * 2).collect();
        assert_eq!(e, y);
    }
}
