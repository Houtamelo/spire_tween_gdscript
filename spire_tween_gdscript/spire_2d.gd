class_name Spire2D

static func do_follow(node: Node2D, follow_this: Node2D, speed: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(CustomTweener.node2d_do_follow(node, follow_this, speed))

## AnimatedSprite2D

static func do_speed_scale(node: AnimatedSprite2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AnimatedSprite2DTweener.do_speed_scale(node, to, duration))

static func do_frame(node: AnimatedSprite2D, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(AnimatedSprite2DTweener.do_frame(node, to, duration))

## Area2D

static func area_do_gravity(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_gravity(node, to, duration))

static func area_do_gravity_point_unit_distance(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_gravity_point_unit_distance(node, to, duration))

static func area_do_gravity_direction_x(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_gravity_direction_x(node, to, duration))

static func area_do_gravity_direction_y(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_gravity_direction_y(node, to, duration))

static func area_do_gravity_point_center_x(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_gravity_point_center_x(node, to, duration))

static func area_do_gravity_point_center_y(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_gravity_point_center_y(node, to, duration))

static func area_do_linear_damp(node: Area2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Area2DTweener.area_do_linear_damp(node, to, duration))

static func area_do_gravity_direction(node: Area2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Area2DTweener.area_do_gravity_direction(node, to, duration))

static func area_do_gravity_point_center(node: Area2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Area2DTweener.area_do_gravity_point_center(node, to, duration))

## AudioStreamPlayer2D

static func do_volume_db(node: AudioStreamPlayer2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayer2DTweener.do_volume_db(node, to, duration))

static func do_volume_linear(node: AudioStreamPlayer2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayer2DTweener.do_volume_linear(node, to, duration))

static func do_pitch_scale(node: AudioStreamPlayer2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayer2DTweener.do_pitch_scale(node, to, duration))

## CharacterBody2D

static func chara_do_velocity_x(node: CharacterBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CharacterBody2DTweener.chara_do_velocity_x(node, to, duration))

static func chara_do_velocity_y(node: CharacterBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CharacterBody2DTweener.chara_do_velocity_y(node, to, duration))

static func chara_do_velocity(node: CharacterBody2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(CharacterBody2DTweener.chara_do_velocity(node, to, duration))

## Node2D

static func do_position_x(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_position_x(node, to, duration))

static func do_move_x(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_position_x(node, to, duration))

static func do_position_y(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_position_y(node, to, duration))

static func do_move_y(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_position_y(node, to, duration))

static func do_global_position_x(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_position_x(node, to, duration))

static func do_global_move_x(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_position_x(node, to, duration))

static func do_global_position_y(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_position_y(node, to, duration))

static func do_global_move_y(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_position_y(node, to, duration))

static func do_scale_x(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_scale_x(node, to, duration))

static func do_scale_y(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_scale_y(node, to, duration))

static func do_global_scale_x(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_scale_x(node, to, duration))

static func do_global_scale_y(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_scale_y(node, to, duration))

static func do_rotation(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_rotation(node, to, duration))

static func do_rotation_degrees(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_rotation_degrees(node, to, duration))

static func do_global_rotation(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_rotation(node, to, duration))

static func do_global_rotation_degrees(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_rotation_degrees(node, to, duration))

static func do_skew(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_skew(node, to, duration))

static func do_global_skew(node: Node2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Node2DTweener.do_global_skew(node, to, duration))

static func do_position(node: Node2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Node2DTweener.do_position(node, to, duration))

static func do_move(node: Node2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Node2DTweener.do_position(node, to, duration))

static func do_global_position(node: Node2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Node2DTweener.do_global_position(node, to, duration))

static func do_global_move(node: Node2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Node2DTweener.do_global_position(node, to, duration))

static func do_scale(node: Node2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Node2DTweener.do_scale(node, to, duration))

static func do_global_scale(node: Node2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Node2DTweener.do_global_scale(node, to, duration))

## PathFollow2D

static func do_h_offset(node: PathFollow2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(PathFollow2DTweener.do_h_offset(node, to, duration))

static func do_v_offset(node: PathFollow2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(PathFollow2DTweener.do_v_offset(node, to, duration))

static func do_progress(node: PathFollow2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(PathFollow2DTweener.do_progress(node, to, duration))

static func do_progress_ratio(node: PathFollow2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(PathFollow2DTweener.do_progress_ratio(node, to, duration))

## RigidBody2D

static func do_angular_damp(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_angular_damp(node, to, duration))

static func do_angular_velocity(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_angular_velocity(node, to, duration))

static func do_center_of_mass_x(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_center_of_mass_x(node, to, duration))

static func do_center_of_mass_y(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_center_of_mass_y(node, to, duration))

static func do_constant_force_x(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_constant_force_x(node, to, duration))

static func do_constant_force_y(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_constant_force_y(node, to, duration))

static func do_constant_torque(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_constant_torque(node, to, duration))

static func do_gravity_scale(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_gravity_scale(node, to, duration))

static func do_inertia(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_inertia(node, to, duration))

static func do_linear_damp(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_linear_damp(node, to, duration))

static func do_mass(node: RigidBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RigidBody2DTweener.do_mass(node, to, duration))

static func do_center_of_mass(node: RigidBody2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(RigidBody2DTweener.do_center_of_mass(node, to, duration))

static func do_constant_force(node: RigidBody2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(RigidBody2DTweener.do_constant_force(node, to, duration))

## StaticBody2D

static func static_do_constant_angular_velocity(node: StaticBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(StaticBody2DTweener.static_do_constant_angular_velocity(node, to, duration))

static func static_do_constant_linear_velocity_x(node: StaticBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(StaticBody2DTweener.static_do_constant_linear_velocity_x(node, to, duration))

static func static_do_constant_linear_velocity_y(node: StaticBody2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(StaticBody2DTweener.static_do_constant_linear_velocity_y(node, to, duration))

static func static_do_constant_linear_velocity(node: StaticBody2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(StaticBody2DTweener.static_do_constant_linear_velocity(node, to, duration))
