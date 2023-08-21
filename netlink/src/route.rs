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

/// Routing attribute: interface L2 address
const IFLA_ADDRESS: c_ushort = 1;
/// Routing attribute: L2 broadcast address
const IFLA_BROADCAST: c_ushort = 2;
/// Routing attribute: Device name
const IFLA_IFNAME: c_ushort = 3;
/// Routing attribute: MTU of the device
const IFLA_MTU: c_ushort = 4;
/// Routing attribute: Link type
const IFLA_LINK: c_ushort = 5;
/// Routing attribute: Queueing discipline
const IFLA_QDISC: c_ushort = 6;
/// Routing attribute: Queueing discipline
const IFLA_STATS: c_ushort = 7;

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

/// Route attribute
#[repr(C)]
struct rtattr {
	/// Length of option
	rta_len: c_ushort,
	/// Type of option
	rta_type: c_ushort,
}

/// A network interface
pub struct Link {
	/// The device's index
	pub index: usize,

	/// Interface L2 address
	pub address: Option<[u8; 6]>,
	/// Broadcast L2 address
	pub broadcast: Option<[u8; 6]>,
	/// Interface name
	pub name: Option<CString>,
	// TODO
}

impl TryFrom<&[u8]> for Link {
	// TODO: have a correct error type
	type Error = ();

	fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
		let hdr: &ifinfomsg = unsafe { util::reinterpret(val) }.ok_or(())?;
		let mut link = Link {
			index: hdr.ifi_index as _,

			address: None,
			broadcast: None,
			name: None,
			// TODO
		};

		// iterate on attributes
		let mut i = size_of::<ifinfomsg>();
		while i < val.len() {
			let hdr: &rtattr = unsafe { util::reinterpret(val) }.ok_or(())?;

			let data = &val[(i + size_of::<rtattr>())..];
			match hdr.rta_type {
				IFLA_ADDRESS => link.address = Some(data.try_into().map_err(|_| ())?),
				IFLA_BROADCAST => link.broadcast = Some(data.try_into().map_err(|_| ())?),
				IFLA_IFNAME => link.name = Some(CString::new(data).map_err(|_| ())?),
				IFLA_MTU => {}   // TODO
				IFLA_LINK => {}  // TODO
				IFLA_QDISC => {} // TODO
				IFLA_STATS => {
					// TODO rtnl_link_stats
				}

				_ => {}
			}

			i += hdr.rta_len as usize;
		}

		Ok(link)
	}
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

	fn get_link_impl(
		&self,
		index: Option<NonZeroUsize>,
		single: bool,
	) -> io::Result<NetlinkIter<'_, Link>> {
		let index = index.map(NonZeroUsize::get).unwrap_or(0);
		let seq = self.sock.next_seq();
		let flags = if single {
			libc::NLM_F_REQUEST
		} else {
			libc::NLM_F_REQUEST | libc::NLM_F_DUMP
		};

		let hdr = nlmsghdr {
			nlmsg_len: (size_of::<nlmsghdr>() + size_of::<ifinfomsg>()) as _,
			nlmsg_type: libc::RTM_GETLINK,
			nlmsg_flags: flags as _,
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
		self.get_link_impl(None, true)
	}

	/// Returns the network interface with the given index.
	///
	/// If the interface doesn't exist, the function returns `None`.
	pub fn get_link(&self, index: NonZeroUsize) -> io::Result<Option<Link>> {
		self.get_link_impl(Some(index), false)?.next().transpose()
	}
}
