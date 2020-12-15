use super::super::iter::Enumerable;

#[derive(Clone, Debug, PartialEq)]
pub struct ParentTestStruct {
    pub structs: Vec<TestStruct>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestStruct {
    pub values: Vec<i32>,
}

#[test]
fn struct_vector_to_i32_vector() {
    let vector: Vec<TestStruct> = (1..5)
        .map(|i| TestStruct {
            values: (0..i).collect(),
        })
        .collect();

    let expected: Vec<i32> = vec![0, 0, 1, 0, 1, 2, 0, 1, 2, 3];
    let actual: Vec<i32> = vector
        .clone()
        .into_iter()
        .select_many_single(|s| s.values.clone().into_iter())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn parent_struct_vector_to_struct_vector() {
    let vector: Vec<ParentTestStruct> = (2..5)
        .map(|i| ParentTestStruct {
            structs: (1..i)
                .map(|j| TestStruct {
                    values: (0..j).collect(),
                })
                .collect(),
        })
        .collect();

    let expected: Vec<TestStruct> = vec![
        TestStruct { values: vec![0] },
        TestStruct { values: vec![0] },
        TestStruct { values: vec![0, 1] },
        TestStruct { values: vec![0] },
        TestStruct { values: vec![0, 1] },
        TestStruct {
            values: vec![0, 1, 2],
        },
    ];
    let actual: Vec<TestStruct> = vector
        .clone()
        .into_iter()
        .select_many_single(|p| p.structs.clone().into_iter())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn parent_struct_vector_to_struct_vector_to_i32_vector() {
    let vector: Vec<ParentTestStruct> = (2..5)
        .map(|i| ParentTestStruct {
            structs: (1..i)
                .map(|j| TestStruct {
                    values: (0..j).collect(),
                })
                .collect(),
        })
        .collect();

    let expected: Vec<i32> = vec![0, 0, 0, 1, 0, 0, 1, 0, 1, 2];
    let actual: Vec<i32> = vector
        .clone()
        .into_iter()
        .select_many_single(|p| p.structs.clone().into_iter())
        .select_many_single(|s| s.values.clone().into_iter())
        .collect();

    assert_eq!(expected, actual);
}
