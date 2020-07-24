//! Linq query in Rust.

pub mod iter;

/// Create linq query
///
/// Use `,` to split each sub-statement.
///
/// See readme for more usage.
///
/// # Examples
///
/// ```
/// use linq::linq;
/// use linq::iter::Enumerable;
///
/// let x = 1..100;
///
/// let y: Vec<i32> = x
///     .clone()
///     .where_by(|p| p <= &5)
///     .order_by(|p| -p)
///     .select(|p| p * 2)
///     .collect();
///
/// let e: Vec<i32> =
///     linq!(from p in x.clone(), where p <= &5, orderby -p, select p * 2).collect();
/// assert_eq!(e, y);
/// ```
#[macro_export]
macro_rules! linq {
    (from $v:ident in $c:expr, select $ms:expr) =>
    {
        $c.select(|$v| $ms)
    };
    (from $v:ident in $c:expr, select distinct $ms:expr) =>
    {
        $c.select(|$v| $ms).distinct()
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ select $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ select distinct $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).select(|$v| $ms).distinct()
    };
    (from $v:ident in $c:expr, orderby $mo:expr, select $ms:expr) =>
    {
        $c.order_by(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, orderby $mo:expr, select distinct $ms:expr) =>
    {
        $c.order_by(|$v| $mo).select(|$v| $ms).distinct()
    };
    (from $v:ident in $c:expr, orderby $mo:expr, descending, select $ms:expr) =>
    {
        $c.order_by_descending(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, orderby $mo:expr, descending, select distinct $ms:expr) =>
    {
        $c.order_by_descending(|$v| $mo).select(|$v| $ms).distinct()
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ orderby $mo:expr, select $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).order_by(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ orderby $mo:expr, select distinct $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).order_by(|$v| $mo).select(|$v| $ms).distinct()
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ orderby $mo:expr, descending, select $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).order_by_descending(|$v| $mo).select(|$v| $ms)
    };
    (from $v:ident in $c:expr, $(where $mw:expr,)+ orderby $mo:expr, descending, select distinct $ms:expr) =>
    {
        $c.where_by(|$v| true $(&& $mw)+ ).order_by_descending(|$v| $mo).select(|$v| $ms).distinct()
    };
    (from $v0:ident in $c0:expr, from $v:ident in $c:expr, select $ms:expr) =>
    {
        $c0.select_many_single(|$v0| $c).select(|$v| $ms)
    };
    (from $v0:ident in $c0:expr, from $v:ident in $c:expr, select distinct $ms:expr) =>
    {
        $c0.select_many_single(|$v0| $c).select(|$v| $ms).distinct()
    };
    (from $v0:ident in $c0:expr, zfrom $v:ident in $c:expr, select $ms:expr) =>
    {
        $c0.select_many(|$v0| $c, |$v0, $v| $ms)
    };
    (from $v0:ident in $c0:expr, zfrom $v:ident in $c:expr, select distinct $ms:expr) =>
    {
        $c0.select_many(|$v0| $c, |$v0, $v| $ms).distinct()
    };
}

#[cfg(test)]
mod tests;