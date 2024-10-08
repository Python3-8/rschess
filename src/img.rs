//! Generate `image-rs` images of `Position`s.

use super::{helpers, Color, InvalidHexError, InvalidPositionImagePropertiesError, Position};
use image::{imageops, Rgba, RgbaImage};
use include_dir::{include_dir, Dir};
use nsvg;
use std::{collections::HashMap, path::PathBuf};

static ASSETS_DIR: Dir = include_dir!("assets");

/// Represents an RGB color.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    /// Creates a new instance of `Rgb`.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Attempts to create a new instance of `Rgb`, from a hex color,
    /// returning an error if the given value is invalid.
    pub fn from_hex(hex: &str) -> Result<Self, InvalidHexError> {
        let hex = hex.replace('#', "");
        if hex.len() != 6 {
            return Err(InvalidHexError(hex));
        }
        let mut values = Vec::new();
        for pair in hex.chars().collect::<Vec<_>>().chunks(2) {
            let pair: String = pair.iter().collect();
            if let Ok(v) = u8::from_str_radix(&pair, 16) {
                values.push(v)
            } else {
                return Err(InvalidHexError(hex));
            }
        }
        Ok(Self(values[0], values[1], values[2]))
    }

    /// Returns the RGB values of this `Rgb` object.
    pub fn values(&self) -> (u8, u8, u8) {
        let Self(r, g, b) = self;
        (*r, *g, *b)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PieceSet {
    /// For built-in piece sets, a set of 27 fixed
    /// options is available. These are the piece sets owned by and
    /// [listed as free to use](https://github.com/lichess-org/lila/blob/master/COPYING.md#exceptions-free)
    /// by Lichess.org.
    Builtin(String),
    /// A custom piece set must include a `HashMap` with the keys
    /// representing the pieces ("wK", "wN", "bP", etc.) and the values
    /// depicting the pieces.
    Custom(HashMap<String, RgbaImage>),
}

impl Default for PieceSet {
    fn default() -> Self {
        Self::Builtin("default".to_owned())
    }
}

/// Represents the properties of an image generated from a position.
/// The board theme can be customized with custom colors for the
/// light and dark squares, the size of the board, and custom piece sets.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PositionImageProperties {
    /// The color to be used for the light squares of the board
    pub light_square_color: Rgb,
    /// The color to be used for the dark squares of the board
    pub dark_square_color: Rgb,
    /// The piece set to use
    pub piece_set: PieceSet,
    /// The width and height of the board in pixels; this value must be greater than or equal to 8
    pub size: usize,
}

impl Default for PositionImageProperties {
    /// The default `PositionImageProperties` has light squares colored `#f3f3f4`, dark squares
    /// colored `#639a59`, the default piece set ([CBurnett's SVG chess pieces](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces#/media/File:Chess_Pieces_Sprite.svg)),
    /// and a 512px by 512px board.
    fn default() -> Self {
        Self {
            light_square_color: Rgb::from_hex("#f3f3f4").unwrap(),
            dark_square_color: Rgb::from_hex("#639a59").unwrap(),
            piece_set: PieceSet::Builtin("default".to_owned()),
            size: 512,
        }
    }
}

/// Creates an image of a `Position`, from the perspective of the side `perspective`.
pub fn position_to_image(position: &Position, props: PositionImageProperties, perspective: Color) -> Result<RgbaImage, InvalidPositionImagePropertiesError> {
    let PositionImageProperties {
        light_square_color,
        dark_square_color,
        piece_set,
        size,
    } = props;
    if size < 8 {
        return Err(InvalidPositionImagePropertiesError::InvalidSize(size));
    }
    let piece_set_name = match &piece_set {
        PieceSet::Builtin(name) => Some({
            let name = name.trim().to_lowercase().replace(' ', "-");
            match name.as_str() {
                "default" | "normal" => "cburnett".to_owned(),
                _ => name,
            }
        }),
        _ => None,
    };
    let mut content = position.content.into_iter().enumerate().collect::<Vec<_>>();
    let ranks: Vec<_> = if perspective.is_white() {
        content.chunks(8).rev().enumerate().collect()
    } else {
        content.reverse();
        content.chunks(8).rev().enumerate().collect()
    };
    let piece_size = size / 8;
    let mut board_image = RgbaImage::new(size as u32, size as u32);
    for (ranki, rank) in ranks {
        for (sqi, (sq, occ)) in rank.iter().enumerate() {
            let sq_color = if helpers::color_complex_of(*sq) { light_square_color } else { dark_square_color };
            let sq_x = sqi * piece_size;
            let sq_y = ranki * piece_size;
            if let Some(piece) = occ {
                let piece_str = format!("{}{}", piece.color(), char::from(piece.piece_type()));
                let piece_image = match &piece_set_name {
                    Some(piece_set) => {
                        let piece_svg_path = PathBuf::from("pieces").join(piece_set).join(format!("{piece_str}.svg"));
                        let piece_svg = nsvg::parse_str(
                            ASSETS_DIR
                                .get_file(piece_svg_path)
                                .ok_or(InvalidPositionImagePropertiesError::InvalidBuiltinPieceSet(piece_set.clone()))?
                                .contents_utf8()
                                .unwrap(),
                            nsvg::Units::Pixel,
                            96.,
                        )
                        .unwrap();
                        piece_svg.rasterize(piece_size as f32 / piece_svg.width()).unwrap()
                    }
                    None => {
                        if let PieceSet::Custom(hm) = &piece_set {
                            let piece_img = hm.get(&piece_str).ok_or(InvalidPositionImagePropertiesError::InvalidCustomPieceSet(piece_set.clone()))?;
                            nsvg::image::RgbaImage::from_vec(
                                piece_size as u32,
                                piece_size as u32,
                                imageops::resize(piece_img, piece_size as u32, piece_size as u32, imageops::FilterType::Nearest).to_vec(),
                            )
                            .unwrap()
                        } else {
                            panic!("the universe is malfunctioning");
                        }
                    }
                };
                for y in 0..piece_size {
                    for x in 0..piece_size {
                        let px = piece_image.get_pixel(x as u32, y as u32);
                        let (put_x, put_y) = ((sq_x + x) as u32, (sq_y + y) as u32);
                        if px.data[3] > 64 {
                            board_image.put_pixel(put_x, put_y, Rgba::from(px.data));
                        } else {
                            board_image.put_pixel(put_x, put_y, Rgba([sq_color.0, sq_color.1, sq_color.2, 255]));
                        }
                    }
                }
            } else {
                for y in 0..piece_size {
                    for x in 0..piece_size {
                        let (put_x, put_y) = ((sq_x + x) as u32, (sq_y + y) as u32);
                        board_image.put_pixel(put_x, put_y, Rgba([sq_color.0, sq_color.1, sq_color.2, 255]));
                    }
                }
            }
        }
    }
    Ok(board_image)
}
