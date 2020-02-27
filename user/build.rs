use std::{
    env,
    path::Path,
};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    glean::build::build_metrics(&path, "used", "os").unwrap();
}
