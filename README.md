# Glean Codegen example

Small example project to demonstrate what the code generation would look like for the Glean Rust API.

## `user` crate

This is an example user of Glean.
This would include `metrics.yaml` and `pings.yaml` files.

It uses a [`build.rs`](user/build.rs) file to call the Glean parser & code generator.
It then includes the generated code and can use it (see [`user/src/metrics.rs`](user/src/metrics.rs)).

## `glean` crate

A simplification of what will become the actual `glean` crate.

It provides a wrapper around the Glean parser in [`glean/src/build.rs`](glean/src/build.rs).
That part might actually become its own crate to reduce compile times when build target != host target.
