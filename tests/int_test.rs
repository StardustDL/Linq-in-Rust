use linq::Queryable;

#[test]
fn try_linq(){
    let iter = 1..100;

    let output : Vec<isize> = 
        linq::into_queryable(iter)
            .where_by(|val| val>=&50)
            .take(5)
            .select(|val| val*val)
            .into_iter().collect();

    assert_eq!(output, &[50*50,51*51,52*52,53*53,54*54]);
}