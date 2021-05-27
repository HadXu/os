use alloc::collections::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PIDS: AtomicUsize = AtomicUsize::new(0);
    pub static ref PROCESS: Mutex<Process> = Mutex::new(Process::new("/", None)); // TODO
}


pub struct Process {
    id: usize,
    env: BTreeMap<String, String>,
    dir: String,
    user: Option<String>,
}

impl Process {
    pub fn new(dir: &str, user: Option<&str>) -> Self {
        let id = PIDS.fetch_add(1, Ordering::SeqCst);
        let env = BTreeMap::new();
        let dir = dir.to_string();
        let user = user.map(String::from);
        Self { id, env, dir, user }
    }
}



