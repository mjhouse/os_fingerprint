# os_fingerprint

This tool uses semi-unique aspects of the host system to generate a u64 value that can be used as an identifier for the operating system and host machine. The values used to create the fingerprint are:

* OS type: 'Windows'/'Linux' etc 
* OS Release: Version information
* CPU count: Count of logical CPUs
* Total memory: Amount of RAM
* Total swap: The swap memory available
* Disk space: The total disk space on the system

This information isn't likely to change very often, so it can be used to identify the system, although it might have issues with two very similar machines. Still a work in progress.
