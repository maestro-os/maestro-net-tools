//! TODO doc

use std::io::Write;
use std::io;
use std::mem::size_of;

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
pub struct ICMPv4Header {
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
}

/// Writes a ping message to the given stream.
///
/// Arguments:
/// - `stream` is the stream to write to.
/// - `seq` is the sequence number.
pub fn write_ping<S: Write>(stream: &mut S, seq: usize) -> io::Result<()> {
	// TODO
	todo!()
}

/// Parses an ICMP packet.
///
/// The function checks checksums.
///
/// If the buffer is not large enough to fit the packet, the function returns `None`.
pub fn parse(buf: &[u8]) -> Option<()> {
	if buf.len() < size_of::<ICMPv4Header>() {
		return None;
	}

	// TODO
	todo!()
}
