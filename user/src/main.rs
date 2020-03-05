use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Serialize, Deserialize};
use procspawn::{self, spawn};
use ipc_channel::ipc::{self, IpcSender};

mod metrics;

static mut IS_PARENT_PROCESS: AtomicBool = AtomicBool::new(true);
static mut IPC_SENDER: Option<IpcSender<Command>> = None;

#[no_mangle]
fn is_parent_proc() -> bool {
    // safe, it's an atomic
    unsafe {
        IS_PARENT_PROCESS.load(Ordering::SeqCst)
    }
}

#[no_mangle]
fn ipc_send_buffer(buffer: Vec<u8>) {
    unsafe {
        match IPC_SENDER {
            Some(ref sender) => sender.send(Command::Bytes(buffer)).unwrap(),
            None => {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Bytes(Vec<u8>)
}

fn main() {
    procspawn::init();

    let (tx, rx) = ipc::channel::<Command>().unwrap();

    let child_handle = spawn(tx, |tx| {
        unsafe {
            IS_PARENT_PROCESS.store(false, Ordering::SeqCst);
            IPC_SENDER = Some(tx);
        }

        println!("Started child process.");
        metrics::used::os.set("Windows");
    });

    metrics::used::os.set("Windows");

    let Command::Bytes(buffer) = rx.recv().unwrap();
    let s: String = bincode::deserialize(&buffer).unwrap();
    println!("received string: {:?}", s);

    child_handle.join().unwrap();
}
