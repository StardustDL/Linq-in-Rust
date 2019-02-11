//! Linq query in Rust.

#[macro_export]
macro_rules! linq {
    (from $v:ident; in $c:expr; select $ms:expr) =>
    {
        $c.map(|$v| $ms)
    };
    (from $v:ident; in $c:expr; $(where $mw:expr;)+ select $ms:expr) =>
    {
        $c.filter(|$v| true $(&& $mw)+ ).map(|$v| $ms)
    };
    (from $v:ident; in $c:expr; orderby $mo:expr; select $ms:expr) =>
    {
        {
            let mut temp : Vec<_> = $c.collect();
            temp.sort_by_key(|$v| $mo);
            temp.into_iter().map(|$v| $ms)
        }
    };
    (from $v:ident; in $c:expr; $(where $mw:expr;)+ orderby $mo:expr; select $ms:expr) =>
    {
        {
            let mut temp : Vec<_> = $c.filter(|$v| true $(&& $mw)+ ).collect();
            temp.sort_by_key(|$v| $mo);
            temp.into_iter().map(|$v| $ms)
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn select() {
        let x = 1..100;
        let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
        let e: Vec<i32> = linq!(from p; in x.clone(); select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn where_by() {
        let x = 1..100;
        let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|p| p * 2).collect();
        let e: Vec<i32> = linq!(from p; in x.clone(); where p <= &5; select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn order_by() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
        let e: Vec<i32> = linq!(from p; in x.clone(); orderby -p; select p * 2).collect();
        assert_eq!(e, y);
    }

    #[test]
    fn where_order() {
        let x = 1..100;
        let mut y: Vec<i32> = x.clone().filter(|p| p <= &5).collect();
        y.sort_by_key(|t| -t);
        let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
        let e: Vec<i32> =
            linq!(from p; in x.clone(); where p <= &5; orderby -p; select p * 2).collect();
        assert_eq!(e, y);
    }
}
