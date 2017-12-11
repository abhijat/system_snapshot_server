# system_snapshot_server

A service to expose the information found on linux in the proc filesystem over http as structured data.

## How to run:
For now there is not much configuration, so simply execute:

    cargo run
    
This will start a web server on port 3000. 
Two URLs are available right now:

/processes -> shows running processes on the machine

/cpu -> shows very basic information about the CPU

### todo -
* add memory related information
* add filesystem related information
* add configuration options
* add a facility to mask user defined processes from appearing in the display
