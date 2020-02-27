#[derive(Debug)]
pub struct StringMetric;

impl StringMetric {
    pub fn new() -> Self {
        Self
    }

    pub fn set<S: Into<String>>(&self, value: S) {
        println!("Setting string metric to: {:?}", value.into());
    }
}

