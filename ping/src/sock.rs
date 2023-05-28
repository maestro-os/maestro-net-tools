//! TODO doc

use std::io;
use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use std::net::SocketAddr;
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
		let res = unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(Self {
			sock: res,
		})
	}

	/// TODO doc
	pub fn recvfrom(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
		// TODO
		todo!()
	}

	/// TODO doc
	pub fn sendto(&self, buf: &[u8], addr: SocketAddr) -> io::Result<usize> {
		let res = match addr {
			SocketAddr::V4(a) => {
				let addr = libc::sockaddr_in {
					sin_addr: libc::in_addr {
						s_addr: u32::from_ne_bytes(a.ip().octets()),
					},
					sin_family: libc::AF_INET as _,
					sin_port: addr.port(),
					sin_zero: [0; 8],
				};

				unsafe {
					libc::sendto(
						self.sock,
						buf.as_ptr() as *const _,
						buf.len(),
						0,
						&addr as *const _ as *const _,
						size_of::<libc::sockaddr_in>() as _,
					)
				}
			}

			SocketAddr::V6(a) => {
				// TODO
				todo!()
			}
		};

		if res >= 0 {
			Ok(res as _)
		} else {
			Err(io::Error::last_os_error())
		}
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
