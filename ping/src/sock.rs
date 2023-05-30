//! TODO doc

use std::io;
use std::mem::size_of;
use std::mem::MaybeUninit;
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
		let res = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_DGRAM, 2048) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(Self {
			sock: res,
		})
	}

	/// Receives a packet.
	pub fn recvfrom(&self, buf: &mut [u8]) -> io::Result<usize> {
		// TODO support IPv6
		let mut a: libc::sockaddr_ll = unsafe { MaybeUninit::uninit().assume_init() };
		let mut a_len = 0;

		let res = unsafe {
			libc::recvfrom(
				self.sock,
				buf.as_mut_ptr() as *mut _,
				buf.len(),
				0,
				&mut a as *mut _ as *mut _,
				&mut a_len,
			)
		};
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(res as _)
	}

	/// Sends a packet.
	pub fn sendto(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
		let addr = match addr {
			SocketAddr::V4(a) => libc::sockaddr_ll {
				sll_family: libc::AF_PACKET as _,
				sll_protocol: 2048u16.to_be() as _, // TODO add support for IPv6
				sll_ifindex: 3,                     // TODO
				sll_hatype: 0,
				sll_pkttype: 0,
				sll_halen: 6,
				sll_addr: [0x8c, 0xf8, 0xc5, 0x56, 0x7d, 0xa9, 0, 0], // TODO
			},

			SocketAddr::V6(_a) => todo!(),
		};

		let res = unsafe {
			libc::sendto(
				self.sock,
				buf.as_ptr() as *const _,
				buf.len(),
				0,
				&addr as *const _ as *const _,
				size_of::<libc::sockaddr_ll>() as _,
			)
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
