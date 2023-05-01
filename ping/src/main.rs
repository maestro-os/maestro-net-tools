//! The `ping` command allows to send ICMP ECHO_REQUEST to network hosts.

mod ping;
mod sock;

use ping::PingContext;
use sock::RawSocket;
use std::env;
use std::num::NonZeroUsize;
use std::process::exit;
use std::time::Duration;

/// Structure storing arguments.
struct Args {
    /// The number of packets to send.
    ///
    /// If `None`, there is no limit.
    count: Option<NonZeroUsize>,
    /// The interval between echo packets.
    ///
    /// If `None`, a duration of 1 second is used.
    interval: Option<Duration>,
    /// The timeout before `ping` exits regardless of how many packets have been sent.
    deadline: Option<Duration>,
    /// The time to wait for a response for each packet.
    ///
    /// If `None`, TODO
    timeout: Option<Duration>,
    /// The size of packets to be sent.
    packet_size: Option<usize>,
    /// IP Time To Live.
    ///
    /// If `None`, TODO
    ttl: Option<u32>,

    /// The destination address or hostname.
    dest: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            count: None,
            interval: None,
            deadline: None,
            timeout: None,
            packet_size: None,
            ttl: None,

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

	let sock = RawSocket::new()
		.unwrap_or_else(|e| {
			// TODO print error
			exit(1);
		});

    let ctx = PingContext {
        count: args.count,
        interval: args.interval.unwrap_or(Duration::from_secs(1)),
        deadline: args.deadline,
        timeout: args.timeout.unwrap_or(Duration::from_secs(2)),
        packet_size: args.packet_size.unwrap_or(56),
        ttl: args.ttl.unwrap_or(115),

        dest: args.dest,

		sock,
    };

    if let Err(e) = ctx.ping() {
		// TODO print error
		exit(1);
	}
}
