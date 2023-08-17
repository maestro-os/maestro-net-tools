//! According to the Linux documentation, the route family: receives routing and link updates and
//! may be used to modify the routing tables (both IPv4 and IPv6), IP addresses, link parameters,
//! neighbor setups, queueing disciplines, traffic classes, and packet classifiers.

use super::nlmsghdr;
use super::util;
use super::Netlink;
use super::NetlinkIter;
use super::NETLINK_ROUTE;
use std::ffi::*;
use std::io;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::NonZeroUsize;

/// Interface info message header.
#[repr(C)]
struct ifinfomsg {
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

/// A netlink socket set to the `NETLINK_ROUTE` family.
pub struct RouteNetlink {
	/// The netlink socket.
	sock: Netlink,
}

impl RouteNetlink {
	/// Creates a new instance.
	pub fn new() -> io::Result<Self> {
		Ok(Self {
			sock: Netlink::new(NETLINK_ROUTE)?,
		})
	}

	fn get_link_impl(&self, index: Option<NonZeroUsize>) -> io::Result<NetlinkIter<'_, Link>> {
		let index = index.map(NonZeroUsize::get).unwrap_or(0);
		let seq = self.sock.next_seq();

		let hdr = nlmsghdr {
			nlmsg_len: (size_of::<nlmsghdr>() + size_of::<ifinfomsg>()) as _,
			nlmsg_type: libc::RTM_GETLINK,
			nlmsg_flags: 0,
			nlmsg_seq: seq,
			nlmsg_pid: 0,
		};
		let if_hdr = ifinfomsg {
			ifi_family: libc::AF_UNSPEC as _,
			ifi_type: 0, // TODO
			ifi_index: index as _,
			ifi_flags: 0,
			ifi_change: !0,
		};

		let mut buf = vec![];
		buf.extend_from_slice(util::to_bytes(&hdr));
		buf.extend_from_slice(util::to_bytes(&if_hdr));

		unsafe { self.sock.send_to(&buf) }?;

		Ok(NetlinkIter {
			sock: &self.sock,
			seq,
			finished: false,

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
