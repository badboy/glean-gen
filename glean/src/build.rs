use std::{
    error::Error,
    fs::File,
    io::{
        prelude::*,
        BufWriter,
    },
    path::Path,

};

pub fn build_metrics(path: &Path, category: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = BufWriter::new(File::create(&path).unwrap());

    // This is where we would call into glean_parser to generate Rust code.
    // For demonstration purposes we just write some simple code into a file.
    writeln!(&mut file, "pub mod {} {{", category)?;
    writeln!(&mut file, "#[allow(non_upper_case_globals)]")?;
    writeln!(&mut file, "pub static {}: glean::once_cell::sync::Lazy<glean::metrics::StringMetric> = glean::once_cell::sync::Lazy::new(|| {{", name)?;
    writeln!(&mut file, "glean::metrics::StringMetric::new()")?;
    writeln!(&mut file, "}});")?;
    writeln!(&mut file, "}}")?;

    // Here we would print some additional lines for cargo.
    // This way cargo knows when it actually needs to rerun the build, otherwise it just reuses
    // previously generated code.
    // E.g.:
    //
    // cargo:rerun-if-changed=/path/to/metrics.yaml
    // cargo:rerun-if-changed=/path/to/pings.yaml

    Ok(())
}
