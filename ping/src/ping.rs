//! This module implements pinging.

use signal_hook::consts::{SIGALRM, SIGINT};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
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
}

impl PingContext {
    /// TODO doc
	///
	/// `seq` is the sequence number of the packet to send.
    fn send_packet(&self, seq: usize) {
        // TODO
    }

	/// Sets the timer to the given duration.
	fn set_timer(_d: Duration) {
		// TODO
		todo!();
	}

    /// Pings using the current context.
    ///
    /// The function returns when pinging is over.
    pub fn ping(self) {
        let addr = "TODO"; // TODO resolve dns
        println!(
            "PING {} ({}) {} data bytes",
            self.dest, addr, self.packet_size
        );

		// Catch signals
		let alarm = Arc::new(AtomicBool::new(false));
		let int = Arc::new(AtomicBool::new(false));
		signal_hook::flag::register(SIGALRM, Arc::clone(&alarm)).unwrap();
		signal_hook::flag::register(SIGINT, Arc::clone(&int)).unwrap();

		// Start timer
		Self::set_timer(self.interval);

        let start = Instant::now();

        let mut transmit_count = 0;
        let mut receive_count = 0;

        loop {
            // Break if count has been reached
            let cont = self.count.map(|c| receive_count < c.get()).unwrap_or(true);
            if int.load(Ordering::Relaxed) || !cont {
                break;
            }

			// Send signal if interval has been reached
			if alarm.load(Ordering::Relaxed) {
				self.send_packet(transmit_count);
				transmit_count += 1;

				// Reset timer
				alarm.store(false, Ordering::Relaxed);
				Self::set_timer(self.interval);
			}

			// Receive packet
			// TODO on receive:
			// println!("{} bytes from {} ({}): icmp_seq={} ttl={} time={}");

			// TODO do not increment if read syscall is interrupted
			receive_count += 1;
        }

        let elapsed = start.elapsed();

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
            transmit_count,
            receive_count,
            loss_percentage,
            elapsed.as_millis()
        );
        // TODO end:
        // println!("rtt min/avg/max/mdev = {}/{}/{}/{} ms");
    }
}
