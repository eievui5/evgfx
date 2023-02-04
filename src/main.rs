use evgfx::{convert, Error, evgfx_error};
use std::env::args;
use std::process::exit;

fn cli() -> Result<(), Error> {
	let args: Vec<String> = args().collect();

	if args.len() != 4 {
		return Err(evgfx_error!("Usage:\n\t{} input output palette", args[0]));
	}

	let input_path = &args[1];
	let output_path = &args[2];
	let palette_path = &args[3];

	let (palettes, tiles) = convert::Config::new()
		.with_tilesize(16, 16)
		.with_transparency_color(0xFF, 0x00, 0xFF)
		.convert_image(input_path)?;

	tiles.write_4bpp(output_path)?;
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
