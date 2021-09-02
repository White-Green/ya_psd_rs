use nom::bytes::complete::tag;
use nom::combinator::{map_res, verify};
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ColorMode {
    Bitmap = 0,
    Grayscale = 1,
    Indexed = 2,
    RGB = 3,
    CMYK = 4,
    Multichannel = 7,
    Duotone = 8,
    Lab = 9,
}

impl ColorMode {
    fn from_u16(value: u16) -> Result<Self, u16> {
        match value {
            0 => Ok(ColorMode::Bitmap),
            1 => Ok(ColorMode::Grayscale),
            2 => Ok(ColorMode::Indexed),
            3 => Ok(ColorMode::RGB),
            4 => Ok(ColorMode::CMYK),
            7 => Ok(ColorMode::Multichannel),
            8 => Ok(ColorMode::Duotone),
            9 => Ok(ColorMode::Lab),
            _ => Err(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PsdHeader {
    version: u16,
    channels: u16,
    height: u32,
    width: u32,
    depth: u16,
    color_mode: ColorMode,
}

impl PsdHeader {
    pub fn version(&self) -> u16 {
        self.version
    }
    pub fn channels(&self) -> u16 {
        self.channels
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn depth(&self) -> u16 {
        self.depth
    }
    pub fn color_mode(&self) -> ColorMode {
        self.color_mode
    }
}

pub(crate) fn parse_header(input: &[u8]) -> IResult<&[u8], PsdHeader> {
    let (input, _) = tag(b"8BPS")(input)?;
    let (input, _) = verify(be_u16, |version| *version == 1)(input)?;
    let (input, _) = tag(&[0u8, 0, 0, 0, 0, 0])(input)?;
    let (input, channels) = verify(be_u16, |channels| (1..=56).contains(channels))(input)?;
    let (input, height) = verify(be_u32, |height| (1..=30_000).contains(height))(input)?;
    let (input, width) = verify(be_u32, |width| (1..=30_000).contains(width))(input)?;
    let (input, depth) = verify(be_u16, |depth| [1, 8, 16, 32].contains(depth))(input)?;
    let (input, color_mode) = map_res(be_u16, ColorMode::from_u16)(input)?;
    Ok((
        input,
        PsdHeader {
            version: 1,
            channels,
            height,
            width,
            depth,
            color_mode,
        },
    ))
}
