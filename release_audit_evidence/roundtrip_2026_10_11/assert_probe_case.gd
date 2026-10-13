extends "res://examples/tests/test_base.gd"
func test_deliberate_mismatch() -> bool:
    assert_eq(1, 2)
    return true
