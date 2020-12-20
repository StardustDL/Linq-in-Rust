use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    pub value: i32,
}

#[test]
fn struct_vector_order_by_i32_field() {
    let vector_a: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();
    let vector_d: Vec<TestStruct> = (1..100).rev().map(|i| TestStruct { value: i }).collect();

    let actual: Vec<TestStruct> = vector_d.into_iter().order_by(|s| s.value).collect();
    assert_eq!(vector_a, actual);
}

#[test]
fn struct_vector_order_by_descending_i32_field() {
    let vector_a: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();
    let vector_d: Vec<TestStruct> = (1..100).rev().map(|i| TestStruct { value: i }).collect();

    let actual: Vec<TestStruct> = vector_a
        .into_iter()
        .order_by_descending(|s| s.value)
        .collect();
    assert_eq!(vector_d, actual);
}
