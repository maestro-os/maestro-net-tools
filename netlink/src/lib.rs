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
pub struct sockaddr_nl {
	/// `AF_NETLINK`
	pub nl_family: libc::sa_family_t,
	/// Padding (zero)
	pub nl_pad: c_ushort,
	/// Port ID
	pub nl_pid: libc::pid_t,
	/// Multicast groups mask
	pub nl_groups: u32,
}

/// Netlink message header.
#[repr(C)]
#[derive(Clone)]
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

	unsafe fn recv_from_impl(
		&self,
		buf: &mut [u8],
		peek: bool,
	) -> io::Result<(usize, sockaddr_nl)> {
		let mut sockaddr = sockaddr_nl::default();

		loop {
			let flags = if peek {
				libc::MSG_PEEK | libc::MSG_TRUNC
			} else {
				0
			};
			let res = unsafe {
				libc::recvfrom(
					self.fd,
					buf.as_mut_ptr() as _,
					buf.len(),
					flags,
					&mut sockaddr as *mut _ as *mut _,
					size_of::<sockaddr_nl>() as _,
				)
			};
			if res < 0 {
				return Err(io::Error::last_os_error());
			}

			// ignore messages that do not come from the kernel
			if sockaddr.nl_pid == 0 {
				return Ok((res as _, sockaddr));
			}
		}
	}

	/// Low-level interface to peek the length of the next message to be received with
	/// [`recv_from`] on the socket.
	pub unsafe fn peek(&self) -> io::Result<(usize, sockaddr_nl)> {
		self.recv_from_impl(&mut [], true)
	}

	/// Low-level interface to receive messages from the socket.
	///
	/// The function blocks untils a message is received from the socket, then writes it on the
	/// given buffer.
	///
	/// On success, the function returns a tuple with:
	/// - The number of bytes read
	/// - The sockaddr
	pub unsafe fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, sockaddr_nl)> {
		self.recv_from_impl(buf, false)
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

	/// Returns the next received message for the given sequence number.
	///
	/// If no message is buffered for this sequence, the function blocks until one is received.
	fn next_msg(&self, seq: u32) -> io::Result<(nlmsghdr, Vec<u8>)> {
		loop {
			// TODO check buffer

			// read message from buffer
			let (len, _) = unsafe { self.peek() }?;
			let mut buf = vec![0; len];
			let (len, _) = unsafe { self.recv_from(&mut buf) }?;
			// if the message is not large enough to fit the header, discard it and wait for next
			// message
			if len < size_of::<nlmsghdr>() {
				continue;
			}

			// get buffer's header
			let hdr: &nlmsghdr = unsafe { util::reinterpret(&buf) }.unwrap();
			// if the message is not part of the requested sequence, buffer it and wait for next
			// message
			if hdr.nlmsg_seq != seq {
				// TODO buffer
				continue;
			}

			return Ok((hdr.clone(), buf[size_of::<nlmsghdr>()..].to_vec()));
		}
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
pub struct NetlinkIter<'sock, T: for<'a> TryFrom<&'a [u8]>> {
	/// The netlink socket.
	sock: &'sock Netlink,
	/// The sequence number on which the iterator works.
	seq: u32,
	/// If `true`, the iterator is finished.
	finished: bool,

	_phantom: PhantomData<T>,
}

impl<'sock, T: for<'a> TryFrom<&'a [u8]>> Iterator for NetlinkIter<'sock, T> {
	type Item = io::Result<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.finished {
			return None;
		}

		// get next message
		let (hdr, msg) = match self.sock.next_msg(self.seq) {
			Ok(m) => m,
			Err(e) => return Some(Err(e)),
		};

		// if the message is singlepart, mark the iterator as finished
		if hdr.nlmsg_flags & (libc::NLM_F_MULTI as u16) == 0
			|| hdr.nlmsg_type == libc::NLMSG_DONE as u16
		{
			self.finished = true;
		}

		// TODO handle error
		let elem = T::try_from(&msg).ok().unwrap();
		Some(Ok(elem))
	}
}
