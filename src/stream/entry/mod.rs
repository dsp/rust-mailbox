//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

#[derive(Clone, Debug)]
pub enum Entry {
	Begin(Begin),
	Escape(String),
	Header(Header),
	Body(Vec<u8>),
	End,
}

mod begin;
pub use self::begin::Begin;

pub mod header;
pub use self::header::Header;

const WS: &'static [u8] = b" \t";

#[inline(always)]
fn is_whitespace(c: u8) -> bool {
	c == b' ' || c == b'\t'
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn begin_ok() {
		let v = Begin::new("From foo@example.com Wed Nov 17 14:35:53 2010".into()).unwrap();
		assert_eq!(v.address(), "foo@example.com");
		assert_eq!(v.timestamp(), "Wed Nov 17 14:35:53 2010");
	}

	#[test]
	fn begin_fail() {
		assert!(Begin::new("From foo@example.com".into()).is_err());
	}

	#[test]
	fn header_ok() {
		let v = Header::new("From: meh. <meh@schizofreni.co>".into()).unwrap();
		assert_eq!(&*v.key(), "From");
		assert_eq!(v.value(), "meh. <meh@schizofreni.co>");
	}

	#[test]
	fn header_fail() {
		assert!(Header::new("From foo@example.com".into()).is_err());
	}
}