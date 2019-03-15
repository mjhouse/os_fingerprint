extern crate sys_info;
extern crate chrono;
extern crate chrono_tz;

use std::collections::hash_map::{DefaultHasher};
use std::hash::{Hash, Hasher};
use std::fs;
use std::io::{BufRead, BufReader};

/*  This is a utility function for two of the fingerprinting
    methods below. It hashes a file.
*/
fn hash_file( file: &str ) -> u64 {
    match fs::File::open(file) {
        Ok(f) => {
            let mut hasher = &mut DefaultHasher::new();
            for line in BufReader::new(f).lines() {
                let content = line.unwrap_or_default();
                content.hash(&mut hasher);
            }
            hasher.finish()
        },
        _ => 0
    }

}

/*  Simple Fingerprint
    This fingerprinting function just uses some information about the
    system to generate a unique hash. It would probably be non-unique
    on very similar machines with the same os installed.
*/
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

/*  Device Fingerprint
    This function hashes /proc/modules to fingerprint the system,
    which would be different depending on the drivers installed.
    Might be a little more useful than the simple fingerprint.
*/
fn device_fp() -> u64 {
    hash_file("/proc/modules")
}

/*  User Fingerprint
    This would get us a hash that would be different depending on the
    user information on the system. This should be fairly unique to the
    system unless some other machine has the exact same users on it,
    created in the same order, and the same system-users.
*/
fn user_fp() -> u64 {
    hash_file("/etc/passwd")
}

fn main() {
    println!("\nSimple Fingerprint: \n\tuses basic system information\n\tResult: {:?}",simple_fp());
    println!("\nDevice Fingerprint: \n\thashes /proc/modules to identify system by loaded drivers\n\tResult: {:?}",device_fp());
    println!("\nUser Fingerprint: \n\thashes /etc/passwd to identify system by users\n\tResult: {:?}",user_fp());
}
