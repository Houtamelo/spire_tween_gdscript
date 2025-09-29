class_name SpireUI

## AspectRatioContainer

static func container_do_ratio(node: AspectRatioContainer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AspectRatioContainerTweener.container_do_ratio(node, to, duration))

## Button

static func button_do_text(node: Button, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(ButtonTweener.button_do_text(node, to, duration))

## ColorRect

static func rect_do_color_r(node: ColorRect, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ColorRectTweener.rect_do_color_r(node, to, duration))

static func rect_do_color_g(node: ColorRect, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ColorRectTweener.rect_do_color_g(node, to, duration))

static func rect_do_color_b(node: ColorRect, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ColorRectTweener.rect_do_color_b(node, to, duration))

static func rect_do_color_a(node: ColorRect, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ColorRectTweener.rect_do_color_a(node, to, duration))

static func rect_do_color(node: ColorRect, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(ColorRectTweener.rect_do_color(node, to, duration))

## Control

static func do_anchor_bottom(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_anchor_bottom(node, to, duration))

static func do_anchor_left(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_anchor_left(node, to, duration))

static func do_anchor_right(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_anchor_right(node, to, duration))

static func do_anchor_top(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_anchor_top(node, to, duration))

static func control_do_rotation(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_rotation(node, to, duration))

static func control_do_rotation_degrees(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_rotation_degrees(node, to, duration))

static func do_custom_minimum_size_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_custom_minimum_size_x(node, to, duration))

static func do_custom_minimum_size_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_custom_minimum_size_y(node, to, duration))

static func control_do_position_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_position_x(node, to, duration))

static func control_do_move_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_position_x(node, to, duration))

static func control_do_position_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_position_y(node, to, duration))

static func control_do_move_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_position_y(node, to, duration))

static func control_do_global_position_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_global_position_x(node, to, duration))

static func control_do_global_position_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_global_position_y(node, to, duration))

static func do_pivot_offset_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_pivot_offset_x(node, to, duration))

static func do_pivot_offset_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_pivot_offset_y(node, to, duration))

static func control_do_scale_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_scale_x(node, to, duration))

static func control_do_scale_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.control_do_scale_y(node, to, duration))

static func do_size_x(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_size_x(node, to, duration))

static func do_size_y(node: Control, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(ControlTweener.do_size_y(node, to, duration))

static func do_custom_minimum_size(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.do_custom_minimum_size(node, to, duration))

static func control_do_position(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.control_do_position(node, to, duration))

static func control_do_move(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.control_do_position(node, to, duration))

static func control_do_global_position(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.control_do_global_position(node, to, duration))

static func do_pivot_offset(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.do_pivot_offset(node, to, duration))

static func control_do_scale(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.control_do_scale(node, to, duration))

static func do_size(node: Control, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(ControlTweener.do_size(node, to, duration))

## Label

static func label_do_visible_ratio(node: Label, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(LabelTweener.label_do_visible_ratio(node, to, duration))

static func label_do_max_lines_visible(node: Label, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(LabelTweener.label_do_max_lines_visible(node, to, duration))

static func label_do_visible_characters(node: Label, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(LabelTweener.label_do_visible_characters(node, to, duration))

static func label_do_text(node: Label, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(LabelTweener.label_do_text(node, to, duration))

## LineEdit

static func do_max_length(node: LineEdit, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(LineEditTweener.do_max_length(node, to, duration))

static func line_edit_do_text(node: LineEdit, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(LineEditTweener.line_edit_do_text(node, to, duration))

static func line_edit_do_placeholder(node: LineEdit, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(LineEditTweener.line_edit_do_placeholder(node, to, duration))

## LinkButton

static func link_button_do_text(node: LinkButton, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(LinkButtonTweener.link_button_do_text(node, to, duration))

## Range

static func do_value(node: Range, to: float, duration: float) -> TweenPropertyF64:
	return TweenPropertyF64.new(RangeTweener.do_value(node, to, duration))

static func do_ratio(node: Range, to: float, duration: float) -> TweenPropertyF64:
	return TweenPropertyF64.new(RangeTweener.do_ratio(node, to, duration))

static func do_min_value(node: Range, to: float, duration: float) -> TweenPropertyF64:
	return TweenPropertyF64.new(RangeTweener.do_min_value(node, to, duration))

static func do_max_value(node: Range, to: float, duration: float) -> TweenPropertyF64:
	return TweenPropertyF64.new(RangeTweener.do_max_value(node, to, duration))

## RichTextLabel

static func rich_do_visible_ratio(node: RichTextLabel, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(RichTextLabelTweener.rich_do_visible_ratio(node, to, duration))

static func rich_do_visible_characters(node: RichTextLabel, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(RichTextLabelTweener.rich_do_visible_characters(node, to, duration))

static func rich_do_text(node: RichTextLabel, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(RichTextLabelTweener.rich_do_text(node, to, duration))

## TextEdit

static func do_scroll_vertical(node: TextEdit, to: float, duration: float) -> TweenPropertyF64:
	return TweenPropertyF64.new(TextEditTweener.do_scroll_vertical(node, to, duration))

static func do_minimap_width(node: TextEdit, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(TextEditTweener.do_minimap_width(node, to, duration))

static func do_scroll_horizontal(node: TextEdit, to: int, duration: float) -> TweenPropertyI32:
	return TweenPropertyI32.new(TextEditTweener.do_scroll_horizontal(node, to, duration))

static func text_edit_do_text(node: TextEdit, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(TextEditTweener.text_edit_do_text(node, to, duration))

static func text_edit_do_placeholder(node: TextEdit, to: String, duration: float) -> TweenPropertyString:
	return TweenPropertyString.new(TextEditTweener.text_edit_do_placeholder(node, to, duration))

## TextureProgressBar

static func do_radial_initial_angle(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_radial_initial_angle(node, to, duration))

static func do_radial_fill_degrees(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_radial_fill_degrees(node, to, duration))

static func do_tint_under_r(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_under_r(node, to, duration))

static func do_tint_under_g(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_under_g(node, to, duration))

static func do_tint_under_b(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_under_b(node, to, duration))

static func do_tint_under_a(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_under_a(node, to, duration))

static func do_tint_over_r(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_over_r(node, to, duration))

static func do_tint_over_g(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_over_g(node, to, duration))

static func do_tint_over_b(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_over_b(node, to, duration))

static func do_tint_over_a(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_over_a(node, to, duration))

static func do_tint_progress_r(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_progress_r(node, to, duration))

static func do_tint_progress_g(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_progress_g(node, to, duration))

static func do_tint_progress_b(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_progress_b(node, to, duration))

static func do_tint_progress_a(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_tint_progress_a(node, to, duration))

static func do_radial_center_offset_x(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_radial_center_offset_x(node, to, duration))

static func do_radial_center_offset_y(node: TextureProgressBar, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(TextureProgressBarTweener.do_radial_center_offset_y(node, to, duration))

static func do_radial_center_offset(node: TextureProgressBar, to: Vector2, duration: float) -> TweenPropertyVector2:
	return TweenPropertyVector2.new(TextureProgressBarTweener.do_radial_center_offset(node, to, duration))

static func do_tint_under(node: TextureProgressBar, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(TextureProgressBarTweener.do_tint_under(node, to, duration))

static func do_tint_over(node: TextureProgressBar, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(TextureProgressBarTweener.do_tint_over(node, to, duration))

static func do_tint_progress(node: TextureProgressBar, to: Color, duration: float) -> TweenPropertyColor:
	return TweenPropertyColor.new(TextureProgressBarTweener.do_tint_progress(node, to, duration))

## VideoStreamPlayer

static func video_do_volume_db(node: VideoStreamPlayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(VideoStreamPlayerTweener.video_do_volume_db(node, to, duration))

static func video_do_volume(node: VideoStreamPlayer, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(VideoStreamPlayerTweener.video_do_volume(node, to, duration))

