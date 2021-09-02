use std::borrow::Cow;

use nom::bytes::complete::take;
use nom::combinator::verify;
use nom::number::complete::be_u32;
use nom::IResult;

use crate::header::{ColorMode, PsdHeader};

#[derive(Debug, PartialEq, Eq)]
pub struct ColorModeData<'a>(Cow<'a, [u8]>);

impl<'a> ColorModeData<'a> {
    pub fn data(&self) -> &[u8] {
        &self.0
    }
    pub(crate) fn into_static(self) -> ColorModeData<'static> {
        let ColorModeData(data) = self;
        ColorModeData(Cow::Owned(data.into_owned()))
    }
}

pub(crate) fn parse_color_mode<'a, 'b>(
    input: &'a [u8],
    header: &'b PsdHeader,
) -> IResult<&'a [u8], ColorModeData<'a>> {
    let (input, len) = match header.color_mode() {
        ColorMode::Indexed => verify(be_u32, |len| *len == 768)(input)?,
        ColorMode::Duotone => be_u32(input)?,
        _ => verify(be_u32, |len| *len == 0)(input)?,
    };
    let (input, data) = take(len)(input)?;
    Ok((input, ColorModeData(Cow::Borrowed(data))))
}
