#[derive(Debug)]
pub enum StringMetric {
    Parent(StringMetricImpl),
    Child(StringMetricIpc),
}

#[derive(Debug)]
pub struct StringMetricImpl;
#[derive(Debug)]
pub struct StringMetricIpc;

extern "C" {
    fn is_parent_proc() -> bool;
    fn ipc_send_buffer(buffer: Vec<u8>);
}

impl StringMetric {
    pub fn new() -> Self {
        if unsafe {is_parent_proc() } {
            StringMetric::Parent(StringMetricImpl::new())
        } else {
            StringMetric::Child(StringMetricIpc::new())
        }
    }

    pub fn set<S: Into<String>>(&self, value: S) {
        let value = value.into();
        match self {
            StringMetric::Parent(p) => p.set(value),
            StringMetric::Child(c) => c.set(value),
        }
    }
}


impl StringMetricImpl {
    pub fn new() -> Self {
        Self
    }

    pub fn set(&self, value: String) {
        println!("Setting string metric to: {:?}", value);
    }
}

impl StringMetricIpc {
    pub fn new() -> Self {
        Self
    }

    pub fn set(&self, value: String) {
        println!("IPC-sending string metric with value: {:?}", value);
        unsafe {
            let buffer = bincode::serialize(&value).unwrap();
            ipc_send_buffer(buffer);
        }
    }
}
