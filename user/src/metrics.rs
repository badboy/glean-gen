//! All generated metrics.

// Include the generated file.
// That file already includes submodules.
// `include!` is similar to cpp's `#include`: it's just a copy'n'paste of the specified file.
// `OUT_DIR` is the output directory handled by cargo, e.g. `target/debug/build/user-2d9f8ddaf67fd7b5/out/`.
// It's the same here and in the `build.rs`
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
