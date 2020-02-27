use std::{
    env,
    path::Path,
};

fn main() {
    // The user can specify a path to the generated code file.
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");

    // Usually this would pass paths to metrics.yaml and pings.yaml files.
    // For demonstration purposes we use a [category].[name] pair only
    glean::build::build_metrics(&path, "used", "os").unwrap();
}
