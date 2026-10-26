// SPDX-License-Identifier: MIT

use crate::prelude::*;

#[unsafe(no_mangle)]
pub unsafe extern "efiapi" fn efi_main(
    handle: Handle,
    system_table: &'static mut SystemTable,
) -> Status {
    unsafe extern "C" {
        fn main() -> Status;
    }

    crate::HANDLE = handle;
    crate::SYSTEM_TABLE = system_table;

    uefi_alloc::init(::core::mem::transmute(system_table));

    main()
}
