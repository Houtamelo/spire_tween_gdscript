extends "res://examples/tests/test_base.gd"

@onready var ball: Sprite2D = $Ball
var last_pos: Vector2
var prev_delta: float = 0.1


func _print_dist(delta: float):
	var dist := ball.global_position.distance_to(last_pos) / prev_delta
	last_pos = ball.global_position
	prev_delta = delta
	if !is_zero_approx(dist): print("Relative distance moved since last frame: " + str(dist))


func test_spiral_logarithmic():
	var tween := spawn_spire_at_center(0, 16*PI, Vector2(1, 1), 0, 8.0, Spire.SPIRAL_LOGARITHMIC)
		#.set_ease(Spire.EASE_OUT_QUAD)
	await wait_finished(tween, 8)
	return true


func test_spiral_logarithmic_sheared():
	var tween := spawn_spire_at_center(0, 16*PI, Vector2(1, 1), 0.5, 8.0, Spire.SPIRAL_LOGARITHMIC, Vector2(0.075, 0.075))
		#.set_ease(Spire.EASE_OUT_QUAD)
	await wait_finished(tween, 8)
	return true


func test_spiral_logarithmic_inverted():
	var tween := spawn_spire_at_center(16*PI, 0, Vector2.ONE, 0, 8.0, Spire.SPIRAL_LOGARITHMIC)
		#.set_ease(Spire.EASE_OUT_QUAD)
	await wait_finished(tween, 8)
	return true


func test_spiral_archimedean():
	var tween := spawn_spire_at_center(0, 64*PI, Vector2.ONE * 5, 0, 8.0, Spire.SPIRAL_ARCHIMEDEAN)
		#.set_ease(Spire.EASE_OUT_QUAD)
	await wait_finished(tween, 8)
	return true


func test_spiral_archimedean_sheared():
	var tween := spawn_spire_at_center(0, 64*PI, Vector2.ONE * 5, -0.3, 8.0, Spire.SPIRAL_ARCHIMEDEAN)
		#.set_ease(Spire.EASE_OUT_QUAD)
	await wait_finished(tween, 8)
	return true


func test_spiral_archimedean_inverted():
	var tween := spawn_spire_at_center(64*PI, 0, Vector2.ONE * 5, 0, 8.0, Spire.SPIRAL_ARCHIMEDEAN)
		#.set_ease(Spire.EASE_OUT_QUAD)
	await wait_finished(tween, 8)
	return true


func test_spiral_hyperbolic():
	var tween := spawn_spire_at_center(0.3, 16*PI, Vector2(512, 512), 0, 8.0, Spire.SPIRAL_HYPERBOLIC)
		#.set_ease(Spire.EASE_IN_SINE)
	await wait_finished(tween, 8)
	return true


func test_spiral_hyperbolic_sheared():
	var tween := spawn_spire_at_center(0.3, 16*PI, Vector2(512, 512), 0.7, 8.0, Spire.SPIRAL_HYPERBOLIC)
		#.set_ease(Spire.EASE_IN_SINE)
	await wait_finished(tween, 8)
	return true


func test_spiral_hyperbolic_inverted():
	var tween := spawn_spire_at_center(16*PI, 0, Vector2(256, 256), 0, 8.0, Spire.SPIRAL_HYPERBOLIC)
		#.set_ease(Spire.EASE_OUT_EXPO)
	await wait_finished(tween, 8)
	return true


func test_spiral_fermat():
	var tween := spawn_spire_at_center(0, 24*PI, Vector2(64, 64), 0, 8.0, Spire.SPIRAL_FERMAT)
	await wait_finished(tween, 8)
	return true


func test_spiral_fermat_sheared():
	var tween := spawn_spire_at_center(0, 24*PI, Vector2(64, 64), -0.5, 8.0, Spire.SPIRAL_FERMAT)
	await wait_finished(tween, 8)
	return true


func test_spiral_fermat_inverted():
	var tween := spawn_spire_at_center(24*PI, 0, Vector2(64, 64), 0, 8.0, Spire.SPIRAL_FERMAT)
	await wait_finished(tween, 8)
	return true


func spawn_spire_at_center(
	from_angle: float, 
	to_angle: float,
	scale: Vector2, 
	shear: float, 
	duration: float, 
	mode: Spire.Spiral, 
	log_growth: Vector2 = Vector2(0.05, 0.05),
) -> SpireMethodFloat:
	return DoNode2D.spiral(
		ball,
		Vector2(1920, 1080) / 2.0,
		from_angle,
		to_angle,
		scale,
		duration,
		0.0,
		shear,
		mode,
		log_growth,
	)
