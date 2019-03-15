# os_fingerprint

This tool uses semi-unique aspects of the host system to generate a u64 value that can be used as an identifier for the operating system and host machine. There are three methods used to do this:

### Simple Fingerprint
#### uses basic system information
  
  * OS type: 'Linux' etc 
  * OS Release: Version information
  * CPU count: Count of logical CPUs
  * Total memory: Amount of RAM
  * Total swap: The swap memory available
  * Disk space: The total disk space on the system

### Device Fingerprint 
#### hashes /proc/modules to identify system by loaded drivers

### User Fingerprint 
#### hashes /etc/passwd to identify system by users
