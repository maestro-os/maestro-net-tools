//! TODO doc

use std::io;
use std::io::Write;
use std::mem::size_of;
use std::net::IpAddr;
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
/// - `seq` is the sequence number.
/// - `ttl` is the Time to Live.
/// - `size` is the size of the packet's payload.
pub fn write_ping<S: Write>(stream: &mut S, seq: u16, ttl: u8, size: usize) -> io::Result<()> {
	let mut hdr = ICMPv4Header {
		version_ihl: 4 | ((20 / 4) << 4) as u8,
		type_of_service: 0,
		total_length: ((size_of::<ICMPv4Header>() + size) as u16).to_be(),

		identification: 0,
		flags_fragment_offset: 0,

		ttl,
		protocol: 1,
		hdr_checksum: 0, // TODO

		src_addr: [0; 4],         // INADDR_ANY
		dst_addr: [127, 0, 0, 1], // TODO

		r#type: 8, // 8 = echo message
		code: 0,
		checksum: 0, // TODO

		identifier: 0,
		seq,
	};

	// Compute header checksum
	let hdr_buf = unsafe {
		slice::from_raw_parts::<u8>(&hdr as *const _ as *const _, size_of::<ICMPv4Header>())
	};
	let chk = compute_rfc1071(&hdr_buf[..20]);
	hdr.hdr_checksum = chk;

	// Compute total checksum
	let hdr_buf = unsafe {
		slice::from_raw_parts::<u8>(&hdr as *const _ as *const _, size_of::<ICMPv4Header>())
	};
	let chk = compute_rfc1071(hdr_buf);
	hdr.checksum = chk;

	// Write header
	let hdr_buf = unsafe {
		slice::from_raw_parts::<u8>(&hdr as *const _ as *const _, size_of::<ICMPv4Header>())
	};
	stream.write(hdr_buf)?;

	// TODO fill with garbage instead?
	// Write payload
	let buf = vec![0; size];
	stream.write(&buf)?;

	stream.flush()
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

	// TODO
	todo!()
}
