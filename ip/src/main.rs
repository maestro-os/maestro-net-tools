//! The `ip` command allows to manipulate routing, network devices and interfaces.

pub mod link;

use std::env;
use std::env::Args;
use std::iter::Peekable;
use std::process::exit;

/// Prints the command usage.
///
/// `bin` is the name of the binary file.
fn print_help(bin: &str) {
	eprintln!("ip command version {}", env!("CARGO_PKG_VERSION"));
	eprintln!();
	eprintln!("Usage:");
	eprintln!("  {bin} [options] <object> [command]");
	eprintln!();
	eprintln!("Options:");
	eprintln!("  -v, -V: show command version");
	eprintln!();
	eprintln!("Objects:");
	eprintln!("  address: TODO");
}

/// Structure with command line options.
#[derive(Default)]
struct Options {
	/// If true, show version.
	version: bool,
}

/// Parses options from command line arguments.
fn parse_options(iter: &mut Peekable<Args>) -> Option<Options> {
	let mut options = Options::default();

	loop {
		let Some(s) = iter.peek() else {
			break;
		};
		if !s.starts_with('-') {
			break;
		}
		let s = iter.next().unwrap();

		match s.as_str() {
			"-v" | "-V" => options.version = true,

			s @ _ => {
				eprintln!("invalid option `{s}`");
				return None;
			}
		}
	}

	Some(options)
}

fn main() {
	let mut iter = env::args().peekable();
	let bin = iter
		.next()
		.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_owned());

	let Some(options) = parse_options(&mut iter) else {
		print_help(&bin);
		exit(1);
	};
	if options.version {
		eprintln!("ip command version {}", env!("CARGO_PKG_VERSION"));
		exit(0);
	}

	let Some(object) = iter.next() else {
		print_help(&bin);
		exit(1);
	};

	let res = match object.as_str() {
		o @ _ if "address".starts_with(o) => link::handle_cmd(iter),
		o @ _ => {
			eprintln!("invalid command `{o}`");
			Ok(false)
		}
	};

	match res {
		Ok(false) => exit(1),
		Err(e) => {
			eprintln!("ip: {object}: {e}");
			exit(1);
		}
		_ => {}
	}
}
