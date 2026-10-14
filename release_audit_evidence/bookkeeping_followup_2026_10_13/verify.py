import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).parent
REPO = Path('/home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin')
GODOT = '/home/houtamelo/bin/godot'
ENV = dict(os.environ, CI='true', GIT_TERMINAL_PROMPT='0', GIT_EDITOR='true', GIT_PAGER='cat')

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def run(label, cmd, cwd=REPO, timeout=240):
    log = ROOT / (label + '.log')
    env = dict(ENV, XDG_DATA_HOME=str(ROOT / 'user-data' / label))
    started = time.monotonic()
    with log.open('w') as output:
        try:
            result = subprocess.run(cmd, cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT, timeout=timeout)
            code = result.returncode
        except subprocess.TimeoutExpired:
            code = 124
    body = log.read_text()
    result = {
        'label': label,
        'command': cmd,
        'exit_code': code,
        'seconds': round(time.monotonic() - started, 3),
        'results': re.findall(r'^.*(?:_RESULT|RESULT).*$', body, flags=re.MULTILINE),
        'script_errors': body.count('SCRIPT ERROR:'),
        'panic_reports': len(re.findall(r"thread .* panicked at", body)),
    }
    print(json.dumps(result), flush=True)
    return result

builds = []
commands = [
    ('workspace-check', ['cargo', 'check', '--workspace', '--all-targets', '--locked']),
    ('native-debug-build', ['cargo', 'build', '-p', 'spire_tween_rust', '--locked', '--features', 'spire_tween/include_gdscript_bridge']),
    ('native-release-build', ['cargo', 'build', '-p', 'spire_tween_rust', '--release', '--locked', '--features', 'spire_tween/include_gdscript_bridge']),
    ('addon-debug-build', ['cargo', 'build', '-p', 'spire_tween', '--locked', '--features', 'include_gdext_lib,include_gdscript_bridge']),
    ('addon-release-build', ['cargo', 'build', '-p', 'spire_tween', '--release', '--locked', '--features', 'include_gdext_lib,include_gdscript_bridge']),
]
if '--runtime-only' in sys.argv:
    commands = []
for label, cmd in commands:
    result = run(label, cmd)
    builds.append(result)
    (ROOT / 'build-results.json').write_text(json.dumps(builds, indent=2) + '\n')
    if result['exit_code']:
        raise SystemExit(result['exit_code'])

native_scenes = [
    ('bookkeeping', 'bookkeeping_runner.tscn', 11),
    ('ownership', 'ownership_runner.tscn', 10),
    ('node-binding', 'node_binding_runner.tscn', 11),
    ('sequence-cycles', 'sequence_cycles_runner.tscn', 10),
    ('delayed-call', 'delayed_call_validity_runner.tscn', 9),
    ('api-compatibility', 'api_compatibility_runner.tscn', 5),
]
addon_cases = [
    ('node-binding', 'node_binding_runner.tscn', 23),
    ('sequence-speed', 'sequence_speed_runner.gd', 9),
    ('sequence-ctors', 'sequence_ctors_runner.gd', 18),
    ('ellipse', 'ellipsis_regressions_runner.gd', 11),
    ('sequence-cycles', 'sequence_cycles_runner.tscn', 6),
    ('ownership', 'ownership_runner.tscn', 15),
    ('delayed-call', 'delayed_call_validity_runner.tscn', 9),
]
runtime = []
libraries = []
for profile in ['debug', 'release']:
    for kind, crate in [('native', 'spire_tween_rust'), ('gdscript', 'spire_tween')]:
        fixture = ROOT / (kind + '-project')
        source = REPO / 'target' / profile / ('lib' + crate + '.so')
        installed = fixture / 'addons/spire_tween/lib/x86_64-unknown-linux-gnu' / profile / source.name
        installed.parent.mkdir(parents=True, exist_ok=True)
        if installed.is_symlink():
            raise RuntimeError(f'Fixture must hold a regular file, found symlink: {installed}')
        shutil.copy2(source, installed)
        if sha(source) != sha(installed):
            raise RuntimeError('Installed fixture library does not match fresh build')
        manifest = fixture / 'addons/spire_tween/spire_tween.gdextension'
        manifest.write_text('[configuration]\nentry_symbol="gdext_rust_init"\ncompatibility_minimum="4.4.1"\nreloadable=true\n\n[libraries]\nlinux.x86_64="res://addons/spire_tween/lib/x86_64-unknown-linux-gnu/' + profile + '/' + source.name + '"\n')
        libraries.append({'kind': kind, 'profile': profile, 'source': str(source), 'installed': str(installed), 'sha256': sha(source)})
        cases = native_scenes if kind == 'native' else addon_cases
        for suite, entry, count in cases:
            cmd = [GODOT, '--headless', '--path', str(fixture), '--fixed-fps', '120']
            if entry.endswith('.gd'):
                cmd += ['--script']
            cmd += ['res://examples/tests/' + entry]
            result = run(kind + '-' + profile + '-' + suite, cmd, timeout=90)
            result.update(profile=profile, kind=kind, methods=count)
            runtime.append(result)
            (ROOT / 'runtime-results.json').write_text(json.dumps(runtime, indent=2) + '\n')

(ROOT / 'libraries.json').write_text(json.dumps(libraries, indent=2) + '\n')
before = json.loads((ROOT / 'before-source.json').read_text())
drift = [name for name, digest in before.items() if sha(REPO / name) != digest]
(ROOT / 'source-verification.json').write_text(json.dumps({'changed_during_verification': drift}, indent=2) + '\n')
print(json.dumps({'VERIFICATION_COMPLETE': True, 'runtime_processes': len(runtime), 'failed_processes': sum(r['exit_code'] != 0 for r in runtime), 'source_drift': drift}), flush=True)
