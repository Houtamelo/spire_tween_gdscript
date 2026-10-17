extends "res://examples/tests/test_runner.gd"

func _ready() -> void:
	test_scenes = [preload("res://examples/tests/runner_failure_fixture.tscn")]
	run_mode = RunMode.Single
	single_test_to_run = "pass"
	for arg in OS.get_cmdline_user_args():
		if arg.begins_with("--runner-self-test="):
			single_test_to_run = arg.trim_prefix("--runner-self-test=")
	if single_test_to_run == "cache":
		test_scenes = [preload("res://examples/tests/runner_cache_fixture.tscn")]
		run_mode = RunMode.FailedOnly
	await super._ready()
