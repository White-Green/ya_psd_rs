use crate::color_mode::{parse_color_mode, ColorModeData};
use crate::header::{parse_header, PsdHeader};
use crate::image_data::{parse_image_data, ImageData};
use crate::image_resource::{parse_image_resources, ImageResources};
use crate::layer_info::{parse_layer_and_mask_information, LayerAndMaskInformation};

pub mod color_mode;
pub mod header;
pub mod image_data;
pub mod image_resource;
pub mod layer_info;

#[derive(Debug, Eq, PartialEq)]
pub struct Psd<'a> {
    header: PsdHeader,
    color_mode: ColorModeData<'a>,
    image_resources: ImageResources<'a>,
    layer_information: LayerAndMaskInformation<'a>,
    image_data: ImageData<'a>,
}

impl<'a> Psd<'a> {
    pub fn header(&self) -> &PsdHeader {
        &self.header
    }
    pub fn color_mode(&self) -> &ColorModeData<'a> {
        &self.color_mode
    }
    pub fn image_resources(&self) -> &ImageResources<'a> {
        &self.image_resources
    }
    pub fn layer_information(&self) -> &LayerAndMaskInformation<'a> {
        &self.layer_information
    }
    pub fn image_data(&self) -> &ImageData<'a> {
        &self.image_data
    }
    pub fn into_static(self) -> Psd<'static> {
        let Psd { header, color_mode, image_resources, layer_information, image_data } = self;
        Psd {
            header,
            color_mode: color_mode.into_static(),
            image_resources: image_resources.into_static(),
            layer_information: layer_information.into_static(),
            image_data: image_data.into_static(),
        }
    }
}

pub fn parse_psd(input: &[u8]) -> Result<Psd, anyhow::Error> {
    let (input, header) = parse_header(input).unwrap();
    let (input, color_mode) = parse_color_mode(input, &header).unwrap();
    let (input, image_resources) = parse_image_resources(input).unwrap();
    let (input, layer_information) = parse_layer_and_mask_information(input).unwrap();
    let (_, image_data) = parse_image_data(input, &header).unwrap();
    Ok(Psd { header, color_mode, image_resources, layer_information, image_data })
}
