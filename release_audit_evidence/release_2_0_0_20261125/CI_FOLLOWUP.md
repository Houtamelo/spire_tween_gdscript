# Hosted validation follow-up

[Run 45730478009](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/45730478009)
tested commit `a3f522115055ae15f12a89eebee27eb723924e01`.
All native builds passed. Windows passed the complete validation on Godot 4.4,
4.5.2, 4.6.3 and 4.7.2, and both Linux and Windows showcase exports passed.
The native timing repair also passed on macOS; its jobs reached export checks.
The complete desktop delivery was blocked by the following configuration issues.

## Universal macOS export settings

Godot rejects universal/ARM64 exports when ASTC texture importing is disabled,
even for these OpenGL projects. Both the generated smoke project and showcase
now enable S3TC/BPTC and ETC2/ASTC importing. Godot's accompanying message about
its default template path was incidental: the custom template existed; disabling
ASTC was the failing condition.

A new real-export regression uses the showcase's production project settings,
exports an actual universal macOS application with the supplied `macos.zip`, then
disables ASTC and requires export to fail with that specific diagnostic. It runs
in the showcase jobs. Both branches passed locally on Godot 4.4.

## Software Vulkan driver for editor probes

The Linux editor rendered through Mesa OpenGL, then probed Vulkan and reported
`VK_KHR_surface` missing. Strict log checks correctly rejected the run. The
workflow now installs `mesa-vulkan-drivers` and selects Lavapipe's ICD, alongside
the existing Xvfb/OpenGL dependencies. Error scanning remains strict.

Forcing a nonexistent Vulkan ICD reproduced this failure locally on Godot 4.4
and 4.7.2. Selecting Lavapipe passed all 33 runtime and 33 editor template cases
on both versions. CI now requires the missing-driver run to fail with the Vulkan
surface diagnostic before running its regular positive checks.

## Preserve framework metadata through export

A full cross-export using the actual macOS library from the hosted build exposed
a third issue. Godot rejected the framework plist's incomplete bundle metadata
and regenerated it, replacing version `2.0.0` with `1.0.0` and deployment target
`11.0` with `10.12`. The missing fields were `CFBundleInfoDictionaryVersion`,
`CFBundleName`, and `CFBundleSupportedPlatforms`.

The framework generator now supplies those fields. Both smoke and showcase
exports require the exported framework plist to match its staged input exactly.
A regression checks a real exported app, changes its framework version, requires
the verifier to reject it, and restores the original bytes before returning.

The full showcase cross-export succeeded without warnings after the metadata
correction; the preservation/failure regression passed against that actual
export. This was a Linux-hosted export of macOS binaries, not macOS execution.
Native macOS execution and the full delivery still require a successful rerun.

The `followup/` directory contains the relevant reproductions and passing logs.

## Inspector input and independent clocks

[The next hosted run](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/45738578012)
passed all three native builds and all three standalone showcase exports, including
the macOS metadata and real-export regressions. It exposed two more assumptions
in the validation harness.

Godot 4.5.2 received the synthetic mouse events but reported no hovered control
in an editor maximized on a window-manager-free Xvfb display. This reproduced
locally. The preview harness now gives the editor a bounded window on a larger
private display and moves the actual virtual cursor out of and back into that
window before clicking. It checks hover as well as visible/enabled buttons,
animation movement, and synchronous restoration. All 230 editor checks and two
runtime checks passed on Godot 4.4, 4.5.2, 4.6.3, and 4.7.2.

Intel macOS 4.5.2/4.6.3 failed the unscaled-time test's assumption that one wall
second always advances the half-speed simulation clock by approximately 0.5s.
The 4.6.3 run measured 0.6s. The replacement checks the clock-selection contract:
at zero engine time scale, an ordinary tween stays unchanged while an unscaled
tween completes; after resuming at half speed, the control consumes one scaled
second with at most the actual final frame's overshoot. Global time scale is
restored even if the test task unwinds. No production timing behavior changed.

The revised real-engine case passed locally on all four supported versions,
including 4.7.2 at one frame per second. Logs in `followup/` record these checks;
the complete hosted matrix and final artifact delivery still require a rerun.
