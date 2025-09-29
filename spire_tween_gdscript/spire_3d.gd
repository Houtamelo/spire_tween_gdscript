class_name Spire3D

## AudioStreamPlayer3D

static func do_volume_db(node: AudioStreamPlayer3D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayer3DTweener.do_volume_db(node, to, duration))

static func do_volume_linear(node: AudioStreamPlayer3D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayer3DTweener.do_volume_linear(node, to, duration))

static func do_pitch_scale(node: AudioStreamPlayer3D, to: float, duration: float) -> TweenPropertyF32:
	return TweenPropertyF32.new(AudioStreamPlayer3DTweener.do_pitch_scale(node, to, duration))

