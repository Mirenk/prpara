/*
pub fn mmap (
    proc: Proc,
    addr: Option<NonZeroUsize>,
    lengh: NonZeroUsize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: RawFd,
    offset: off_t
) -> () {
    let mut regs = proc.regs;

    regs.rax = SYS_mmap as u64;
    regs.rdx = 0;
    regs.rsi = size;
}
*/
