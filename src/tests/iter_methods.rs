#[cfg(test)]
use super::super::iter::Enumerable;

#[test]
fn select() {
    let x = 1..100;
    let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
    let e: Vec<i32> = x.clone().select(|p| p * 2).collect();
    assert_eq!(e, y);
}

#[test]
fn select_many() {
    let x = 1..5;
    let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
    let e: Vec<i32> = x.clone().select_many_single(|p| 0..p).collect();
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
    let e: Vec<_> = x.clone().select_many(|p| 0..p, |p, t| (p, t)).collect();
    assert_eq!(e, y);
}

#[test]
fn where_by() {
    let x = 1..100;
    let y: Vec<i32> = x.clone().filter(|p| p <= &5).map(|p| p * 2).collect();
    let e: Vec<i32> = x.clone().where_by(|p| p <= &5).select(|p| p * 2).collect();
    assert_eq!(e, y);
}

#[test]
fn order_by_descending() {
    let x = 1..100;
    let mut y: Vec<i32> = x.clone().collect();
    y.sort_by_key(|t| -t);
    let y: Vec<i32> = y.into_iter().rev().map(|t| t * 2).collect();
    let e: Vec<i32> = x
        .clone()
        .order_by_descending(|p| -p)
        .select(|p| p * 2)
        .collect();
    assert_eq!(e, y);
}

#[test]
fn order_by() {
    let x = 1..100;
    let mut y: Vec<i32> = x.clone().collect();
    y.sort_by_key(|t| -t);
    let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
    let e: Vec<i32> = x.clone().order_by(|p| -p).select(|p| p * 2).collect();
    assert_eq!(e, y);
}

#[test]
fn where_order() {
    let x = 1..100;
    let mut y: Vec<i32> = x.clone().filter(|p| p <= &5).collect();
    y.sort_by_key(|t| -t);
    let y: Vec<i32> = y.into_iter().map(|t| t * 2).collect();
    let e: Vec<i32> = x
        .clone()
        .where_by(|p| p <= &5)
        .order_by(|p| -p)
        .select(|p| p * 2)
        .collect();
    assert_eq!(e, y);
}

#[test]
fn concate() {
    let x = 0..100;
    let y = 100..200;
    let e = x.concate(y);
    assert!((0..200).eq(e));
}

#[test]
fn aggregate() {
    let x = 0..10;
    assert_eq!(x.clone().aggregate(1, |b, v| b * v), x.clone().product());
}

#[test]
fn contains() {
    let x = 0..10;
    assert!(x.clone().contains(&0));
    assert!(x.clone().contains(&5));
    assert!(!x.clone().contains(&10));
}

#[test]
fn reverse() {
    let a = [1, 2, 3];

    let mut iter = a.iter().reverse();

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(iter.next(), None);
}

#[test]
fn single() {
    assert!((0..0).single().is_none());
    assert!((0..2).single().is_none());
    assert_eq!((0..1).single(), Some(0));
}

#[test]
fn first() {
    assert!((0..0).first().is_none());
    assert_eq!((0..2).first(), Some(0));
    assert_eq!((0..1).first(), Some(0));
}

#[test]
fn element_at() {
    let a = [1, 2, 3];

    assert_eq!(a.iter().element_at(0), Some(&1));
    assert_eq!(a.iter().element_at(1), Some(&2));
    assert_eq!(a.iter().element_at(2), Some(&3));
    assert_eq!(a.iter().element_at(3), None);
}

#[test]
fn distict() {
    let a = [1, 2, 3, 2, 3, 5];
    let mut iter = a.iter().distinct(); 
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&5));

    assert_eq!(iter.next(), None);
}

#[test]
fn union() {
    let a = [1, 2, 3, 2, 3, 4];
    let b = [1, 2, 2, 5, 3, 6];
    let mut iter = a.iter().union(b.iter()); 
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));

    assert_eq!(iter.next(), None);
}