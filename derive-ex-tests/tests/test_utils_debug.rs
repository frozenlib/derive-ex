pub fn assert_debug_eq(a: impl std::fmt::Debug, e: impl std::fmt::Debug) {
    assert_eq!(format!("{:?}", a), format!("{:?}", e));
    assert_eq!(format!("{:#?}", a), format!("{:#?}", e));
}
