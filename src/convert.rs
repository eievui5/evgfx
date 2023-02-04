use crate::Error;
use image::GenericImageView;
use image::Pixel;
use image::{Rgb, Rgba};

use std::fs::File;
use std::io::Write;

pub struct Tile {
	indexes: Vec<usize>,
}

impl Tile {
	pub fn new() -> Self {
		Self {
			indexes: Vec::<usize>::new()
		}
	}

	pub fn convert_to_4bpp(&self) -> Result<Vec<u8>, String> {
		let mut result = Vec::<u8>::new();
		for i in (0..self.indexes.len()).step_by(2) {
			// Limit the number of valid indices to 16.
			if self.indexes[i] >= 16 || self.indexes[i + 1] >= 16 {
				return Err(String::from("Input image has too many colors"));
			}
			result.push((self.indexes[i] | self.indexes[i + 1] << 4) as u8);
		}
		Ok(result)
	}
}

pub struct TileAtlas {
	pub atlas: Vec<Tile>
}

impl TileAtlas {
	pub fn new() -> Self {
		Self {
			atlas: Vec::<Tile>::new(),
		}
	}

	pub fn write_4bpp(&self, output_path: &str) -> Result<(), Error> {
		let mut output = File::create(output_path).map_err(|err| {
			format!("Failed to create: {output_path}: {err}")
		})?;

		for i in &self.atlas {
			output.write(&i.convert_to_4bpp()?)?;
		}
		Ok(())
	}
}

pub struct Palette {
	table: Vec<Rgb<u8>>,
}

impl Palette {
	pub fn new() -> Self {
		Self { table: Vec::<Rgb<u8>>::new() }
	}

	pub fn insert(&mut self, color: &Rgb<u8>) {
		self.table.push(*color);
	}

	pub fn get(&mut self, color: &Rgba<u8>) -> Option<usize> {
		for (i, c) in self.table.iter().enumerate() {
			if *c == color.to_rgb() {
				return Some(i);
			}
		}
		None
	}

	pub fn write_rgb555(&self, output_path: &str) -> Result<(), Error> {
		let mut output = File::create(output_path).map_err(|err| {
			format!("Failed to create: {output_path}: {err}")
		})?;

		for i in &self.table {
			output.write(
				&(
					(i.0[0] as u16) >> 3
					| ((i.0[1] as u16) >> 3) << 5
					| ((i.0[2] as u16) >> 3) << 10
				).to_le_bytes()
			)?;
		}
		Ok(())
	}
}

/// Configuration options for splicing images.
/// A single config can be used for multiple images.
pub struct Config {
	/// How large a metatile/sprite is within the input map.
	// For animation spritesheets this could potentially change.
	// How could variable frame sizes be handled?
	// - Read in a list of tile offsets and sizes.
	// - Take in a list of big tiles but cut them down to smaller sizes if they fit.
	// - Expose this program as a library so the user can manage the conversion of these on their own.
	pub width: u32,
	pub height: u32,
	/// How large a tile is in hardware.
	// For most systems this is probably 8, but it might be worst exposing.
	pub sub_width: u32,
	pub sub_height: u32,
	/// If set, maps a color to index 0 (transparent)
	pub transparency_color: Option<Rgb<u8>>,
	/// If the alpha channel is lower than this value, the color is transparent.
	pub alpha_threshold: u8,
}

impl Config {
	pub fn new() -> Self {
		Self {
			width: 8,
			height: 8,
			sub_width: 8,
			sub_height: 8,
			transparency_color: None,
			alpha_threshold: 128, // half seems good???
		}
	}

	/// Set the tile width and height configuration.
	pub fn with_tilesize(mut self, width: u32, height: u32) -> Self {
		self.width = width;
		self.height = height;
		self
	}

	/// Set a transparency color.
	/// If defined, this effectively reserves palette 0, even if the color is unused.
	pub fn with_transparency_color(mut self, r: u8, g: u8, b: u8) -> Self {
		self.transparency_color = Some(Rgb([r, g, b]));
		self
	}

	/// Convert an image into a list of palettes and indices.
	/// The resulting `Tile`s may be converted into a particular format.
	pub fn convert_image(&self, img_path: &str) -> Result<(Palette, TileAtlas), Error> {
		let img = &image::open(img_path).map_err(|err| {
			format!("Failed to open {img_path}: {err}")
		})?;

		let mut tiles = TileAtlas::new();
		let mut palette = Palette::new();
		if let Some(transparency_color) = self.transparency_color {
			palette.insert(&transparency_color);
		}

		for tile_y in (0..img.height()).step_by(self.height as usize) {
			for tile_x in (0..img.width()).step_by(self.width as usize) {
				for subtile_y in (tile_y..(tile_y + self.height)).step_by(self.sub_height as usize) {
					for subtile_x in (tile_x..(tile_x + self.width)).step_by(self.sub_width as usize) {
						tiles.atlas.push(create_tile(
								*img.view(
									subtile_x,
									subtile_y,
									self.sub_width,
									self.sub_height,
								),
								&mut palette,
								self.alpha_threshold,
						));
					}
				}
			}
		}
		Ok((palette, tiles))
	}
}

/// Convert an image into a list of palette indices.
fn create_tile<T: GenericImageView<Pixel = Rgba<u8>>>(
	img: T,
	palette: &mut Palette,
	alpha_threshold: u8,
) -> Tile {
	let mut tile = Tile::new();
	for y in 0..img.height() {
		for x in 0..img.width() {
			let pixel = img.get_pixel(x, y);
			if pixel.0[3] < alpha_threshold {
				tile.indexes.push(0);
				continue;
			}

			if palette.get(&pixel).is_none() {
				palette.insert(&pixel.to_rgb());
			}
			// Because we explicitly add missing colors above,
			// this is safe to unwrap.
			tile.indexes.push(palette.get(&pixel).unwrap());
		}
	}
	tile
}
