pub mod convert;
pub extern crate image;

use image::ImageError;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub struct Error {
	msg: String,
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "{}", self.msg)
	}
}

impl From<ImageError> for Error {
	fn from(err: ImageError) -> Self {
		Self {
			msg: err.to_string()
		}
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Self {
			msg: err.to_string()
		}
	}
}

impl From<String> for Error {
	fn from(msg: String) -> Self {
		Self {
			msg
		}
	}
}

#[macro_export] macro_rules! evgfx_error {
	($($args:expr),+) => {
		Error::from(format!($($args),+))
	}
}
