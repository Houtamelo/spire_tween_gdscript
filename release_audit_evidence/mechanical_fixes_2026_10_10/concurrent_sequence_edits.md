# Compilation interrupted by concurrent sequence changes

The Android build encountered an unrelated in-progress `RustFn(Box<dyn FnOnce>)`
signature in `spire_tween/src/tweens/sequence/data.rs`. After this changed to
`FnOnce()`, the retry encountered missing `Ctor` match arms at lines 149, 193,
281, 658, 674, 756 and 765 (line numbers at the time of that build).

The mechanical task does not own sequence implementation, and did not alter
these changes. Both attempts fail fast before publishing a release result. The
source needs finishing before rebuilding a consistent platform/profile matrix.
