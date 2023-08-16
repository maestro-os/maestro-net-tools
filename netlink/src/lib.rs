//! Netlink is a kernel feature allowing to inspect and control network interfaces.
//!
//! The netlink interface can be accessed from userspace through a socket.

pub mod route;
pub mod util;

use std::ffi::*;
use std::io;
use std::marker::PhantomData;
use std::mem::size_of;
use std::sync::atomic;
use std::sync::atomic::AtomicU32;

/// Netlink family: route
const NETLINK_ROUTE: c_int = 0;

/// Socket address for netlink sockets.
#[repr(C)]
#[derive(Default)]
struct sockaddr_nl {
	/// `AF_NETLINK`
	nl_family: libc::sa_family_t,
	/// Padding (zero)
	nl_pad: c_ushort,
	/// Port ID
	nl_pid: libc::pid_t,
	/// Multicast groups mask
	nl_groups: u32,
}

/// Netlink message header.
#[repr(C)]
struct nlmsghdr {
	/// Length of the message including header
	nlmsg_len: u32,
	/// Type of message content
	nlmsg_type: u16,
	/// Additional flags
	nlmsg_flags: u16,
	/// Sequence number
	nlmsg_seq: u32,
	/// Sender port ID
	nlmsg_pid: u32,
}

/// A netlink socket.
pub struct Netlink {
	/// The socket's file descriptor.
	fd: c_int,
	/// The next sequence number to be used.
	next_seq: AtomicU32,
}

impl Netlink {
	/// Creates a new instance.
	///
	/// `family` is the netlink group to communicate with.
	pub fn new(family: c_int) -> io::Result<Self> {
		let fd = unsafe { libc::socket(libc::AF_NETLINK, libc::SOCK_RAW, family) };
		if fd < 0 {
			return Err(io::Error::last_os_error());
		}

		Ok(Self {
			fd,
			next_seq: AtomicU32::new(0),
		})
	}

	/// Low-level interface to receive messages from the socket.
	///
	/// The function blocks untils a message is received from the socket, then writes it on the
	/// given buffer.
	///
	/// On success, the function returns the number of bytes read.
	pub unsafe fn recv_from(&self, buf: &mut [u8]) -> io::Result<usize> {
		let mut sockaddr = sockaddr_nl::default();

		loop {
			let res = unsafe {
				libc::recvfrom(
					self.fd,
					buf.as_mut_ptr() as _,
					buf.len(),
					0,
					&mut sockaddr as *mut _ as *mut _,
					size_of::<sockaddr_nl>() as _,
				)
			};
			if res < 0 {
				return Err(io::Error::last_os_error());
			}

			// ignore messages that do not come from the kernel
			if sockaddr.nl_pid == 0 {
				return Ok(res as _);
			}
		}
	}

	/// Low-level interface to send messages on the socket.
	///
	/// The function sends the whole content of the buffer.
	pub unsafe fn send_to(&self, buf: &[u8]) -> io::Result<()> {
		let mut i = 0;
		while i < buf.len() {
			let sockaddr = sockaddr_nl {
				nl_family: libc::AF_NETLINK as _,
				nl_pad: 0,
				nl_pid: 0,
				nl_groups: 0,
			};

			let slice = &buf[i..];
			let res = unsafe {
				libc::sendto(
					self.fd,
					slice.as_ptr() as _,
					slice.len(),
					0,
					&sockaddr as *const _ as _,
					size_of::<sockaddr_nl>() as _,
				)
			};
			if res < 0 {
				return Err(io::Error::last_os_error());
			}

			i += res as usize;
		}

		Ok(())
	}

	/// Returns the next sequence number.
	pub fn next_seq(&self) -> u32 {
		self.next_seq.fetch_add(1, atomic::Ordering::Relaxed)
	}
}

impl Drop for Netlink {
	fn drop(&mut self) {
		unsafe {
			libc::close(self.fd);
		}
	}
}

/// An iterator on netlink objects.
pub struct NetlinkIter<'sock, T> {
	/// The netlink socket.
	sock: &'sock Netlink,
	/// The sequence number on which the iterator works.
	seq: u32,

	_phantom: PhantomData<T>,
}

impl<'sock, T> Iterator for NetlinkIter<'sock, T> {
	type Item = io::Result<T>;

	fn next(&mut self) -> Option<Self::Item> {
		// TODO
		todo!()
	}
}
