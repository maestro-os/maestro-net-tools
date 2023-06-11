//! TODO doc

use std::io;
use std::mem::size_of;
use std::net::SocketAddr;
use std::os::fd::AsRawFd;

// TODO allow setting TTL for sent packets

/// A socket working with the ICMP protocol.
pub struct IcmpSocket {
	/// The socket's file descriptor.
	sock: i32,
}

impl IcmpSocket {
	/// Creates a new raw socket.
	///
	/// If the process doesn't have the permission to open a raw socket, the function returns an
	/// error.
	pub fn new() -> io::Result<Self> {
		// TODO add support for IPv6

		// Create socket
		let res = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_ICMP) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		// Enable TTL retrieve
		let sock = res;
		let res = unsafe {
			libc::setsockopt(
				sock,
				libc::SOL_IP,
				libc::IP_RECVTTL,
				&1u32 as *const _ as _,
				size_of::<u32>() as _,
			)
		};
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(Self {
			sock,
		})
	}

	/// Receives a packet.
	///
	/// The function returns a tuple containing:
	/// - The length of the received message
	/// - The packet's TTL
	pub fn recvmsg(&self, buf: &mut [u8], addr: &SocketAddr) -> io::Result<(usize, u8)> {
		// TODO support IPv6

		let mut ctrl_buf: [u8; 1024] = [0; 1024];
		let mut msghdr = match addr {
			SocketAddr::V4(a) => libc::msghdr {
				msg_name: &libc::sockaddr_in {
					sin_family: libc::AF_INET as _,
					sin_port: 0,
					sin_addr: libc::in_addr {
						s_addr: u32::from_le_bytes(a.ip().octets()),
					},
					sin_zero: [0; 8],
				} as *const _ as _,
				msg_namelen: size_of::<libc::sockaddr_in>() as _,
				msg_iov: &mut libc::iovec {
					iov_base: buf.as_mut_ptr() as _,
					iov_len: buf.len(),
				},
				msg_iovlen: 1,
				msg_control: ctrl_buf.as_mut_ptr() as _,
				msg_controllen: ctrl_buf.len(),
				msg_flags: 0,
			},

			SocketAddr::V6(_a) => todo!(), // TODO
		};

		let res = unsafe { libc::recvmsg(self.sock, &mut msghdr, 0) };
		if res < 0 {
			return Err(io::Error::last_os_error());
		}

		// Get TTL from control
		let _chdr = unsafe { ctrl_buf.as_ptr() as *const libc::cmsghdr };
		// TODO make safer by check the size of received data and the level/type of the hdr
		let ttl = ctrl_buf[size_of::<libc::cmsghdr>()];

		Ok((res as _, ttl))
	}

	/// Sends a packet.
	pub fn sendto(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
		let addr = match addr {
			SocketAddr::V4(a) => libc::sockaddr_in {
				sin_family: libc::AF_INET as _,
				sin_addr: libc::in_addr {
					s_addr: u32::from_le_bytes(a.ip().octets()),
				},
				sin_port: 0,
				sin_zero: [0; 8],
			},

			SocketAddr::V6(_a) => todo!(), // TODO
		};

		let res = unsafe {
			libc::sendto(
				self.sock,
				buf.as_ptr() as *const _,
				buf.len(),
				0,
				&addr as *const _ as *const _,
				size_of::<libc::sockaddr_in>() as _,
			)
		};

		if res >= 0 {
			Ok(res as _)
		} else {
			Err(io::Error::last_os_error())
		}
	}
}

impl AsRawFd for IcmpSocket {
	fn as_raw_fd(&self) -> i32 {
		self.sock
	}
}
