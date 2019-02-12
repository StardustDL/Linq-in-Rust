use linq::linq;
use linq::iter::Enumerable;


#[test]
fn try_linq() {
    let x = 1..100;
    let y: Vec<i32> = x.clone().map(|p| p * 2).collect();
    let e: Vec<i32> = linq!(from p in x.clone(), select p * 2).collect();
    assert_eq!(e, y);
}
