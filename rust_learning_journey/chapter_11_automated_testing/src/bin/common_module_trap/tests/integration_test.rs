mod common;
use common_module_trap::add;
#[test]
fn testing() {
    common::setup_dummy_data();
    assert_eq!(add(10, 20), 30);
}
