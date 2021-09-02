use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct ImageResourceBlock<'a> {
    resource_id: u16,
    name: Cow<'a, [u8]>,
    resource_data: Cow<'a, [u8]>,
}

impl<'a> ImageResourceBlock<'a> {
    pub fn resource_id(&self) -> u16 {
        self.resource_id
    }
    pub fn name(&self) -> &[u8] {
        &self.name
    }
    pub fn resource_data(&self) -> &[u8] {
        &self.resource_data
    }
    fn into_static(self) -> ImageResourceBlock<'static> {
        let ImageResourceBlock {
            resource_id,
            name,
            resource_data,
        } = self;
        ImageResourceBlock {
            resource_id,
            name: Cow::Owned(name.into_owned()),
            resource_data: Cow::Owned(resource_data.into_owned()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ImageResources<'a>(Vec<ImageResourceBlock<'a>>);

impl<'a> ImageResources<'a> {
    pub fn data(&self) -> &[ImageResourceBlock] {
        &self.0
    }
    pub(crate) fn into_static(self) -> ImageResources<'static> {
        let ImageResources(list) = self;
        ImageResources(
            list.into_iter()
                .map(ImageResourceBlock::into_static)
                .collect(),
        )
    }
}

pub(crate) fn parse_image_resources(input: &[u8]) -> IResult<&[u8], ImageResources> {
    let (input, len) = be_u32(input)?;
    let mut resources = Vec::new();
    let mut blocks_input = &input[..len as usize];
    while !blocks_input.is_empty() {
        let (input, block) = parse_image_resource_block(blocks_input)?;
        resources.push(block);
        blocks_input = input;
    }
    Ok((&input[len as usize..], ImageResources(resources)))
}

fn parse_image_resource_block(input: &[u8]) -> IResult<&[u8], ImageResourceBlock> {
    let (input, _) = tag(b"8BIM")(input)?;
    let (input, resource_id) = be_u16(input)?;
    let (input, name_len) = be_u8(input)?;
    let name = &input[..name_len as usize];
    let input = &input[name_len as usize | 1..];
    let (input, data_len) = be_u32(input)?;
    Ok((
        &input[((data_len + 1) & !1) as usize..],
        ImageResourceBlock {
            resource_id,
            name: Cow::Borrowed(name),
            resource_data: Cow::Borrowed(&input[..data_len as usize]),
        },
    ))
}
