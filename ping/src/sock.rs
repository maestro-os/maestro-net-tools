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
		let sock = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_RAW, 0) };
		Ok(Self {
			sock,
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
		// TODO
		todo!()
	}
}

impl Write for RawSocket {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		// TODO
		todo!()
	}

	fn flush(&mut self) -> io::Result<()> {
		// TODO
		todo!()
	}
}
