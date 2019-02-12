//! Linq query in Rust.

mod m_expansion;
mod m_order_by;
mod m_zip_from;

pub mod ops {
    pub use super::m_expansion::expansion as select_many;

    pub use super::m_zip_from::zip_from as select_many_zip;

    pub use super::m_order_by::order_by;
}

/// Create linq query
///
/// Use `;` to split each sub-statement.
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
        ::linq::ops::order_by($c, |$v| $mo, false).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; orderby $mo:expr; descending; select $ms:expr) =>
    {
        ::linq::ops::order_by($c, |$v| $mo, true).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; $(where $mw:expr;)+ orderby $mo:expr; select $ms:expr) =>
    {
        ::linq::ops::order_by($c.filter(|$v| true $(&& $mw)+ ), |$v| $mo, false).map(|$v| $ms)
    };
    (from $v:ident in $c:expr; $(where $mw:expr;)+ orderby $mo:expr; descending; select $ms:expr) =>
    {
        ::linq::ops::order_by($c.filter(|$v| true $(&& $mw)+ ), |$v| $mo, true).map(|$v| $ms)
    };
    (from $v0:ident in $c0:expr; from $v:ident in $c:expr; select $ms:expr) =>
    {
        ::linq::ops::select_many($c0, |$v0| $c).map(|$v| $ms)
    };
    (from $v0:ident in $c0:expr; zfrom $v:ident in $c:expr; select $ms:expr) =>
    {
        ::linq::ops::select_many_zip($c0, |$v0| $c).map(|__linq_zfrom_arg| {
            let ($v0, $v) = __linq_zfrom_arg;
            $ms
        })
    };
}
