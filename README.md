# bpf example program in pure rust 

### build
`cargo build -Z build-std=core,alloc --release`

### load it to lo
`sudo ip link set dev lo xdp obj target/bpfel-unknown-none/release/rust-bpf-example sec xdp`
### show bpf program print
`sudo cat /sys/kernel/debug/tracing/trace_pipe`

### unload
`sudo ip link set dev lo xdp off`
