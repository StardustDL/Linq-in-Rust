use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    pub value: i32,
}

#[test]
fn aggregate_struct_vector_sum_of_first_ten_squares() {
    let vector: Vec<TestStruct> = (1..11).map(|i| TestStruct { value: i }).collect();

    let actual = vector
        .into_iter()
        .aggregate(0, |val, next| val + next.value * next.value);

    assert_eq!(actual, 385);
}
