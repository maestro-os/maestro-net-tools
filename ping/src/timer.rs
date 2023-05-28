//! A timer allows to wake the current process at a specified interval.

use std::ffi::c_void;
use std::io;
use std::ptr::null_mut;
use std::time::Duration;

/// A timer handled by the kernel.
///
/// The timer sends a `SIGALRM` signal to the current process at the given interval.
pub struct Timer {
	/// The timer's ID.
	id: *mut c_void,
}

impl Timer {
	/// Creates a new timer.
	pub fn new(interval: Duration) -> io::Result<Self> {
		// TODO disable `SIGALRM` during setup?

		// Create timer
		let mut id: *mut c_void = null_mut::<_>();
		let res = unsafe { libc::timer_create(libc::CLOCK_REALTIME, null_mut::<_>(), &mut id) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		// Set the timer's interval
		let nanos = interval.as_nanos();
		let its = libc::itimerspec {
			it_interval: libc::timespec {
				tv_sec: (nanos / 1000000000) as _,
				tv_nsec: (nanos % 1000000000) as _,
			},
			it_value: libc::timespec {
				tv_sec: (nanos / 1000000000) as _,
				tv_nsec: (nanos % 1000000000) as _,
			},
		};
		let res = unsafe { libc::timer_settime(id, 0, &its, null_mut::<_>()) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(Self {
			id,
		})
	}
}

impl Drop for Timer {
	fn drop(&mut self) {
		unsafe {
			libc::timer_delete(self.id);
		}
	}
}
