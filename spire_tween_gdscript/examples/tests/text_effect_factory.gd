extends RefCounted

# Test fixture authoring: select a kind and explicitly configure its endpoints.
static func make(kind: int) -> SpireTextEffect:
	var effect := SpireTextEffect.new()
	effect.kind = kind
	match kind:
		Spire.TEXT_EFFECT_KIND_COLOR:
			effect.from = Color.WHITE
			effect.to = Color.WHITE
		Spire.TEXT_EFFECT_KIND_FADE:
			effect.from = 0.0
			effect.to = 1.0
		Spire.TEXT_EFFECT_KIND_OFFSET:
			effect.from = Vector2.ZERO
			effect.to = Vector2.ZERO
		Spire.TEXT_EFFECT_KIND_ROTATION:
			effect.from = Vector3.ZERO
			effect.to = Vector3.ZERO
		Spire.TEXT_EFFECT_KIND_SCALE:
			effect.from = Vector2.ONE
			effect.to = Vector2.ONE
	return effect

static func is_rotation(kind: int) -> bool:
	return kind in [Spire.TEXT_EFFECT_KIND_ROTATION, Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION, Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION]

static func is_punch(kind: int) -> bool:
	return kind in [Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET, Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION, Spire.TEXT_EFFECT_KIND_PUNCH_SCALE]

static func is_shake(kind: int) -> bool:
	return kind in [Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET, Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION, Spire.TEXT_EFFECT_KIND_SHAKE_SCALE]
