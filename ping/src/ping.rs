//! This module implements pinging.

use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use std::num::NonZeroUsize;
use std::sync::Arc;
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
}

impl PingContext {
    /// TODO doc
    async fn send_packet(&self) {
        // TODO
    }

    /// Pings using the current context.
    ///
    /// The function returns when pinging is over.
    pub async fn ping(self: Arc<Self>) {
        let addr = "TODO"; // TODO resolve dns
        println!(
            "PING {} ({}) {} data bytes",
            self.dest, addr, self.packet_size
        );

        let start = Instant::now();

        let mut seq = 0;
        let mut receive_count = 0;

        while !*self.int.lock().unwrap() {
            // Break if count has been reached
            let cont = self.count.map(|c| receive_count < c.get()).unwrap_or(true);
            if !cont {
                break;
            }

            tokio::select! {
                biased;

                // Wait for interrupt signal
                _ = tokio::task::spawn_blocking(move || {
                    let mut signals = Signals::new(&[SIGINT]).unwrap();
                    signals.forever().next();
                }) => break,

                // Send packet
                _ = self.send_packet() => {}

                // Receive packet
                _ = tokio::task::spawn_blocking(move || {
                    // TODO block receiving packets
                    // TODO on receive:
                    // println!("{} bytes from {} ({}): icmp_seq={} ttl={} time={}");

                    receive_count += 1;
                }) => {}
            }
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
            transmit_count,
            receive_count,
            loss_percentage,
            elapsed.as_millis()
        );
        // TODO end:
        // println!("rtt min/avg/max/mdev = {}/{}/{}/{} ms");
    }
}
