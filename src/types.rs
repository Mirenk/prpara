use thiserror::Error;

pub type Address = u64;

pub type Pid = i32;

#[derive(Error, Debug)]
pub enum Error {
    #[error("PidError")]
    PidError,
    #[error("HashError")]
    HashError,
    #[error("MapError")]
    MapError,
    #[error("ElfLoadError")]
    ElfLoadError,
    #[error("PtraceAttachError")]
    PtraceAttachError,
    #[error("PtraceContinueError")]
    PtraceContinueError,
    #[error("PtraceSetOptionError")]
    PtraceSetOptionError,
    #[error("PtraceGetRegsError")]
    PtraceGetRegsError,
    #[error("PtraceSetRegsError")]
    PtraceSetRegsError,
    #[error("PtraceReadError")]
    PtraceReadError,
    #[error("PtraceWriteError")]
    PtraceWriteError,
    #[error("PtraceDetachError")]
    PtraceDetachError,
    #[error("WaitPidError")]
    WaitPidError,
    #[error("RunSyscallError")]
    RunSyscallError,
}
