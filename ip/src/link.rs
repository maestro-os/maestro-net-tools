//! A link is a network interface.

use netlink::route::RouteNetlink;
use std::env::Args;
use std::io;
use std::iter::Peekable;

/// Prints help for the `address` object.
fn print_help() {
	// TODO
	todo!()
}

/// Prints network interfaces list with details.
fn print_list() -> io::Result<()> {
	let sock = RouteNetlink::new()?;
	for link in sock.list_links()? {
		let link = link?;
		println!(
			"{}: TODO: TODO",
			link.index,
			//link.name.unwrap_or_default().display()
		);
		// TODO
	}

	Ok(())
}

/// Handles the command in the given iterator.
///
/// On success, the function returns `true`, else it returns `false`.
pub fn handle_cmd(mut iter: Peekable<Args>) -> io::Result<bool> {
	let cmd = iter.next();

	match cmd.as_deref() {
		Some(c) if "help".starts_with(c) => {
			print_help();
			Ok(true)
		}

		None => {
			print_list()?;
			Ok(false)
		}

		Some(c) if "show".starts_with(c) => {
			print_list()?;
			Ok(true)
		}

		Some(c) => {
			eprintln!("Command {c} unknown. Type \"ip address help\" for help");
			Ok(false)
		}
	}
}
