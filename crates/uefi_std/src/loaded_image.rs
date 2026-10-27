// SPDX-License-Identifier: MIT

use uefi::loaded_image::LoadedImage as UefiLoadedImage;

use crate::prelude::*;
use crate::proto::Protocol;

pub struct LoadedImage(pub &'static mut UefiLoadedImage);

impl Protocol<UefiLoadedImage> for LoadedImage {
    fn guid() -> Guid {
        UefiLoadedImage::GUID
    }

    fn new(inner: &'static mut UefiLoadedImage) -> Self {
        LoadedImage(inner)
    }
}
