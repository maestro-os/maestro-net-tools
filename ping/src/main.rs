//! The `ping` command allows to send ICMP ECHO_REQUEST to network hosts.

mod ping;

use ping::PingContext;
use std::env;
use std::num::NonZeroUsize;

/// Structure storing arguments.
struct Args {
	// TODO

	/// The number of packets to send.
	///
	/// If `None`, there is no limit.
	count: Option<NonZeroUsize>,
	/// The size of packets to be sent.
	packet_size: usize,

	/// The destination address or hostname.
	dest: String,
}

impl Default for Args {
	fn default() -> Self {
		Self {
			// TODO

			count: None,
			packet_size: 56,

			dest: String::new(),
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
	let args = parse_args();

	let mut ctx = PingContext {
		count: args.count,
		packet_size: args.packet_size,

		dest: args.dest,
	};
    ctx.ping();
}
