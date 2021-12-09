use crate::vmlinux::BPF_FUNC_trace_printk;
use core::mem;

#[inline(always)]
pub fn bpf_printk<T, const LEN: usize>(fmt: [u8; LEN], value: T) -> i64 {
    let bpf_trace_printk: unsafe extern "C" fn(
        fmt: *const cty::c_char,
        fmt_size: cty::uint32_t,
        ...
    ) -> cty::c_long = unsafe { mem::transmute(BPF_FUNC_trace_printk as usize) };
    unsafe { bpf_trace_printk(fmt.as_ptr() as *const i8, LEN as cty::uint32_t, value) }
}
