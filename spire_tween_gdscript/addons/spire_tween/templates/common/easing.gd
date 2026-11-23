@tool
extends RefCounted
## A discrete reveal step for Typewriter. Pop and Bounce use native Ease variants.


static func typewriter(p: float) -> float:
	return 1.0 if p >= 1.0 else 0.0
