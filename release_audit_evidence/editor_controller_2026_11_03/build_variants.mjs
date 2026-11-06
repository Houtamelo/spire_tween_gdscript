import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(evidence, '../..');
const work = path.join(repo, 'target/editor_controller_probe_2026_11_03');
const native = path.join(work, 'native');
fs.mkdirSync(native, { recursive: true });
fs.cpSync(path.join(repo, 'spire_tween'), path.join(native, 'spire_tween'), { recursive: true });
fs.copyFileSync(path.join(repo, 'Cargo.lock'), path.join(native, 'Cargo.lock'));
fs.writeFileSync(path.join(native, 'Cargo.toml'), `[workspace]\nmembers = ["spire_tween"]\nresolver = "3"\n\n[profile.dev]\ndebug-assertions = true\nopt-level = 3\nlto = false\ndebug = true\n`);

const original = fs.readFileSync(path.join(repo, 'spire_tween/src/global/mod.rs'), 'utf8');
function replaceOnce(source, from, to) {
  if (!source.includes(from) || source.indexOf(from) !== source.lastIndexOf(from)) {
    throw new Error(`Expected exactly one source marker: ${from}`);
  }
  return source.replace(from, to);
}
const baseline = original.replace('#[class(init, tool, base = Node, internal)]', '#[class(init, base = Node, internal)]');
let instrumented = replaceOnce(baseline, 'pub(crate) struct TweensController {\n    base: Base<Node>,', 'pub(crate) struct TweensController {\n    base: Base<Node>,\n    #[var]\n    probe_process_ticks: i64,\n    #[var]\n    probe_physics_ticks: i64,');
instrumented = replaceOnce(instrumented, '    fn process(&mut self, delta_time: f64) {', '    fn process(&mut self, delta_time: f64) {\n        self.probe_process_ticks += 1;');
instrumented = replaceOnce(instrumented, '    fn physics_process(&mut self, delta_time: f64) {', '    fn physics_process(&mut self, delta_time: f64) {\n        self.probe_physics_ticks += 1;');
const hash = buffer => createHash('sha256').update(buffer).digest('hex');
const builds = [];
for (const variant of ['no_tool', 'tool']) {
  const source = variant === 'tool'
    ? replaceOnce(instrumented, '#[class(init, base = Node, internal)]', '#[class(init, tool, base = Node, internal)]')
    : instrumented;
  fs.writeFileSync(path.join(native, 'spire_tween/src/global/mod.rs'), source);
  const args = ['build', '--offline', '-p', 'spire_tween', '--features', 'include_gdext_lib,include_gdscript_bridge,api-4-4', '-j', '2'];
  const result = spawnSync('cargo', args, {
    cwd: native,
    env: { ...process.env, CI: 'true', CARGO_TARGET_DIR: path.join(work, 'target') },
    encoding: 'utf8',
    maxBuffer: 16 * 1024 * 1024,
  });
  fs.writeFileSync(path.join(evidence, `${variant}_build.log`), (result.stdout ?? '') + (result.stderr ?? ''));
  if (result.status !== 0) throw new Error(`${variant} build failed: ${result.stderr}`);
  const artifact = path.join(work, 'artifacts', variant, 'libspire_tween.so');
  fs.mkdirSync(path.dirname(artifact), { recursive: true });
  fs.copyFileSync(path.join(work, 'target/debug/libspire_tween.so'), artifact, fs.constants.COPYFILE_FICLONE);
  builds.push({ variant, command: ['cargo', ...args], source_sha256: hash(source), artifact, artifact_sha256: hash(fs.readFileSync(artifact)) });
  console.log(`Built ${variant}: ${artifact}`);
}
fs.writeFileSync(path.join(evidence, 'builds.json'), JSON.stringify({ original_source_sha256: hash(original), builds }, null, 2) + '\n');
