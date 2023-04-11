//! The `ping` command allows to send ICMP ECHO_REQUEST to network hosts.

use std::env;
use std::net::IpAddr;
use std::net::Ipv4Addr;

/// Structure storing arguments.
struct Args {
	// TODO

	/// The destination address.
	dest: IpAddr,
}

impl Default for Args {
	fn default() -> Self {
		Self {
			dest: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
		}
	}
}

/// Parses command line arguments.
fn parse_args() -> Args {
	let mut iter = env::args();
	let _bin = iter.next().unwrap_or_else(|| "ping".to_owned());

	let mut args = Args::default();

	// TODO parse options

	let Some(dest) = iter.next() else {
		// TODO print usage
		todo!();
	};
	args.dest = dest.parse().unwrap(); // TODO handle error

	args
}

fn main() {
	let _args = parse_args();

    // TODO
}
