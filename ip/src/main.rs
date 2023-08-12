//! The `ip` command allows to manipulate routing, network devices and interfaces.

use std::env;

/// Prints the command usage.
///
/// `bin` is the name of the binary file.
fn print_help(bin: &str) {
	eprintln!("ip command version {}", env!("CARGO_PKG_VERSION"));
	eprintln!();
	eprintln!("Usage:");
	eprintln!("  {bin} [options] <object> <command>");
	eprintln!();
	eprintln!("Options:");
	// TODO
	eprintln!();
	eprintln!("Objects:");
	// TODO
}

fn main() {
	let mut iter = env::args();
	let bin = iter
		.next()
		.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_owned());

	// TODO
}
