use std::borrow::Cow;
use std::convert::TryInto;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map_res;
use nom::number::complete::{be_i16, be_i32, be_u16, be_u32, be_u8};
use nom::IResult;
use once_cell::sync::OnceCell;

#[derive(Debug, Eq, PartialEq)]
pub struct LayerAndMaskInformation<'a> {
    layer_info: Vec<LayerTreeNode<'a>>,
    global_layer_mask_info: Cow<'a, [u8]>,
    additional_layer_information: Cow<'a, [u8]>,
}

impl<'a> LayerAndMaskInformation<'a> {
    pub fn layer_info(&self) -> &[LayerTreeNode<'a>] {
        &self.layer_info
    }
    pub fn global_layer_mask_info(&self) -> &[u8] {
        &self.global_layer_mask_info
    }
    pub fn additional_layer_information(&self) -> &[u8] {
        &self.additional_layer_information
    }
    pub(crate) fn into_static(self) -> LayerAndMaskInformation<'static> {
        let LayerAndMaskInformation { layer_info, global_layer_mask_info, additional_layer_information } = self;
        LayerAndMaskInformation {
            layer_info: layer_info.into_iter().map(LayerTreeNode::into_static).collect(),
            global_layer_mask_info: Cow::Owned(global_layer_mask_info.into_owned()),
            additional_layer_information: Cow::Owned(additional_layer_information.into_owned()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LayerRecord<'a> {
    layer_top: i32,
    layer_left: i32,
    layer_bottom: i32,
    layer_right: i32,
    channel_info: Vec<ChannelInfo<'a>>,
    transparency_mask: Option<ChannelInfo<'a>>,
    user_supplied_layer_mask: Option<ChannelInfo<'a>>,
    real_user_supplied_layer_mask: Option<ChannelInfo<'a>>,
    blend_mode: BlendMode,
    opacity: u8,
    clipping: Clipping,
    flags: LayerRecordFlags,
    layer_mask_data: Option<LayerMaskData>,
    layer_blending_ranges_data: Cow<'a, [u8]>,
    layer_name: Cow<'a, [u8]>,
    additional_layer_info: Vec<AdditionalLayerInformation<'a>>,
}

impl<'a> LayerRecord<'a> {
    pub fn layer_top(&self) -> i32 {
        self.layer_top
    }
    pub fn layer_left(&self) -> i32 {
        self.layer_left
    }
    pub fn layer_bottom(&self) -> i32 {
        self.layer_bottom
    }
    pub fn layer_right(&self) -> i32 {
        self.layer_right
    }
    pub fn channel_info(&self) -> &[ChannelInfo<'a>] {
        &self.channel_info
    }
    pub fn transparency_mask(&self) -> Option<&ChannelInfo<'a>> {
        self.transparency_mask.as_ref()
    }
    pub fn user_supplied_layer_mask(&self) -> Option<&ChannelInfo<'a>> {
        self.user_supplied_layer_mask.as_ref()
    }
    pub fn real_user_supplied_layer_mask(&self) -> Option<&ChannelInfo<'a>> {
        self.real_user_supplied_layer_mask.as_ref()
    }
    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
    pub fn opacity(&self) -> u8 {
        self.opacity
    }
    pub fn clipping(&self) -> Clipping {
        self.clipping
    }
    pub fn flags(&self) -> LayerRecordFlags {
        self.flags
    }
    pub fn layer_mask_data(&self) -> Option<&LayerMaskData> {
        self.layer_mask_data.as_ref()
    }
    pub fn layer_blending_ranges_data(&self) -> &[u8] {
        &self.layer_blending_ranges_data
    }
    pub fn layer_name(&self) -> &[u8] {
        &self.layer_name
    }
    pub fn additional_layer_info(&self) -> &[AdditionalLayerInformation] {
        &self.additional_layer_info
    }
    fn into_static(self) -> LayerRecord<'static> {
        let LayerRecord {
            layer_top,
            layer_left,
            layer_bottom,
            layer_right,
            channel_info,
            transparency_mask,
            user_supplied_layer_mask,
            real_user_supplied_layer_mask,
            blend_mode,
            opacity,
            clipping,
            flags,
            layer_mask_data,
            layer_blending_ranges_data,
            layer_name,
            additional_layer_info,
        } = self;
        LayerRecord {
            layer_top,
            layer_left,
            layer_bottom,
            layer_right,
            channel_info: channel_info.into_iter().map(ChannelInfo::into_static).collect(),
            transparency_mask: transparency_mask.map(ChannelInfo::into_static),
            user_supplied_layer_mask: user_supplied_layer_mask.map(ChannelInfo::into_static),
            real_user_supplied_layer_mask: real_user_supplied_layer_mask.map(ChannelInfo::into_static),
            blend_mode,
            opacity,
            clipping,
            flags,
            layer_mask_data,
            layer_blending_ranges_data: Cow::Owned(layer_blending_ranges_data.into_owned()),
            layer_name: Cow::Owned(layer_name.into_owned()),
            additional_layer_info: additional_layer_info.into_iter().map(AdditionalLayerInformation::into_static).collect(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LayerMaskData {
    layer_mask_top: i32,
    layer_mask_left: i32,
    layer_mask_bottom: i32,
    layer_mask_right: i32,
    default_color: u8,
    flags: LayerMaskFlags,
    optional: Option<LayerMaskOptionalData>,
}

impl LayerMaskData {
    pub fn layer_mask_top(&self) -> i32 {
        self.layer_mask_top
    }
    pub fn layer_mask_left(&self) -> i32 {
        self.layer_mask_left
    }
    pub fn layer_mask_bottom(&self) -> i32 {
        self.layer_mask_bottom
    }
    pub fn layer_mask_right(&self) -> i32 {
        self.layer_mask_right
    }
    pub fn default_color(&self) -> u8 {
        self.default_color
    }
    pub fn flags(&self) -> LayerMaskFlags {
        self.flags
    }
    pub fn optional(&self) -> Option<&LayerMaskOptionalData> {
        self.optional.as_ref()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LayerMaskOptionalData {
    real_flags: LayerMaskFlags,
    real_user_mask_background: u8,
    layer_mask_top: i32,
    layer_mask_left: i32,
    layer_mask_bottom: i32,
    layer_mask_right: i32,
}

impl LayerMaskOptionalData {
    pub fn real_flags(&self) -> LayerMaskFlags {
        self.real_flags
    }
    pub fn real_user_mask_background(&self) -> u8 {
        self.real_user_mask_background
    }
    pub fn layer_mask_top(&self) -> i32 {
        self.layer_mask_top
    }
    pub fn layer_mask_left(&self) -> i32 {
        self.layer_mask_left
    }
    pub fn layer_mask_bottom(&self) -> i32 {
        self.layer_mask_bottom
    }
    pub fn layer_mask_right(&self) -> i32 {
        self.layer_mask_right
    }
}

bitflags::bitflags! {
    pub struct LayerMaskFlags : u8 {
        const POSITION_RELATIVE_TO_LAYER = 0b0000_0001;
        const LAYER_MASK_DISABLED = 0b0000_0010;
        const INVERT_LAYER_MASK_WHEN_BLENDING = 0b0000_0100;
        const INDICATES_THAT_THE_USER_MASK_ACTUALLY_CAME_FROM_RENDERING_OTHER_DATA = 0b0000_1000;
        const INDICATES_THAT_THE_USER_AND_OR_VECTOR_MASKS_HAVE_PARAMEWTERS_APPLIED_TO_THEM = 0b0001_0000;
    }
}

bitflags::bitflags! {
    pub struct LayerRecordFlags : u8 {
        const TRANSPARENCY_PROTECTED = 0b0000_0001;
        const VISIBLE = 0b0000_0010;
        const OBSOLETE = 0b0000_0100;
        const PHOTOSHOP_5_0_LATER = 0b0000_1000;
        const PIXEL_DATA_IRRELEVANT_TO_APPEARANCE_OF_DOCUMENT = 0b0001_0000;
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SectionDividerType {
    BoundingSectionDivider,
    OpenFolder,
    ClosedFolder,
    AnyOtherType,
}

impl SectionDividerType {
    fn from_u32(value: u32) -> Result<Self, u32> {
        match value {
            0 => Ok(SectionDividerType::AnyOtherType),
            1 => Ok(SectionDividerType::OpenFolder),
            2 => Ok(SectionDividerType::ClosedFolder),
            3 => Ok(SectionDividerType::BoundingSectionDivider),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SectionDividerSubType {
    Normal,
    SceneGroup,
}

impl SectionDividerSubType {
    fn from_u32(value: u32) -> Result<Self, u32> {
        match value {
            0 => Ok(SectionDividerSubType::Normal),
            1 => Ok(SectionDividerSubType::SceneGroup),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AdditionalLayerInformation<'a> {
    SectionDivider { section_divider_type: SectionDividerType, key: Option<BlendMode>, sub_type: Option<SectionDividerSubType> },
    Unknown { key: Cow<'a, [u8; 4]>, data: Cow<'a, [u8]> },
}

impl<'a> AdditionalLayerInformation<'a> {
    fn into_static(self) -> AdditionalLayerInformation<'static> {
        match self {
            AdditionalLayerInformation::SectionDivider { section_divider_type, key, sub_type } => AdditionalLayerInformation::SectionDivider { section_divider_type, key, sub_type },
            AdditionalLayerInformation::Unknown { key, data } => AdditionalLayerInformation::Unknown { key: Cow::Owned(key.into_owned()), data: Cow::Owned(data.into_owned()) },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ChannelInfo<'a> {
    channel_id: i16,
    channel_data_length: u32,
    channel_data_width: u32,
    channel_data_height: u32,
    compression: ImageCompression,
    data: Cow<'a, [u8]>,
    raw_data: OnceCell<Cow<'a, [u8]>>,
}

impl<'a> ChannelInfo<'a> {
    pub fn channel_id(&self) -> i16 {
        self.channel_id
    }
    pub fn channel_data_length(&self) -> u32 {
        self.channel_data_length
    }
    pub fn compression(&self) -> ImageCompression {
        self.compression
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn raw_data(&self) -> &[u8] {
        self.raw_data.get_or_init(|| match self.compression {
            ImageCompression::Raw => self.data.clone(),
            ImageCompression::RLE => {
                let mut result = Vec::with_capacity(self.channel_data_width as usize * self.channel_data_height as usize);
                let mut data = &self.data[self.channel_data_height as usize * 2..];
                while !data.is_empty() {
                    let (&len, follow) = data.split_first().unwrap();
                    match len as i8 {
                        len @ 0..=127 => {
                            let len = len as usize;
                            result.extend(&follow[..len + 1]);
                            data = &follow[len + 1..];
                        }
                        len @ -127..=-1 => {
                            for _ in 0..-len as usize + 1 {
                                result.push(follow[0]);
                            }
                            data = &follow[1..];
                        }
                        -128 => {
                            println!("may be error");
                        }
                    }
                }
                Cow::Owned(result)
            }
            ImageCompression::ZipWithoutPrediction | ImageCompression::ZipWithPrediction => {
                panic!("Zip compression is not supported")
            }
        })
    }
    fn into_static(self) -> ChannelInfo<'static> {
        let _ = self.raw_data();
        let ChannelInfo {
            channel_id,
            channel_data_length,
            channel_data_width,
            channel_data_height,
            compression,
            data,
            raw_data,
        } = self;
        let raw_data = raw_data.into_inner().unwrap();
        let raw_data_cell = OnceCell::<Cow<'static, [u8]>>::new();
        raw_data_cell.set(Cow::Owned(raw_data.into_owned())).unwrap();
        ChannelInfo {
            channel_id,
            channel_data_length,
            channel_data_width,
            channel_data_height,
            compression,
            data: Cow::Owned(data.into_owned()),
            raw_data: raw_data_cell,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlendMode {
    Passthrough,
    Normal,
    Dissolve,
    Darken,
    Multiply,
    Colorburn,
    Linearburn,
    Darkercolor,
    Lighten,
    Screen,
    Colordodge,
    Lineardodge,
    Lightercolor,
    Overlay,
    Softlight,
    Hardlight,
    Vividlight,
    Linearlight,
    Pinlight,
    Hardmix,
    Difference,
    Exclusion,
    Subtract,
    Divide,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    fn try_from(input: &[u8]) -> Result<Self, &[u8]> {
        match input {
            b"pass" => Ok(BlendMode::Passthrough),
            b"norm" => Ok(BlendMode::Normal),
            b"diss" => Ok(BlendMode::Dissolve),
            b"dark" => Ok(BlendMode::Darken),
            b"mul " => Ok(BlendMode::Multiply),
            b"idiv" => Ok(BlendMode::Colorburn),
            b"lbrn" => Ok(BlendMode::Linearburn),
            b"dkCl" => Ok(BlendMode::Darkercolor),
            b"lite" => Ok(BlendMode::Lighten),
            b"scrn" => Ok(BlendMode::Screen),
            b"div " => Ok(BlendMode::Colordodge),
            b"lddg" => Ok(BlendMode::Lineardodge),
            b"lgCl" => Ok(BlendMode::Lightercolor),
            b"over" => Ok(BlendMode::Overlay),
            b"sLit" => Ok(BlendMode::Softlight),
            b"hLit" => Ok(BlendMode::Hardlight),
            b"vLit" => Ok(BlendMode::Vividlight),
            b"lLit" => Ok(BlendMode::Linearlight),
            b"pLit" => Ok(BlendMode::Pinlight),
            b"hMix" => Ok(BlendMode::Hardmix),
            b"diff" => Ok(BlendMode::Difference),
            b"smud" => Ok(BlendMode::Exclusion),
            b"fsub" => Ok(BlendMode::Subtract),
            b"fdiv" => Ok(BlendMode::Divide),
            b"hue " => Ok(BlendMode::Hue),
            b"sat " => Ok(BlendMode::Saturation),
            b"colr" => Ok(BlendMode::Color),
            b"lum " => Ok(BlendMode::Luminosity),
            _ => Err(input),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Clipping {
    Base,
    NonBase,
}

impl Clipping {
    fn try_from(input: u8) -> Result<Self, u8> {
        match input {
            0 => Ok(Clipping::Base),
            1 => Ok(Clipping::NonBase),
            _ => Err(input),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageCompression {
    Raw,
    RLE,
    ZipWithoutPrediction,
    ZipWithPrediction,
}

impl ImageCompression {
    pub(crate) fn from_u16(value: u16) -> Result<Self, u16> {
        match value {
            0 => Ok(ImageCompression::Raw),
            1 => Ok(ImageCompression::RLE),
            2 => Ok(ImageCompression::ZipWithoutPrediction),
            3 => Ok(ImageCompression::ZipWithPrediction),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ChannelImageData<'a> {
    compression: ImageCompression,
    data: &'a [u8],
}

impl<'a> ChannelImageData<'a> {
    pub fn compression(&self) -> ImageCompression {
        self.compression
    }
    pub fn data(&self) -> &[u8] {
        self.data
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LayerTreeNode<'a> {
    Leaf(LayerRecord<'a>),
    Node { folder: LayerRecord<'a>, children: Vec<LayerTreeNode<'a>> },
}

impl<'a> LayerTreeNode<'a> {
    fn into_static(self) -> LayerTreeNode<'static> {
        match self {
            LayerTreeNode::Leaf(record) => LayerTreeNode::Leaf(record.into_static()),
            LayerTreeNode::Node { folder, children } => LayerTreeNode::Node {
                folder: folder.into_static(),
                children: children.into_iter().map(LayerTreeNode::into_static).collect(),
            },
        }
    }
}

pub(crate) fn parse_layer_and_mask_information(input: &[u8]) -> IResult<&[u8], LayerAndMaskInformation> {
    let (input, len) = be_u32(input)?;
    let (follow, input) = take(len)(input)?;
    let (input, layer_info) = parse_layer_info(input)?;
    let (input, global_layer_mask_info) = parse_global_layer_mask_info(input)?;

    Ok((
        follow,
        LayerAndMaskInformation {
            layer_info,
            global_layer_mask_info: Cow::Borrowed(global_layer_mask_info),
            additional_layer_information: Cow::Borrowed(input),
        },
    ))
}

fn parse_layer_info(input: &[u8]) -> IResult<&[u8], Vec<LayerTreeNode>> {
    let (input, len) = be_u32(input)?;
    let (follow, input) = take(len)(input)?;
    let (mut input, layer_count) = be_i16(input)?;
    let mut layer_records = Vec::new();
    for _ in 0..layer_count.abs() {
        let (i, layer_record) = parse_layer_record(input)?;
        layer_records.push(layer_record);
        input = i;
    }
    let (_input, _) = parse_channel_image_data(input, &mut layer_records)?;
    sort_channel_data(&mut layer_records);
    let layers = into_layer_tree(layer_records);
    Ok((follow, layers))
}

fn sort_channel_data(layer_records: &mut [LayerRecord]) {
    for layer_record in layer_records {
        let (mut channel_info, mut masks): (Vec<_>, Vec<_>) = layer_record.channel_info.drain(..).partition(|channel| channel.channel_id >= 0);
        channel_info.sort_by_key(|ch| ch.channel_id);
        let transparency_mask = masks.iter().position(|ch| ch.channel_id == -1).map(|index| masks.swap_remove(index));
        let mut user_supplied_layer_mask = masks.iter().position(|ch| ch.channel_id == -2).map(|index| masks.swap_remove(index));
        let mut real_user_supplied_layer_mask = masks.iter().position(|ch| ch.channel_id == -3).map(|index| masks.swap_remove(index));
        channel_info.extend(masks);
        if let Some(LayerMaskData {
            layer_mask_top,
            layer_mask_left,
            layer_mask_bottom,
            layer_mask_right,
            optional,
            ..
        }) = &layer_record.layer_mask_data
        {
            if let Some(ChannelInfo { channel_data_width, channel_data_height, .. }) = &mut user_supplied_layer_mask {
                *channel_data_width = (*layer_mask_right - *layer_mask_left) as u32;
                *channel_data_height = (*layer_mask_bottom - *layer_mask_top) as u32;
            } else {
                eprintln!("may be error");
            }
            if let Some(LayerMaskOptionalData { layer_mask_top, layer_mask_left, layer_mask_bottom, layer_mask_right, .. }) = optional {
                if let Some(ChannelInfo { channel_data_width, channel_data_height, .. }) = &mut real_user_supplied_layer_mask {
                    *channel_data_width = (*layer_mask_right - *layer_mask_left) as u32;
                    *channel_data_height = (*layer_mask_bottom - *layer_mask_top) as u32;
                } else {
                    eprintln!("may be error");
                }
            } else if real_user_supplied_layer_mask.is_some() {
                eprintln!("may be error");
            }
        } else if user_supplied_layer_mask.is_some() {
            eprintln!("may be error");
        }
        layer_record.channel_info = channel_info;
        layer_record.transparency_mask = transparency_mask;
        layer_record.user_supplied_layer_mask = user_supplied_layer_mask;
        layer_record.real_user_supplied_layer_mask = real_user_supplied_layer_mask;
    }
}

fn into_layer_tree(layers: Vec<LayerRecord>) -> Vec<LayerTreeNode> {
    let mut stack = vec![Vec::new()];
    enum SectionDividerTypeInner {
        Start,
        End,
    }
    for layer in layers {
        let divider = layer.additional_layer_info().iter().find_map(|info| match info {
            AdditionalLayerInformation::SectionDivider { section_divider_type, .. } => match section_divider_type {
                SectionDividerType::BoundingSectionDivider => Some(SectionDividerTypeInner::Start),
                SectionDividerType::OpenFolder | SectionDividerType::ClosedFolder => Some(SectionDividerTypeInner::End),
                SectionDividerType::AnyOtherType => {
                    eprintln!("may be error");
                    None
                }
            },
            _ => None,
        });
        match divider {
            Some(SectionDividerTypeInner::Start) => stack.push(Vec::new()),
            Some(SectionDividerTypeInner::End) => {
                let mut layers = stack.pop().expect("invalid layer structure");
                layers.reverse();
                stack.last_mut().expect("invalid layer structure").push(LayerTreeNode::Node { folder: layer, children: layers });
            }
            None => stack.last_mut().expect("invalid layer structure").push(LayerTreeNode::Leaf(layer)),
        }
    }
    let [mut list]: [_; 1] = stack.try_into().expect("invalid layer structure");
    list.reverse();
    list
}

fn parse_channel_image_data<'a, 'b>(mut input: &'a [u8], layer_records: &'b mut [LayerRecord<'a>]) -> IResult<&'a [u8], ()> {
    for layer_record in layer_records {
        for channel_info in &mut layer_record.channel_info {
            let len = channel_info.channel_data_length();
            let (i, data) = take(len)(input)?;
            let (data, compression) = map_res(be_u16, ImageCompression::from_u16)(data)?;
            channel_info.compression = compression;
            channel_info.data = Cow::Borrowed(data);
            input = i;
        }
    }
    Ok((input, ()))
}

fn parse_layer_record(input: &[u8]) -> IResult<&[u8], LayerRecord> {
    let (input, layer_top) = be_i32(input)?;
    let (input, layer_left) = be_i32(input)?;
    let (input, layer_bottom) = be_i32(input)?;
    let (input, layer_right) = be_i32(input)?;
    let (mut input, channels) = be_u16(input)?;
    let mut channel_info = Vec::new();
    for _ in 0..channels {
        let (i, channel_id) = be_i16(input)?;
        let (i, channel_data_length) = be_u32(i)?;
        channel_info.push(ChannelInfo {
            channel_id,
            channel_data_length,
            channel_data_width: (layer_right - layer_left) as u32,
            channel_data_height: (layer_bottom - layer_top) as u32,
            compression: ImageCompression::Raw,
            data: Cow::Borrowed(&i[..0]),
            raw_data: OnceCell::new(),
        });
        input = i;
    }
    let (input, _) = tag(b"8BIM")(input)?;
    let (input, blend_mode) = map_res(take(4usize), BlendMode::try_from)(input)?;
    let (input, opacity) = be_u8(input)?;
    let (input, clipping) = map_res(be_u8, Clipping::try_from)(input)?;
    let (input, flags) = map_res(be_u8, |flags| LayerRecordFlags::from_bits(flags).ok_or(flags))(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, len) = be_u32(input)?;
    let (follow, input) = take(len)(input)?;
    let (input, layer_mask_data_len) = be_u32(input)?;
    let (input, layer_mask_data) = take(layer_mask_data_len)(input)?;
    let (_, layer_mask_data) = parse_layer_mask_data(layer_mask_data)?;
    let (input, layer_blending_ranges_len) = be_u32(input)?;
    let (input, layer_blending_ranges_data) = take(layer_blending_ranges_len)(input)?;
    let (input, layer_name_len) = be_u8(input)?;
    let (input, layer_name) = take(layer_name_len)(input)?;
    let mut input = &input[3 - (layer_name_len as usize & 3)..];
    let mut additional_layer_info = Vec::new();
    while !input.is_empty() {
        let (i, _) = alt((tag(b"8BIM"), tag(b"8B64")))(input)?;
        let (i, key) = take(4usize)(i)?;
        let (i, len) = be_u32(i)?;
        let (i, data) = take(len as usize)(i)?;
        let (follow, info) = parse_additional_layer_info(key.try_into().unwrap(), data)?;
        assert_eq!(follow.len(), 0);
        additional_layer_info.push(info);
        input = i;
    }
    Ok((
        follow,
        LayerRecord {
            layer_top,
            layer_left,
            layer_bottom,
            layer_right,
            channel_info,
            transparency_mask: None,
            user_supplied_layer_mask: None,
            real_user_supplied_layer_mask: None,
            blend_mode,
            opacity,
            clipping,
            flags,
            layer_mask_data,
            layer_blending_ranges_data: Cow::Borrowed(layer_blending_ranges_data),
            layer_name: Cow::Borrowed(layer_name),
            additional_layer_info,
        },
    ))
}

fn parse_layer_mask_data(input: &[u8]) -> IResult<&[u8], Option<LayerMaskData>> {
    if input.is_empty() {
        return Ok((input, None));
    }
    let (input, layer_mask_top) = be_i32(input)?;
    let (input, layer_mask_left) = be_i32(input)?;
    let (input, layer_mask_bottom) = be_i32(input)?;
    let (input, layer_mask_right) = be_i32(input)?;
    let (input, default_color) = be_u8(input)?;
    let (input, flags) = map_res(be_u8, |flags| LayerMaskFlags::from_bits(flags).ok_or(flags))(input)?;
    if input.len() == 2 {
        return Ok((
            &input[..0],
            Some(LayerMaskData {
                layer_mask_top,
                layer_mask_left,
                layer_mask_bottom,
                layer_mask_right,
                default_color,
                flags,
                optional: None,
            }),
        ));
    }
    let (input, real_flags) = map_res(be_u8, |flags| LayerMaskFlags::from_bits(flags).ok_or(flags))(input)?;
    let (input, real_user_mask_background) = be_u8(input)?;
    let (input, mask_top) = be_i32(input)?;
    let (input, mask_left) = be_i32(input)?;
    let (input, mask_bottom) = be_i32(input)?;
    let (input, mask_right) = be_i32(input)?;
    Ok((
        input,
        Some(LayerMaskData {
            layer_mask_top,
            layer_mask_left,
            layer_mask_bottom,
            layer_mask_right,
            default_color,
            flags,
            optional: Some(LayerMaskOptionalData {
                real_flags,
                real_user_mask_background,
                layer_mask_top: mask_top,
                layer_mask_left: mask_left,
                layer_mask_bottom: mask_bottom,
                layer_mask_right: mask_right,
            }),
        }),
    ))
}

fn parse_global_layer_mask_info(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, len) = be_u32(input)?;
    take(len)(input)
}

fn parse_additional_layer_info<'a>(key: &'a [u8; 4], data: &'a [u8]) -> IResult<&'a [u8], AdditionalLayerInformation<'a>> {
    match key {
        b"lsct" => {
            let (data, section_type) = map_res(be_u32, SectionDividerType::from_u32)(data)?;
            if data.is_empty() {
                return Ok((data, AdditionalLayerInformation::SectionDivider { section_divider_type: section_type, key: None, sub_type: None }));
            }
            let (data, _) = tag(b"8BIM")(data)?;
            let (data, blend_mode) = map_res(take(4usize), BlendMode::try_from)(data)?;
            if data.is_empty() {
                return Ok((
                    data,
                    AdditionalLayerInformation::SectionDivider {
                        section_divider_type: section_type,
                        key: Some(blend_mode),
                        sub_type: None,
                    },
                ));
            }
            let (data, sub_type) = map_res(be_u32, SectionDividerSubType::from_u32)(data)?;
            Ok((
                data,
                AdditionalLayerInformation::SectionDivider {
                    section_divider_type: section_type,
                    key: Some(blend_mode),
                    sub_type: Some(sub_type),
                },
            ))
        }
        _ => Ok((&data[..0], AdditionalLayerInformation::Unknown { key: Cow::Borrowed(key), data: Cow::Borrowed(data) })),
    }
}
