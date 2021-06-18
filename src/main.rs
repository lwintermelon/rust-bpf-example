#![no_std]
#![no_main]

use rust_bpf_example::{
    bpf_helpers::bpf_printk,
    vmlinux::{xdp_action, xdp_md, XDP_PASS},
};

#[no_mangle]
#[link_section = "license"]
static LICENSE: [u8; 4] = *b"GPL\x00";

#[panic_handler]
fn panic_impl(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = "xdp"]
unsafe fn xdp_pass(ctx: *mut xdp_md) -> xdp_action {
    let data = (*ctx).data;
    let data_end = (*ctx).data_end;
    let packet_size = data_end - data;

    bpf_printk(*b"packet size: %d\0", packet_size);
    return XDP_PASS;
}
