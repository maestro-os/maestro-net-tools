//! This module implements pinging.

use std::num::NonZeroUsize;
use std::thread;
use std::time::Duration;

/// A pinging context.
pub struct PingContext {
	/// The number of packets to send.
	///
	/// If `None`, there is no limit.
	pub count: Option<NonZeroUsize>,
	/// The size of packets to be sent.
	pub packet_size: usize,

	/// The destination address or hostname.
	pub dest: String,
}

impl PingContext {
	/// Pings using the current context.
	///
	/// The function returns when pinging is over.
	pub fn ping(&mut self) {
		let addr = "TODO"; // TODO get IP for dest
		println!("PING {} ({}) {} data bytes", self.dest, addr, self.packet_size);

		let interval = Duration::from_millis(1000); // TODO take from params

		let mut seq = 0;

		loop {
			// Break if count has been reached
			match self.count {
				Some(count) if seq >= count.get() => break,
				_ => {},
			}

			// TODO
			println!("send");

			seq += 1;

			thread::sleep(interval);
		}

		// TODO on receive:
		// println!("{} bytes from {} ({}): icmp_seq={} ttl={} time={}");

		println!();
		println!("--- {} ping statistics ---", self.dest);
		// TODO end:
		// println!("{} packets transmitted, {} received, {}% packet loss, time {}");
		// println!("rtt min/avg/max/mdev = {}/{}/{}/{} ms");
	}
}
