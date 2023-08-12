//! Netlink is a kernel feature allowing to inspect and control network interfaces.
//!
//! The netlink interface can be accessed through a socket.

use core::ffi::c_int;
use std::io;

/// Netlink message header.
#[repr(C)]
struct NlMsgHdr {
	/// Length of the message including header.
	nlmsg_len: u32,
	/// Type of message content.
	nlmsg_type: u16,
	/// Additional flags.
	nlmsg_flags: u16,
	/// Sequence number.
	nlmsg_seq: u32,
	/// Sender port ID.
	nlmsg_pid: u32,
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
}
