use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct ParentTestStruct {
    pub structs: Vec<TestStruct>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestStruct {
    pub value: i32,
}

#[test]
fn struct_vector_contains_vector() {
    let vector: Vec<TestStruct> = (1..5).map(|i| TestStruct { value: i }).collect();

    let value = TestStruct { value: 3 };

    let actual = vector.clone().into_iter().contains(&value);

    assert_eq!(true, actual);
}

#[test]
fn struct_vector_not_contains_vector() {
    let vector: Vec<TestStruct> = (1..5).map(|i| TestStruct { value: i }).collect();

    let value = TestStruct { value: 9 };

    let actual = vector.into_iter().contains(&value);

    assert_eq!(false, actual);
}

#[test]
fn parent_struct_vector_where_contains() {
    let vector: Vec<ParentTestStruct> = (1..5)
        .map(|i| ParentTestStruct {
            structs: (0..i).map(|j| TestStruct { value: j }).collect(),
        })
        .collect();

    let value = TestStruct { value: 2 };

    let expected: Vec<ParentTestStruct> = vec![
        ParentTestStruct {
            structs: vec![
                TestStruct { value: 0 },
                TestStruct { value: 1 },
                TestStruct { value: 2 },
            ],
        },
        ParentTestStruct {
            structs: vec![
                TestStruct { value: 0 },
                TestStruct { value: 1 },
                TestStruct { value: 2 },
                TestStruct { value: 3 },
            ],
        },
    ];
    let actual: Vec<ParentTestStruct> = vector
        .into_iter()
        .where_by(|p| p.structs.clone().into_iter().contains(&value))
        .collect();

    assert_eq!(expected, actual);
}