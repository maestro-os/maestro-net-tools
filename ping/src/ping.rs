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
	/// The interval between echo packets.
	pub interval: Duration,
	/// The timeout before `ping` exits regardless of how many packets have been sent.
	///
	/// If `None`, there is no deadline.
	pub deadline: Option<Duration>,
	/// The time to wait for a response for each packet.
	pub timeout: Duration,
	/// The size of packets to be sent.
	pub packet_size: usize,
	/// IP Time To Live.
	pub ttl: u32,

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

		let mut seq = 0;

		loop {
			// Break if count has been reached
			let cont = self.count
				.map(|c| seq >= c.get())
				.unwrap_or(true);
			if !cont {
				break;
			}

			// TODO
			println!("send");

			seq += 1;

			thread::sleep(self.interval);
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
