extern crate sys_info;
extern crate chrono;
extern crate chrono_tz;

use std::collections::hash_map::{DefaultHasher};
use std::hash::{Hash, Hasher};

fn simple_fp() -> u64 {
    let o = sys_info::os_type().unwrap_or_default();
    let v = sys_info::os_release().unwrap_or_default();
    let c = sys_info::cpu_num().unwrap_or_default();

    let (t,s) = match sys_info::mem_info() {
        Ok(v) => (v.total,v.swap_total),
        _ => (0,0)
    };

    let d = match sys_info::disk_info() {
        Ok(v) => v.total,
        _ => 0
    };

    let mut hasher = &mut DefaultHasher::new();
    o.hash(&mut hasher); // OS (e.g. 'Windows' or 'Linux')
    v.hash(&mut hasher); // release (e.g. '6.10')
    c.hash(&mut hasher); // logical cpus (e.g. '8')
    t.hash(&mut hasher); // total memory (e.g '33402824' in kb?)
    s.hash(&mut hasher); // total swap mem (e.g. '38383560' in kb?)
    d.hash(&mut hasher); // total disk space
    hasher.finish()
}

fn main() {
    dbg!(simple_fp());
}
