trait StringMetricT: Sync + Send {
    fn set(&self, value: String);
}

pub struct StringMetric(Box<dyn StringMetricT>);

#[derive(Debug)]
pub struct StringMetricImpl;
#[derive(Debug)]
pub struct StringMetricIpc;

/// For ease of implementation we rely on externally implemented functionality to tell us if we're
/// using IPC or not.
extern "Rust" {
    /// Return `true` if this is running in the parent process, `false` if it is a child process.
    ///
    /// Child processes will use IPC to communicate back with the parent process.
    fn is_parent_proc() -> bool;

    /// Send a byte buffer using IPC.
    ///
    /// Glean encodes its commands into an opaque byte buffer, the receiving side is responsible to
    /// decode it and turn that into a command again.
    fn ipc_send_buffer(buffer: Vec<u8>);
}

impl StringMetric {
    pub fn new() -> Self {
        // At metric instantiation we determine if this is a parent-process metric or not.
        if unsafe {is_parent_proc() } {
            StringMetric(Box::new(StringMetricImpl::new()))
        } else {
            StringMetric(Box::new(StringMetricIpc::new()))
        }
    }

    /// Set a string metric to a value.
    ///
    /// This will "set" the metric if in the parent process
    /// or forward the value over IPC if it is in the child process.
    ///
    /// **NOTE**:
    ///
    /// This still means it has a branch, but I assume a CPU will be very good at predicting the
    /// right branch here, as we don't change `self`'s state after initialization.
    pub fn set<S: Into<String>>(&self, value: S) {
        let value = value.into();
        self.0.set(value)
    }
}

impl StringMetricImpl {
    fn new() -> Self {
        Self
    }
}

/// Main process implementation of a string metric.
impl StringMetricT for StringMetricImpl {

    fn set(&self, value: String) {
        println!("Setting string metric to: {:?}", value);
    }
}


impl StringMetricIpc {
    fn new() -> Self {
        Self
    }
}

/// IPC implementation of a string metric.
impl StringMetricT for StringMetricIpc {
    fn set(&self, value: String) {
        println!("IPC-sending string metric with value: {:?}", value);
        unsafe {
            let buffer = bincode::serialize(&value).unwrap();
            ipc_send_buffer(buffer);
        }
    }
}
