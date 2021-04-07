#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GUID(pub(crate) uefi_core::guid::GUID);

impl GUID {
    pub const fn new(guid: (u32, u16, u16, [u8; 8])) -> GUID {
        GUID(uefi_core::guid::GUID(guid.0, guid.1, guid.2, guid.3))
    }
}

pub const ACPI_20: GUID =
    GUID::new((0x8868e871, 0xe4f1, 0x11d3, [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81]));

pub const ACPI: GUID =
    GUID::new((0xeb9d2d30, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]));

pub const SAL_SYSTEM: GUID =
    GUID::new((0xeb9d2d32, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]));

pub const SMBIOS: GUID =
    GUID::new((0xeb9d2d31, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]));

pub const SMBIOS3: GUID =
    GUID::new((0xf2fd1544, 0x9794, 0x4a2c, [0x99, 0x2e, 0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94]));

pub const MPS: GUID =
    GUID::new((0xeb9d2d2f, 0x2d88, 0x11d3, [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]));

pub const JSON_CONFIG_DATA: GUID =
    GUID::new((0x87367f87, 0x1119, 0x41ce, [0xaa, 0xec, 0x8b, 0xe0, 0x11, 0x1f, 0x55, 0x8a]));

pub const JSON_CAPSULE_DATA: GUID =
    GUID::new((0x35e7a725, 0x8dd2, 0x4cac, [0x80, 0x11, 0x33, 0xcd, 0xa8, 0x10, 0x90, 0x56]));

pub const JSON_CAPSULE_RESULT: GUID =
    GUID::new((0xdbc461c3, 0xb3de, 0x422a, [0xb9, 0xb4, 0x98, 0x86, 0xfd, 0x49, 0xa1, 0xe5]));

pub const RT_PROPERTIES: GUID =
    GUID::new((0xeb66918a, 0x7eef, 0x402a, [0x84, 0x2e, 0x93, 0x1d, 0x21, 0xc3, 0x8a, 0xe9]));

pub const MEMORY_ATTRIBUTES: GUID =
    GUID::new((0xdcfa911d, 0x26eb, 0x469f, [0xa2, 0x20, 0x38, 0xb7, 0xdc, 0x46, 0x12, 0x20]));

pub const SIMPLE_FILE_SYSTEM_PROTOCOL: GUID =
    GUID::new((0x964e5b22, 0x6459, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]));

pub const FILE_INFO: GUID =
    GUID::new((0x09576e92, 0x6d3f, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]));

pub const FILE_SYSTEM_INFO: GUID =
    GUID::new((0x09576e93, 0x6d3f, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]));