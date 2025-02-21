use crate::{ExpectedSyscall, SyscallLogEntry};
use std::cell::Cell;

// TODO: Add Allow.
// TODO: Add Command.
// TODO: Add Exit.
// TODO: Add Memop.
// TODO: Add Subscribe.
mod raw_syscalls_impl;
mod thread_local;
// TODO: Add Yield.

/// A fake implementation of the Tock kernel. Provides
/// `libtock_platform::Syscalls` by implementing
/// `libtock_platform::RawSyscalls`. Allows `fake::Driver`s to be attached, and
/// routes system calls to the correct fake driver.
///
/// Note that there can only be one `Kernel` instance per thread, as a
/// thread-local variable is used to implement `libtock_platform::RawSyscalls`.
/// As such, test code is given a `Rc<Kernel>` rather than a `Kernel` instance
/// directly. Because `Rc` is a shared reference, Kernel extensively uses
/// internal mutability.
// TODO: Define the `fake::Driver` trait and add support for fake drivers in
// Kernel.
pub struct Kernel {
    expected_syscalls: Cell<std::collections::VecDeque<ExpectedSyscall>>,
    name: &'static str,
    syscall_log: Cell<Vec<SyscallLogEntry>>,
}

impl Kernel {
    /// Creates a `Kernel` for this thread and returns a reference to it. This
    /// instance should be dropped at the end of the test, before this thread
    /// creates another `Kernel`. `name` should be a string identifying the test
    /// case, and is used to provide better diagnostics.
    pub fn new(name: &'static str) -> std::rc::Rc<Kernel> {
        let rc = std::rc::Rc::new(Kernel {
            expected_syscalls: Default::default(),
            name,
            syscall_log: Default::default(),
        });
        thread_local::set_kernel(&rc);
        rc
    }

    /// Adds an ExpectedSyscall to the expected syscall queue.
    ///
    /// # What is the expected syscall queue?
    ///
    /// In addition to routing system calls to drivers, `Kernel` supports
    /// injecting artificial system call responses. The primary use case for
    /// this feature is to simulate errors without having to implement error
    /// simulation in each `fake::Driver`.
    ///
    /// The expected syscall queue is a FIFO queue containing anticipated
    /// upcoming system calls. It starts empty, and as long as it is empty, the
    /// expected syscall functionality does nothing. When the queue is nonempty
    /// and a system call is made, the system call is compared with the next
    /// queue entry. If the system call matches, then the action defined by the
    /// expected syscall is taken. If the call does not match, the call panics
    /// (to make the unit test fail).
    pub fn add_expected_syscall(&self, expected_syscall: ExpectedSyscall) {
        let mut queue = self.expected_syscalls.take();
        queue.push_back(expected_syscall);
        self.expected_syscalls.set(queue);
    }

    /// Returns the system call log and empties it.
    pub fn take_syscall_log(&self) -> Vec<SyscallLogEntry> {
        self.syscall_log.take()
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        thread_local::clear_kernel();
    }
}

// -----------------------------------------------------------------------------
// Crate implementation details below.
// -----------------------------------------------------------------------------

impl Kernel {
    // Appends a log entry to the system call queue.
    #[allow(unused)] // TODO: Remove when a system call is implemented.
    fn log_syscall(&self, syscall: SyscallLogEntry) {
        let mut log = self.syscall_log.take();
        log.push(syscall);
        self.syscall_log.set(log);
    }

    // Retrieves the first syscall in the expected syscalls queue, removing it
    // from the queue. Returns None if the queue was empty.
    #[allow(unused)] // TODO: Remove when a system call is implemented.
    fn pop_expected_syscall(&self) -> Option<ExpectedSyscall> {
        let mut queue = self.expected_syscalls.take();
        let expected_syscall = queue.pop_front();
        self.expected_syscalls.set(queue);
        expected_syscall
    }

    // Panics, indicating that this Kernel was leaked. It is unlikely that this
    // panic will cause the correct test case to fail, but if this Kernel is
    // well-named the panic message should indicate where the leak occurred.
    fn report_leaked(&self) -> ! {
        panic!(
            "The fake::Kernel with name '{}' was never cleaned up; \
                perhaps a Rc<Kernel> was leaked?",
            self.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verifies the name propagates correctly into the report_leaked() error
    // message.
    #[test]
    fn name_to_report_leaked() {
        let result = std::panic::catch_unwind(|| {
            Kernel::new("name_to_report_leaked").report_leaked();
        });
        let panic_arg = result.expect_err("Kernel::report_leaked did not panic");
        let message = panic_arg
            .downcast_ref::<String>()
            .expect("Wrong panic payload type");
        assert!(message.contains("name_to_report_leaked"));
    }

    // TODO: We cannot currently test the expected syscall queue or the syscall
    // log, because ExpectedSyscall and SyscallLogEntry are currently
    // uninhabited types. When we implement a system call, we should add tests
    // for that functionality as well.
}
