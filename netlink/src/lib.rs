//! Netlink is a kernel feature allowing to inspect and control network interfaces.
//!
//! The netlink interface can be accessed from userspace through a socket.

pub mod util;

use std::ffi::c_int;
use std::ffi::c_uchar;
use std::ffi::c_uint;
use std::ffi::c_ushort;
use std::io;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::NonZeroUsize;

/// Netlink message header.
#[repr(C)]
struct NlMsgHdr {
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

/// Interface info message header.
#[repr(C)]
struct IfInfoMsg {
	/// TODO doc
	ifi_family: c_uchar,
	/// Device type
	ifi_type: c_ushort,
	/// Interface index
	ifi_index: c_int,
	/// Device flags
	ifi_flags: c_uint,
	/// TODO doc
	ifi_change: c_uint,
}

/// TODO doc
pub struct Link {
	// TODO
}

/// A netlink socket.
pub struct Netlink {
	/// The socket's file descriptor.
	fd: c_int,
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
		})
	}

	/// Low-level interface to send messages on the socket.
	pub unsafe fn send_to(&self, _buf: &[u8]) -> io::Result<()> {
		// TODO
		todo!()
	}

	fn get_link_impl(&self, index: Option<NonZeroUsize>) -> io::Result<NetlinkIter<'_, Link>> {
		let index = index.map(NonZeroUsize::get).unwrap_or(0);
		// TODO
		let seq = 0;

		let hdr = NlMsgHdr {
			nlmsg_len: (size_of::<NlMsgHdr>() + size_of::<IfInfoMsg>()) as _,
			nlmsg_type: libc::RTM_GETLINK,
			nlmsg_flags: 0,
			nlmsg_seq: seq,
			nlmsg_pid: 0,
		};
		let if_hdr = IfInfoMsg {
			ifi_family: libc::AF_UNSPEC as _,
			ifi_type: 0, // TODO
			ifi_index: index as _,
			ifi_flags: 0, // TODO
			ifi_change: !0,
		};

		let mut buf = vec![];
		buf.extend_from_slice(util::to_bytes(&hdr));
		buf.extend_from_slice(util::to_bytes(&if_hdr));

		unsafe { self.send_to(&buf) }?;

		Ok(NetlinkIter {
			sock: self,
			seq,

			_phantom: PhantomData,
		})
	}

	/// Lists network interfaces.
	pub fn list_links(&self) -> io::Result<NetlinkIter<'_, Link>> {
		self.get_link_impl(None)
	}

	/// Returns the network interface with the given index.
	///
	/// If the interface doesn't exist, the function returns `None`.
	pub fn get_link(&self, index: NonZeroUsize) -> io::Result<Option<Link>> {
		self.get_link_impl(Some(index))?.next().transpose()
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
