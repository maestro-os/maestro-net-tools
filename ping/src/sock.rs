//! TODO doc

use std::io;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;

/// A raw socket, allowing to access the ICMP protocol.
pub struct RawSocket {
	/// The socket's file descriptor.
	sock: i32,
}

impl RawSocket {
	/// Creates a new raw socket.
	///
	/// If the process doesn't have the permission to open a raw socket, the function returns an
	/// error.
	pub fn new() -> io::Result<Self> {
		let res = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_RAW, 0) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(Self {
			sock: res,
		})
	}
}

impl AsRawFd for RawSocket {
	fn as_raw_fd(&self) -> i32 {
		self.sock
	}
}

impl Read for RawSocket {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let res = unsafe { libc::read(self.sock, buf.as_mut_ptr() as *mut _, buf.len()) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(res as _)
	}
}

impl Write for RawSocket {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let res = unsafe { libc::write(self.sock, buf.as_ptr() as *const _, buf.len()) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(res as _)
	}

	fn flush(&mut self) -> io::Result<()> {
		// TODO
		Ok(())
	}
}
