use std::borrow::Cow;

use nom::combinator::map_res;
use nom::number::complete::be_u16;
use nom::IResult;
use once_cell::sync::OnceCell;

use crate::header::PsdHeader;
use crate::layer_info::ImageCompression;

#[derive(Debug, Eq, PartialEq)]
pub struct ImageData<'a> {
    compression: ImageCompression,
    data: Cow<'a, [u8]>,
    raw_data: OnceCell<Vec<Cow<'a, [u8]>>>,
    width: u32,
    height: u32,
    channels: u16,
}

impl<'a> ImageData<'a> {
    pub fn compression(&self) -> ImageCompression {
        self.compression
    }
    pub fn raw_data(&self) -> &[Cow<'a, [u8]>] {
        self.raw_data.get_or_init(|| {
            let mut list = Vec::with_capacity(self.channels as usize);
            match self.compression {
                ImageCompression::Raw => {
                    let len_one_channel = self.height as usize * self.width as usize;
                    let mut data = match self.data {
                        Cow::Borrowed(data) => data,
                        Cow::Owned(_) => unreachable!(),
                    };
                    // let mut data = self.data.deref();
                    while !data.is_empty() {
                        let (one_channel, follow) = data.split_at(len_one_channel);
                        list.push(Cow::Borrowed(one_channel));
                        data = follow;
                    }
                }
                ImageCompression::RLE => {
                    let mut data = &self.data[self.height as usize * self.channels as usize * 2..];
                    for _ in 0..self.channels {
                        let mut data_one_channel = Vec::with_capacity(self.width as usize * self.height as usize);
                        while data_one_channel.len() < self.width as usize * self.height as usize {
                            let (&len, follow) = data.split_first().unwrap();
                            match len as i8 {
                                len @ 0..=127 => {
                                    let len = len as usize;
                                    data_one_channel.extend(&follow[..len + 1]);
                                    data = &follow[len + 1..];
                                }
                                len @ -127..=-1 => {
                                    for _ in 0..-len as usize + 1 {
                                        data_one_channel.push(follow[0]);
                                    }
                                    data = &follow[1..];
                                }
                                -128 => {
                                    eprintln!("may be error");
                                }
                            }
                        }
                        assert_eq!(data_one_channel.len(), self.width as usize * self.height as usize);
                        list.push(Cow::Owned(data_one_channel));
                    }
                }
                ImageCompression::ZipWithoutPrediction | ImageCompression::ZipWithPrediction => {
                    panic!("Zip compression is not supported");
                }
            }
            list
        })
    }
    pub(crate) fn into_static(self) -> ImageData<'static> {
        let _ = self.raw_data();
        let ImageData { compression, data, raw_data, width, height, channels } = self;
        let raw_data = raw_data.into_inner().unwrap();
        let raw_data_cell = OnceCell::new();
        raw_data_cell.set(raw_data.into_iter().map(Cow::into_owned).map(Cow::Owned).collect()).unwrap();
        ImageData {
            compression,
            data: Cow::Owned(data.into_owned()),
            raw_data: raw_data_cell,
            width,
            height,
            channels,
        }
    }
}

pub(crate) fn parse_image_data<'a, 'b>(input: &'a [u8], header: &'b PsdHeader) -> IResult<&'a [u8], ImageData<'a>> {
    let (input, compression) = map_res(be_u16, ImageCompression::from_u16)(input)?;
    Ok((
        &input[..0],
        ImageData {
            compression,
            data: Cow::Borrowed(input),
            raw_data: OnceCell::new(),
            width: header.width(),
            height: header.height(),
            channels: header.channels(),
        },
    ))
}
