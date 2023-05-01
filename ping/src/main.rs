//! The `ping` command allows to send ICMP ECHO_REQUEST to network hosts.

mod packet;
mod ping;
mod sock;
mod timer;

use ping::PingContext;
use sock::RawSocket;
use std::env;
use std::num::NonZeroUsize;
use std::process::exit;
use std::time::Duration;

/// The program's version
const VERSION: &str = "0.1";

/// Structure storing arguments.
struct Args {
    /// The number of packets to send.
    ///
    /// If `None`, there is no limit.
    count: Option<NonZeroUsize>,
    /// If `true`, pinging broadcast is allowed.
    allow_broadcast: bool,
    /// The interval between echo packets.
    ///
    /// If `None`, the default value is used.
    interval: Option<Duration>,
    /// The timeout before `ping` exits regardless of how many packets have been sent.
    deadline: Option<Duration>,
    /// The time to wait for a response for each packet.
    ///
    /// If `None`, the default value is used.
    timeout: Option<Duration>,
    /// The size of packets to be sent.
    ///
    /// If `None`, the default value is used.
    packet_size: Option<usize>,
    /// IP Time To Live.
    ///
    /// If `None`, the default value is used.
    ttl: Option<u8>,

    /// The destination address or hostname.
    dest: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            count: None,
            allow_broadcast: false,
            interval: None,
            deadline: None,
            timeout: None,
            packet_size: None,
            ttl: None,

            dest: String::new(),
        }
    }
}

/// Prints the command usage.
///
/// `bin` is the name of the binary file.
fn print_usage(bin: &str) {
    eprintln!("Usage:");
    eprintln!("  {bin} [options] <destination>");
    eprintln!();
    eprintln!("  <destination> is the DNS or IP to the host");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -4\t\tuse IPv4");
    eprintln!("  -6\t\tuse IPv6");
    eprintln!("  -c <count>\tstop after <count> replies");
    eprintln!("  -b\t\tallow pinging broadcast");
    eprintln!("  -i <interval>\tseconds between each packet sent");
    eprintln!("  -s <size>\tnumber of bytes to be sent");
    eprintln!("  -t <ttl>\tdefines TTL");
    eprintln!("  -V\t\tprints version, then exits");
    eprintln!("  -w <deadline>\ttime to wait before exit in seconds");
    eprintln!("  -W <timeout>\ttime to wait for a reply in seconds");
    eprintln!("  -h\t\tprints this help");
}

/// Parses command line arguments.
fn parse_args() -> Args {
    let mut iter = env::args();
    let bin = iter.next().unwrap_or_else(|| "ping".to_owned());

    let mut args = Args::default();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-4" => todo!(), // TODO

            "-6" => todo!(), // TODO

            "-c" => {
                let count = iter
                    .next()
                    .map(|s| s.parse())
                    .transpose()
                    .unwrap_or_else(|e| {
                        eprintln!("{bin}: {e}");
                        exit(1);
                    })
                    .unwrap_or_else(|| {
                        print_usage(&bin);
                        exit(1);
                    });

                args.count = Some(count);
            }

            "-b" => args.allow_broadcast = true,

            "-i" => {
                let interval = iter
                    .next()
                    .map(|s| {
                        s.parse()
                            .map(|i: f32| Duration::from_millis((i * 1000.) as _))
                    })
                    .transpose()
                    .unwrap_or_else(|e| {
                        eprintln!("{bin}: {e}");
                        exit(1);
                    })
                    .unwrap_or_else(|| {
                        print_usage(&bin);
                        exit(1);
                    });

                args.interval = Some(interval);
            }

            "-s" => {
                let size = iter
                    .next()
                    .map(|s| s.parse())
                    .transpose()
                    .unwrap_or_else(|e| {
                        eprintln!("{bin}: {e}");
                        exit(1);
                    })
                    .unwrap_or_else(|| {
                        print_usage(&bin);
                        exit(1);
                    });

                args.packet_size = Some(size);
            }

            "-t" => {
                let ttl = iter
                    .next()
                    .map(|s| s.parse())
                    .transpose()
                    .unwrap_or_else(|e| {
                        eprintln!("{bin}: {e}");
                        exit(1);
                    })
                    .unwrap_or_else(|| {
                        print_usage(&bin);
                        exit(1);
                    });

                args.ttl = Some(ttl);
            }

            "-V" => {
                println!("ping (maestro-net-tools) version {VERSION}");
                exit(0);
            }

            "-w" => todo!(), // TODO

            "-W" => todo!(), // TODO

            "-h" => {
                print_usage(&bin);
                exit(0);
            }

            _ => args.dest = arg,
        }
    }

    if args.dest.is_empty() {
        print_usage(&bin);
        exit(1);
    }

    args
}

fn main() {
    let args = parse_args();

    let sock = RawSocket::new().unwrap_or_else(|e| {
        // TODO print error
        exit(1);
    });

    let mut ctx = PingContext {
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
        eprintln!("ping: {}: {}", ctx.dest, e);
        exit(1);
    }
}
