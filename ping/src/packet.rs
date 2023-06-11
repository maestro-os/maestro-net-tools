//! TODO doc

use crate::sock::RawSocket;
use std::io;
use std::mem::size_of;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::slice;

// TODO support IPv6

/// Computes a checksum on `data` according to RFC1071.
fn compute_rfc1071(data: &[u8]) -> u16 {
	let mut sum: u32 = 0;
	let mut i = 0;

	// Main loop
	while i < (data.len() & !1) {
		sum += ((data[i + 1] as u32) << 8) | (data[i] as u32);
		i += 2;
	}

	// Add remaining byte
	if i < data.len() {
		sum += data[i] as u32;
	}

	// Folding 32-bits value into 16-bits
	while (sum >> 16) != 0 {
		sum = (sum & 0xffff) + (sum >> 16);
	}

	(!sum) as u16
}

/// The ICMP header.
///
/// The header is split in two parts:
/// - The IPv4 header
/// - The actual ICMP header
///
/// The header is followed by data.
///
/// For more informations, see RFC 792.
#[repr(C, packed)]
#[derive(Debug)]
struct ICMPv4Header {
	/// The version of the header with the IHL (header length).
	version_ihl: u8,
	/// The type of service.
	type_of_service: u8,
	/// The total length of the datagram.
	total_length: u16,

	/// TODO doc
	identification: u16,
	/// TODO doc
	flags_fragment_offset: u16,

	/// Time-To-Live.
	ttl: u8,
	/// Protocol number.
	protocol: u8,
	/// The checksum of the header (RFC 1071).
	hdr_checksum: u16,

	/// Source address.
	src_addr: [u8; 4],
	/// Destination address.
	dst_addr: [u8; 4],

	// Beginning of the actual ICMP header
	/// TODO doc
	r#type: u8,
	/// TODO doc
	code: u8,
	/// TODO doc
	checksum: u16,

	// Beginning of fields specific to Echo and Echo reply messages
	/// TODO doc
	identifier: u16,
	/// The sequence number.
	seq: u16,
}

/// Writes a ping message to the given stream.
///
/// Arguments:
/// - `stream` is the stream to write to.
/// - `addr` is the destination address.
/// - `seq` is the sequence number.
/// - `ttl` is the Time to Live.
/// - `size` is the size of the packet's payload.
pub fn write_ping(
	stream: &mut RawSocket,
	addr: &IpAddr,
	seq: u16,
	ttl: u8,
	size: usize,
) -> io::Result<()> {
	let buf = match addr {
		IpAddr::V4(a) => {
			let mut hdr = ICMPv4Header {
				version_ihl: ((4 << 4) | (20 / 4) as u8).to_be(),
				type_of_service: 0,
				total_length: ((size_of::<ICMPv4Header>() + size) as u16).to_be(),

				identification: 0,
				flags_fragment_offset: 0x40, // do not fragment

				ttl,
				protocol: 1u8.to_be(),
				hdr_checksum: 0,

				src_addr: [0; 4], // INADDR_ANY
				dst_addr: a.octets(),

				r#type: 8u8.to_be(), // 8 = echo message
				code: 0,
				checksum: 0,

				identifier: 1u16.to_be(),
				seq: seq.to_be(),
			};

			// Compute header checksum
			let hdr_buf = unsafe {
				slice::from_raw_parts::<u8>(
					&hdr as *const _ as *const _,
					size_of::<ICMPv4Header>(),
				)
			};
			let chk = compute_rfc1071(&hdr_buf[..20]);
			hdr.hdr_checksum = chk;

			let mut buf: Vec<u8> = vec![0; size_of::<ICMPv4Header>() + size];

			// Write header
			let hdr_buf = unsafe {
				slice::from_raw_parts::<u8>(
					&hdr as *const _ as *const _,
					size_of::<ICMPv4Header>(),
				)
			};
			buf[..hdr_buf.len()].copy_from_slice(hdr_buf);
			println!("send: {hdr:?}");
			println!("buf: {buf:x?}");

			// Write payload
			// TODO
			for (i, b) in buf[hdr_buf.len()..].iter_mut().enumerate() {
				*b = i as _;
			}

			// Compute ICMP checksum
			let chk = compute_rfc1071(&buf[20..]);
			hdr.checksum = chk;
			let a = hdr.checksum;
			println!("a: {:x}", a);

			// Update header to add checksum
			let hdr_buf = unsafe {
				slice::from_raw_parts::<u8>(
					&hdr as *const _ as *const _,
					size_of::<ICMPv4Header>(),
				)
			};
			buf[..hdr_buf.len()].copy_from_slice(hdr_buf);

			buf
		}

		IpAddr::V6(_) => todo!(), // TODO
	};

	stream.sendto(&buf, &SocketAddr::new(*addr, 0))?;
	Ok(())
}

/// Informations about a packet reply.
pub struct ReplyInfo {
	/// The size of the entire packet. Used to discard it from the buffer.
	pub size: usize,
	/// The size of the payload in bytes.
	pub payload_size: usize,
	/// The source IP address.
	pub src_addr: IpAddr,

	/// The sequence number of the reply.
	pub seq: u16,
	/// Time to Live.
	pub ttl: u8,
}

/// Parses an ICMP packet.
///
/// The function checks checksums.
///
/// If the buffer is not large enough to fit the packet, the function returns `None`.
pub fn parse(buf: &[u8]) -> Option<ReplyInfo> {
	if buf.len() < size_of::<ICMPv4Header>() {
		return None;
	}

	let hdr = unsafe { &*(buf.as_ptr() as *const ICMPv4Header) };
	println!("{:?}", hdr);

	// TODO check hdr size
	// TODO check type of service/protocol
	// TODO check checksum

	// Check if this is a ping reply, else discard
	if hdr.r#type != 0 || hdr.code != 0 {
		// TODO discard
		return None;
	}
	if buf.len() < hdr.total_length as usize {
		return None;
	}
	// TODO check payload checksum

	Some(ReplyInfo {
		size: hdr.total_length as usize,
		payload_size: 0, // TODO
		src_addr: IpAddr::V4(hdr.src_addr.into()),

		seq: hdr.seq,
		ttl: hdr.ttl,
	})
}
