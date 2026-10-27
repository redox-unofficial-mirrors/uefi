// SPDX-License-Identifier: MIT

use uefi::pointer::SimplePointer;

use crate::prelude::*;
use crate::proto::Protocol;

pub struct Pointer(pub &'static mut SimplePointer);

impl Protocol<SimplePointer> for Pointer {
    fn guid() -> Guid {
        SimplePointer::GUID
    }

    fn new(inner: &'static mut SimplePointer) -> Self {
        Pointer(inner)
    }
}
