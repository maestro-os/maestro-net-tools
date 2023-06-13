//! This module implements address utilities.

use std::ffi::CString;
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::ptr::null;
use std::ptr::null_mut;
use std::str::FromStr;

// TODO implement selection between ipv4 and ipv6
/// Resolves the given name and returns the associated address.
fn resolve_name(name: &str) -> io::Result<IpAddr> {
	let name = CString::new(name)?;
	let mut result = null_mut::<_>();

	let res = unsafe { libc::getaddrinfo(name.as_ptr(), null::<_>(), null::<_>(), &mut result) };
	if res < 0 {
		return Err(io::Error::last_os_error());
	}

	// Get results
	let mut r = result;
	let ip = loop {
		if r.is_null() {
			break None;
		}
		let res = unsafe { &*r };

		match res.ai_family {
			libc::AF_INET => {
				let data = unsafe { &(*res.ai_addr).sa_data };
				let ip = IpAddr::V4(Ipv4Addr::new(
					data[2] as _,
					data[3] as _,
					data[4] as _,
					data[5] as _,
				));
				break Some(ip);
			}

			// TODO support IPV6

			_ => {}
		}

		r = res.ai_next;
	};

	unsafe {
		libc::freeaddrinfo(result);
	}

	ip.ok_or(io::Error::new(io::ErrorKind::Other, "Name or service not known"))
}

/// Parses the given address.
///
/// If the address is a domain name, it is parsed and the function returns the corresponding
/// address.
pub fn parse(addr: &str) -> io::Result<IpAddr> {
	let res = IpAddr::from_str(addr);
	match res {
		Ok(a) => Ok(a),
		Err(_) => resolve_name(addr),
	}
}
