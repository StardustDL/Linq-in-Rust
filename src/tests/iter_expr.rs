use super::super::iter::Enumerable;

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
            from p in linq!(from y in x.clone(), where y % 2 == 0, select y), from t in 0..p, select t).collect();
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
    let e: Vec<i32> = linq!(from p in x.clone(), orderby -p, descending, select p * 2).collect();
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
    let e: Vec<i32> = linq!(from p in x.clone(), where p <= &5, orderby -p, select p * 2).collect();
    assert_eq!(e, y);
}

#[test]
fn distinct() {
    let x = [1, 2, 4, 2, 5, 6];
    let y: Vec<i32> = x.iter().distinct().cloned().collect();
    let e: Vec<i32> = linq!(from p in x.iter(), select distinct p).cloned().collect();
    assert_eq!(e, y);
}