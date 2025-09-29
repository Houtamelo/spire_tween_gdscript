class_name Spire

## AnimationPlayer

static func do_speed_scale(node: AnimationPlayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AnimationPlayerTweener.do_speed_scale(node, to, duration))

## AudioStreamPlayer

static func do_volume_db(node: AudioStreamPlayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayerTweener.do_volume_db(node, to, duration))

static func do_volume_linear(node: AudioStreamPlayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayerTweener.do_volume_linear(node, to, duration))

static func do_pitch_scale(node: AudioStreamPlayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayerTweener.do_pitch_scale(node, to, duration))

## Camera2D

static func do_drag_horizontal_offset(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_drag_horizontal_offset(node, to, duration))

static func do_drag_vertical_offset(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_drag_vertical_offset(node, to, duration))

static func do_drag_left_margin(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_drag_left_margin(node, to, duration))

static func do_drag_right_margin(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_drag_right_margin(node, to, duration))

static func do_drag_top_margin(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_drag_top_margin(node, to, duration))

static func do_drag_bottom_margin(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_drag_bottom_margin(node, to, duration))

static func do_zoom_x(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_zoom_x(node, to, duration))

static func do_zoom_y(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_zoom_y(node, to, duration))

static func do_offset_x(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_offset_x(node, to, duration))

static func do_offset_y(node: Camera2D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(Camera2DTweener.do_offset_y(node, to, duration))

static func do_zoom(node: Camera2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Camera2DTweener.do_zoom(node, to, duration))

static func do_offset(node: Camera2D, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(Camera2DTweener.do_offset(node, to, duration))

## CanvasItem

static func do_modulate_r(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_r(node, to, duration))

static func do_color_r(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_r(node, to, duration))

static func do_modulate_g(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_g(node, to, duration))

static func do_color_g(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_g(node, to, duration))

static func do_modulate_b(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_b(node, to, duration))

static func do_color_b(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_b(node, to, duration))

static func do_modulate_a(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_a(node, to, duration))

static func do_color_a(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_modulate_a(node, to, duration))

static func do_self_modulate_r(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_r(node, to, duration))

static func do_self_color_r(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_r(node, to, duration))

static func do_self_modulate_g(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_g(node, to, duration))

static func do_self_color_g(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_g(node, to, duration))

static func do_self_modulate_b(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_b(node, to, duration))

static func do_self_color_b(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_b(node, to, duration))

static func do_self_modulate_a(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_a(node, to, duration))

static func do_self_color_a(node: CanvasItem, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasItemTweener.do_self_modulate_a(node, to, duration))

static func do_z_index(node: CanvasItem, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(CanvasItemTweener.do_z_index(node, to, duration))

static func do_modulate(node: CanvasItem, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(CanvasItemTweener.do_modulate(node, to, duration))

static func do_color(node: CanvasItem, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(CanvasItemTweener.do_modulate(node, to, duration))

static func do_self_modulate(node: CanvasItem, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(CanvasItemTweener.do_self_modulate(node, to, duration))

static func do_self_color(node: CanvasItem, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(CanvasItemTweener.do_self_modulate(node, to, duration))

## CanvasLayer

static func canvas_layer_do_rotation(node: CanvasLayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasLayerTweener.canvas_layer_do_rotation(node, to, duration))

static func canvas_layer_do_follow_viewport_scale(node: CanvasLayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasLayerTweener.canvas_layer_do_follow_viewport_scale(node, to, duration))

static func canvas_layer_do_offset_x(node: CanvasLayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasLayerTweener.canvas_layer_do_offset_x(node, to, duration))

static func canvas_layer_do_offset_y(node: CanvasLayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasLayerTweener.canvas_layer_do_offset_y(node, to, duration))

static func canvas_layer_do_scale_x(node: CanvasLayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasLayerTweener.canvas_layer_do_scale_x(node, to, duration))

static func canvas_layer_do_scale_y(node: CanvasLayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasLayerTweener.canvas_layer_do_scale_y(node, to, duration))

static func canvas_layer_do_offset(node: CanvasLayer, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(CanvasLayerTweener.canvas_layer_do_offset(node, to, duration))

static func canvas_layer_do_scale(node: CanvasLayer, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(CanvasLayerTweener.canvas_layer_do_scale(node, to, duration))

## CanvasModulate

static func canvas_modulate_do_color_r(node: CanvasModulate, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasModulateTweener.canvas_modulate_do_color_r(node, to, duration))

static func canvas_modulate_do_color_g(node: CanvasModulate, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasModulateTweener.canvas_modulate_do_color_g(node, to, duration))

static func canvas_modulate_do_color_b(node: CanvasModulate, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasModulateTweener.canvas_modulate_do_color_b(node, to, duration))

static func canvas_modulate_do_color_a(node: CanvasModulate, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(CanvasModulateTweener.canvas_modulate_do_color_a(node, to, duration))

static func canvas_modulate_do_color(node: CanvasModulate, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(CanvasModulateTweener.canvas_modulate_do_color(node, to, duration))
