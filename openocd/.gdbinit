# connect to openocd gdb server
# target extended-remote localhost:3333

# or, have gdb start openocd and communicate via pipes
target extended-remote | openocd -c "gdb_port pipe; log_output openocd.log" -f zcu111.cfg

# hardware-specific limits
set remote hardware-breakpoint-limit 6
set remote hardware-watchpoint-limit 4
# enable aarch64-specific debug messages
set debug aarch64
# for some reason 'load' never works on the initial connection
monitor reset halt
