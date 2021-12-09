# simple bpf program in pure rust 
## prerequisites

* rust nightly toolchain with `rust-src` component
* [bpf-linker](https://github.com/aya-rs/bpf-linker.git) installed

## build
`cargo build --release`

### load it to lo
`sudo ip link set dev lo xdp obj target/bpfel-unknown-none/release/rust-bpf-example sec xdp`
### show bpf program print
`sudo cat /sys/kernel/debug/tracing/trace_pipe`

### unload
`sudo ip link set dev lo xdp off`

------

if u want to write bpf program in rust, there is a great library: [aya](https://github.com/aya-rs/aya).
