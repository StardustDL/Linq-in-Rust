use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    pub value: i32,
}

#[test]
fn struct_vector_where_by_i32_field() {
    let vector: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();

    let expected: Vec<TestStruct> = (1..50).map(|i| TestStruct { value: i }).collect();
    let actual: Vec<TestStruct> = vector
        .clone()
        .into_iter()
        .where_by(|s| s.value < 50)
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn struct_vector_where_by_struct() {
    let vector: Vec<TestStruct> = (1..100).map(|i| TestStruct { value: i }).collect();

    let value = TestStruct { value: 20 };

    let expected: Vec<TestStruct> = vec![value.clone()];
    let actual: Vec<TestStruct> = vector
        .clone()
        .into_iter()
        .where_by(|s| s == &value)
        .collect();

    assert_eq!(expected, actual);
}
