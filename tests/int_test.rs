use linq::linq;
use linq::linq_multi_from;

#[test]
fn try_linq(){
    let x = 1..100;
    let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
    let e: Vec<i32> = linq!(from p in x.clone(); select p * 2).collect();
    assert_eq!(e, y);
}

#[test]
fn select_many(){
    let x = 1..5;
    let y = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
    let e: Vec<i32> = linq!(from p in x.clone(); from t in 0..p; select t).collect();
    assert_eq!(e, y);
}