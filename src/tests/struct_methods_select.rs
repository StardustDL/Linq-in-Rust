use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultStruct {
    pub value: i32,
}

#[test]
fn struct_vector_to_i32_vector() {
    let vector: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();

    let expected: Vec<i32> = (1..100).collect();
    let actual: Vec<i32> = vector.clone().into_iter().select(|s| s.value).collect();

    assert_eq!(expected, actual);
}

#[test]
fn struct_vector_to_i32_vector_with_multiplication() {
    let vector: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();

    let expected: Vec<i32> = (1..100).map(|i| i * 2).collect();
    let actual: Vec<i32> = vector.clone().into_iter().select(|s| s.value * 2).collect();

    assert_eq!(expected, actual);
}

#[test]
fn struct_vector_to_struct_vector() {
    let vector: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();

    let expected: Vec<TestStruct> = vector.clone();
    let actual: Vec<TestStruct> = vector.clone().into_iter().select(|s| s.clone()).collect();

    assert_eq!(expected, actual);
}

#[test]
fn select_struct_vector_to_different_struct_vector() {
    let vector: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();

    let expected: Vec<ResultStruct> = (1..100).map(|i| ResultStruct { value: i * 2 }).collect();
    let actual: Vec<ResultStruct> = vector
        .clone()
        .into_iter()
        .select(|s| ResultStruct { value: s.value * 2 })
        .collect();

    assert_eq!(expected, actual);
}
