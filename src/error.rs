use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("PtraceAttachError")]
    PtraceAttachError,
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
