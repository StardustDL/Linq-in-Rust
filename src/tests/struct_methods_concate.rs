use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    pub value: i32,
}

#[test]
fn concate_struct_vector_with_struct_vector() {
    let vector_a: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();
    let vector_b: Vec<TestStruct> = (100..200).map(|i| TestStruct { value: i }).collect();

    let expected: Vec<TestStruct> = (1..200).map(|i| TestStruct { value: i }).collect();
    let actual: Vec<TestStruct> = vector_a.into_iter().concate(vector_b.into_iter()).collect();

    assert_eq!(actual, expected);
}
