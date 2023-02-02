use evgfx::convert::{self, write_graphics};
use evgfx::Error;
use std::env::args;
use std::process::exit;

fn cli() -> Result<(), Error> {
	let args: Vec<String> = args().collect();

	if args.len() != 4 {
		return Err(Error::from(format!(
			"Usage:\n\t{} input output palette",
			args[0],
		)));
	}

	let input_path = &args[1];
	let output_path = &args[2];
	let palette_path = &args[3];

	let (palettes, tiles) = convert::Config::new()
		.with_tilesize(16, 16)
		.with_transparency_color(0xFF, 0x00, 0xFF)
		.splice_image(&image::open(input_path).map_err(|err| {
			format!("Failed to open {output_path}: {err}")
		})?);

	write_graphics(tiles, output_path)?;
	palettes.write_rgb555(palette_path)?;

	Ok(())
}

fn main() {
	// Once try/catch is implemented, cli() can be inlined.
	if let Err(err) = cli() {
		eprintln!("{err}");
		exit(1);
	}
}
