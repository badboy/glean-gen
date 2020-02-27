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

    writeln!(&mut file, "pub mod {} {{", category)?;
    writeln!(&mut file, "#[allow(non_upper_case_globals)]")?;
    writeln!(&mut file, "pub static {}: glean::once_cell::sync::Lazy<glean::metrics::StringMetric> = glean::once_cell::sync::Lazy::new(|| {{", name)?;
    writeln!(&mut file, "glean::metrics::StringMetric::new()")?;
    writeln!(&mut file, "}});")?;
    writeln!(&mut file, "}}")?;

    Ok(())
}
