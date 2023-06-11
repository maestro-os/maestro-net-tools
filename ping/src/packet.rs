//! TODO doc

use crate::sock::IcmpSocket;
use std::io;
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
/// The header is followed by data.
///
/// For more informations, see RFC 792.
#[repr(C, packed)]
#[derive(Debug)]
struct ICMPv4Header {
	/// Packet type
	r#type: u8,
	/// Packet code
	code: u8,
	/// Checksum of the header + data
	checksum: u16,

	// Beginning of fields specific to Echo and Echo reply messages
	/// The identifier of the ping sequence.
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
	stream: &mut IcmpSocket,
	addr: &IpAddr,
	seq: u16,
	size: usize,
) -> io::Result<()> {
	let buf = match addr {
		IpAddr::V4(_) => {
			let mut hdr = ICMPv4Header {
				r#type: 8u8.to_be(), // 8 = echo message
				code: 0,
				checksum: 0,

				identifier: 1u16.to_be(),
				seq: seq.to_be(),
			};
			let mut buf: Vec<u8> = vec![0; size_of::<ICMPv4Header>() + size];

			// Write header
			let hdr_buf = unsafe {
				slice::from_raw_parts::<u8>(
					&hdr as *const _ as *const _,
					size_of::<ICMPv4Header>(),
				)
			};
			buf[..hdr_buf.len()].copy_from_slice(hdr_buf);

			// Write payload
			// TODO
			for (i, b) in buf[hdr_buf.len()..].iter_mut().enumerate() {
				*b = i as _;
			}

			// Compute ICMP checksum
			let chk = compute_rfc1071(&buf);
			hdr.checksum = chk;

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

	stream.sendto(&buf, addr)?;
	Ok(())
}

/// Informations about a packet reply.
pub struct ReplyInfo {
	/// The sequence number of the reply.
	pub seq: u16,
	/// The size of the payload in bytes.
	pub payload_size: usize,
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

	// Check if this is a ping reply, else discard
	if hdr.r#type != 0 || hdr.code != 0 {
		return None;
	}
	// Check checksum
	if compute_rfc1071(buf) != 0 {
		return None;
	}

	// TODO check payload content

	Some(ReplyInfo {
		seq: u16::from_be(hdr.seq),
		payload_size: buf.len() - size_of::<ICMPv4Header>(),
	})
}
