use std::ffi::{c_int, c_void};

use nix::libc::{
    off_t, size_t, SYS_mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, PT_NULL,
};
use nix::sys::ptrace::{self, AddressType, Options};
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::{libc::user_regs_struct, sys::wait::WaitStatus};

use crate::types::Error;
use crate::{
    types::{Address, Pid},
    Result,
};

pub struct Proc {
    pid: nix::unistd::Pid,
}

impl Proc {
    // memory allocation at proccess
    pub fn mem_alloc(&mut self, size: usize) -> Result<Address> {
        let orig_regs = ptrace::getregs(self.pid).map_err(|_| Error::PtraceGetRegsError)?;

        // mmap(0, size, PROT_EXEC|PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
        let mmap_regs = prepare_mmap(
            &orig_regs,
            PT_NULL as *mut c_void,
            size as size_t,
            PROT_EXEC | PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );
        unsafe { self.run_syscall(mmap_regs, None) }
    }

    // write to process
    pub fn write_buf(&mut self, addr: Address, data: Vec<u8>) -> Result<()> {
        let word_size = 8;
        let align_size = word_size - 1;

        let align_addr = addr & !align_size;
        let align_head_size = addr - align_addr;
        let len = data.len() as u64;
        let len = align_head_size + len;
        let count: usize = (len + align_size / word_size).try_into().unwrap();

        let align_head_size = align_head_size as usize;
        let mut write_buf: Vec<u8> = ptrace::read(self.pid, align_addr as AddressType)
            .unwrap()
            .to_le_bytes()
            .to_vec()[0..align_head_size]
            .to_vec();

        let align_tail_size = (word_size - (len % 8)) as usize;
        let mut tail_buf: Vec<u8> =
            ptrace::read(self.pid, (align_addr + (len / word_size)) as AddressType)
                .unwrap()
                .to_le_bytes()
                .to_vec()[align_tail_size..]
                .to_vec();
        write_buf.extend_from_slice(&data);
        write_buf.append(&mut tail_buf);

        for i in (0..count).step_by(word_size as usize) {
            let addr = (align_addr + i as u64) as AddressType;
            let data = &write_buf[i..(i + word_size as usize)];
            let data = u64::from_le_bytes(data.to_vec().try_into().unwrap()) as *mut c_void;

            unsafe {
                let _ = ptrace::write(self.pid, addr, data);
            };
        }
        Ok(())
    }

    // run syscall at process
    pub unsafe fn run_syscall(
        &self,
        regs: user_regs_struct,
        stack: Option<Vec<u8>>,
    ) -> Result<u64> {
        // backup original regs
        let orig_regs = ptrace::getregs(self.pid).map_err(|_| Error::PtraceGetRegsError)?;

        // get program counter address
        let rip = regs.rip as *mut c_void;

        // backup original machine code
        let orig_code =
            ptrace::read(self.pid, rip).map_err(|_| Error::PtraceReadError)? as *mut c_void;

        // prepare run syscall
        let syscall_code = 0xcc050f as *mut c_void; // syscall machine code. (syscall; int3;)

        // syscall run as process
        ptrace::setregs(self.pid, regs).map_err(|_| Error::PtraceSetRegsError)?;
        ptrace::write(self.pid, rip, syscall_code).map_err(|_| Error::PtraceWriteError)?; // unsafe
        ptrace::cont(self.pid, None).map_err(|_| Error::PtraceContinueError)?;

        // get result and restore
        if let Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) = waitpid(self.pid, None) {
            // get return value from rax
            let regs = ptrace::getregs(self.pid).map_err(|_| Error::PtraceGetRegsError)?;
            let ret = regs.rax;

            // restore regs and machine code
            ptrace::setregs(self.pid, orig_regs).map_err(|_| Error::PtraceSetRegsError)?;
            ptrace::write(self.pid, rip, orig_code).map_err(|_| Error::PtraceWriteError)?; // unsafe

            Ok(ret)
        } else {
            Err(Error::WaitPidError)
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
}

// make proc instance
pub fn new(pid: Pid) -> Result<Proc> {
    let pid = nix::unistd::Pid::from_raw(pid);

    // attach to process
    ptrace::attach(pid).map_err(|_| Error::PtraceAttachError)?;
    if let Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) = waitpid(pid, None) {
        ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD)
            .map_err(|_| Error::PtraceSetOptionError)?;

        let obj = Proc { pid };

        Ok(obj)
    } else {
        Err(Error::WaitPidError)
    }
}

// example for run_syscall()
fn prepare_mmap(
    orig_regs: &user_regs_struct,
    addr: *mut c_void,
    len: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
) -> user_regs_struct {
    let mut regs = orig_regs.clone();

    // set args to regs
    regs.rax = SYS_mmap as u64;
    regs.rdi = addr as u64;
    regs.rsi = len as u64;
    regs.rdx = prot as u64;
    regs.rcx = flags as u64;
    regs.r10 = flags as u64;
    regs.r8 = fd as u64;
    regs.r9 = offset as u64;

    regs
}
