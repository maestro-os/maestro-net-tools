//! This module implements pinging.

use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

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
	/// The current sequence number.
	pub seq: Mutex<usize>,
	/// Tells whether pinging was interrupted.
	pub int: Mutex<bool>,
}

impl PingContext {
	/// Pings using the current context.
	///
	/// The function returns when pinging is over.
	pub fn ping(self: Arc<Self>) {
		let addr = "TODO"; // TODO resolve dns
		println!("PING {} ({}) {} data bytes", self.dest, addr, self.packet_size);

		let start = Instant::now();

		let self1 = self.clone();
		thread::spawn(move || {
			let mut signals = Signals::new(&[SIGINT]).unwrap();
			if signals.forever().next().is_some() {
				*self1.int.lock().unwrap() = true;
			}
		});

		// Sending packets
		let self2 = self.clone();
		thread::spawn(move || {
			loop {
				let cont = self2.count
					.map(|c| *self2.seq.lock().unwrap() < c.get())
					.unwrap_or(true);
				if !cont {
					break;
				}

				// TODO send a packet

				*self2.seq.lock().unwrap() += 1;

				thread::sleep(self2.interval);
			}
		});

		let mut receive_count = 0;

		while !*self.int.lock().unwrap() {
			// Break if count has been reached
			let cont = self.count
				.map(|c| receive_count < c.get())
				.unwrap_or(true);
			if !cont {
				break;
			}

			// TODO block receiving packets
			// TODO on receive:
			// println!("{} bytes from {} ({}): icmp_seq={} ttl={} time={}");

			receive_count += 1;
		}

		let elapsed = start.elapsed();

		let transmit_count = *self.seq.lock().unwrap();
		let loss_count = if receive_count <= transmit_count {
			transmit_count - receive_count
		} else {
			0
		};
		let loss_percentage = loss_count * 100 / transmit_count;

		println!();
		println!("--- {} ping statistics ---", self.dest);
		println!(
			"{} packets transmitted, {} received, {}% packet loss, time {}ms",
			transmit_count, receive_count, loss_percentage, elapsed.as_millis()
		);
		// TODO end:
		// println!("rtt min/avg/max/mdev = {}/{}/{}/{} ms");
	}
}
