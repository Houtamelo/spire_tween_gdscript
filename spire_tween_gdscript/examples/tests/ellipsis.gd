extends "res://examples/tests/test_base.gd"

const CENTER := Vector2(1920, 1080) / 2.0
@onready var ball: Sprite2D = $Ball


func _ready():
	await run_test(test_ellipsis_varying_inverted)


func test_circle() -> bool:
	var tween := DoNode2D.circle(ball, CENTER, 0, 2 * PI, 256, 4.0)
	await wait_finished(tween, 4.0)
	return true


func test_circle_inverted() -> bool:
	var tween := DoNode2D.circle(ball, CENTER, 2 * PI, 0, 256, 4.0)
	await wait_finished(tween, 4.0)
	return true


func test_ellipsis() -> bool:
	var tween := DoNode2D.ellipsis(ball, CENTER, 0, 2 * PI, Vector2(256, 128+64), Vector2(256, 128+64), 4.0)
	await wait_finished(tween, 4.0)
	return true


func test_ellipsis_varying() -> bool:
	var tween := DoNode2D.ellipsis(ball, CENTER, 0, 8 * PI, Vector2(256, 128+64), Vector2(0, 0), 12.0)
	await wait_finished(tween, 12.0)
	return true


func test_ellipsis_varying_inverted() -> bool:
	var tween := DoNode2D.ellipsis(ball, CENTER, 0, 8 * PI, Vector2(0, 0), Vector2(128, 256), 12.0)
	await wait_finished(tween, 12.0)
	return true
