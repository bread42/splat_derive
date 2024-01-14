use splat_derive::Splat;

#[derive(Splat)]
struct TestStruct {
    field_one: u16,
    field_two: u16,
    field_three: u16,
}

#[test]
fn struct_fields() {
    let test_struct = TestStruct::splat(250);
    assert_eq!(test_struct.field_one, 250);
    assert_eq!(test_struct.field_two, 250);
    assert_eq!(test_struct.field_three, 250);
}

#[derive(Splat)]
struct TestTupleStruct(i32, i32, i32);

#[test]
fn tuple_struct_fields() {
    let test_tuple_struct = TestTupleStruct::splat(-1_000_000);
    assert_eq!(test_tuple_struct.0, -1_000_000);
    assert_eq!(test_tuple_struct.1, -1_000_000);
    assert_eq!(test_tuple_struct.2, -1_000_000);
}
