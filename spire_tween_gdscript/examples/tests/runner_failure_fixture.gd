extends "res://examples/tests/test_base.gd"

signal never_emitted

func test_pass() -> bool:
	assert_eq(1, 1)
	assert_ne(1, 2)
	assert_le(1, 2)
	assert_ge(2, 1)
	_assert_within_tolerance(0.0)
	return true

func test_async_pass() -> bool:
	await next_frame()
	return true

func test_helper_failure() -> bool:
	assert_eq(1, 2)
	return true

func test_other_helpers_fail() -> bool:
	assert_ne(1, 1)
	assert_le(2, 1)
	assert_ge(1, 2)
	_assert_within_tolerance(1.0)
	return true

func test_false_return() -> bool:
	return false

func test_nil_return():
	return null

func test_body_assertion() -> bool:
	assert(false, "deliberate direct assertion")
	return true

func test_async_assertion() -> bool:
	await next_frame()
	assert(false, "deliberate asynchronous assertion")
	return true

func test_callback_failure() -> bool:
	get_tree().process_frame.connect(func(): assert_eq(1, 2), CONNECT_ONE_SHOT)
	await next_frame()
	return true

func test_timeout() -> bool:
	await never_emitted
	return true

func test_paused_timeout() -> bool:
	get_tree().paused = true
	await never_emitted
	return true
