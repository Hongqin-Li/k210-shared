//! OV2640 camera handling
use crate::soc::dvp::DVP;
use crate::board::def::OV2640_ADDR;

/** Initial configuration for OV2640 chip */
static CONFIG: &[(u8, u8)] = &[
    (0xff, 0x01), // Bank: Sensor Address
    (0x12, 0x80), // COM7 SRST: Initiates system reset

    (0xff, 0x00), // Bank: DSP Address
    (0x2c, 0xff), // ??
    (0x2e, 0xdf), // ??

    (0xff, 0x01), // Bank: Sensor Address
    (0x3c, 0x32), // ??
    (0x11, 0x00),
    (0x09, 0x02),
    (0x04, 0x88),
    (0x13, 0xe5),
    (0x14, 0x48),
    (0x2c, 0x0c),
    (0x33, 0x78),
    (0x3a, 0x33),
    (0x3b, 0xfb),
    (0x3e, 0x00),
    (0x43, 0x11),
    (0x16, 0x10),
    (0x39, 0x92),
    (0x35, 0xda),
    (0x22, 0x1a),
    (0x37, 0xc3),
    (0x23, 0x00),
    (0x34, 0xc0),
    (0x36, 0x1a),
    (0x06, 0x88),
    (0x07, 0xc0),
    (0x0d, 0x87),
    (0x0e, 0x41),
    (0x4c, 0x00),
    (0x48, 0x00),
    (0x5b, 0x00),
    (0x42, 0x03),
    (0x4a, 0x81),
    (0x21, 0x99),
    (0x24, 0x40),
    (0x25, 0x38),
    (0x26, 0x82),
    (0x5c, 0x00),
    (0x63, 0x00),
    (0x46, 0x22),
    (0x0c, 0x3c),
    (0x61, 0x70),
    (0x62, 0x80),
    (0x7c, 0x05),
    (0x20, 0x80),
    (0x28, 0x30),
    (0x6c, 0x00),
    (0x6d, 0x80),
    (0x6e, 0x00),
    (0x70, 0x02),
    (0x71, 0x94),
    (0x73, 0xc1),
    (0x3d, 0x34),
    (0x5a, 0x57),
    (0x12, 0x40),
    (0x17, 0x11),
    (0x18, 0x43),
    (0x19, 0x00),
    (0x1a, 0x4b),
    (0x32, 0x09),
    (0x37, 0xc0),
    (0x4f, 0xca),
    (0x50, 0xa8),
    (0x5a, 0x23),
    (0x6d, 0x00),
    (0x3d, 0x38),

    (0xff, 0x00), // Bank: DSP Address
    (0xe5, 0x7f),
    (0xf9, 0xc0),
    (0x41, 0x24),
    (0xe0, 0x14),
    (0x76, 0xff),
    (0x33, 0xa0),
    (0x42, 0x20),
    (0x43, 0x18),
    (0x4c, 0x00),
    (0x87, 0xd5),
    (0x88, 0x3f),
    (0xd7, 0x03),
    (0xd9, 0x10),
    (0xd3, 0x82),
    (0xc8, 0x08),
    (0xc9, 0x80),
    (0x7c, 0x00),
    (0x7d, 0x00),
    (0x7c, 0x03),
    (0x7d, 0x48),
    (0x7d, 0x48),
    (0x7c, 0x08),
    (0x7d, 0x20),
    (0x7d, 0x10),
    (0x7d, 0x0e),
    (0x90, 0x00),
    (0x91, 0x0e),
    (0x91, 0x1a),
    (0x91, 0x31),
    (0x91, 0x5a),
    (0x91, 0x69),
    (0x91, 0x75),
    (0x91, 0x7e),
    (0x91, 0x88),
    (0x91, 0x8f),
    (0x91, 0x96),
    (0x91, 0xa3),
    (0x91, 0xaf),
    (0x91, 0xc4),
    (0x91, 0xd7),
    (0x91, 0xe8),
    (0x91, 0x20),
    (0x92, 0x00),
    (0x93, 0x06),
    (0x93, 0xe3),
    (0x93, 0x05),
    (0x93, 0x05),
    (0x93, 0x00),
    (0x93, 0x04),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x93, 0x00),
    (0x96, 0x00),
    (0x97, 0x08),
    (0x97, 0x19),
    (0x97, 0x02),
    (0x97, 0x0c),
    (0x97, 0x24),
    (0x97, 0x30),
    (0x97, 0x28),
    (0x97, 0x26),
    (0x97, 0x02),
    (0x97, 0x98),
    (0x97, 0x80),
    (0x97, 0x00),
    (0x97, 0x00),
    (0xc3, 0xed),
    (0xa4, 0x00),
    (0xa8, 0x00),
    (0xc5, 0x11),
    (0xc6, 0x51),
    (0xbf, 0x80),
    (0xc7, 0x10),
    (0xb6, 0x66),
    (0xb8, 0xa5),
    (0xb7, 0x64),
    (0xb9, 0x7c),
    (0xb3, 0xaf),
    (0xb4, 0x97),
    (0xb5, 0xff),
    (0xb0, 0xc5),
    (0xb1, 0x94),
    (0xb2, 0x0f),
    (0xc4, 0x5c),
    (0xc0, 0x64),
    (0xc1, 0x4b),
    (0x8c, 0x00),
    (0x86, 0x3d),
    (0x50, 0x00),
    (0x51, 0xc8),
    (0x52, 0x96),
    (0x53, 0x00),
    (0x54, 0x00),
    (0x55, 0x00),
    (0x5a, 0xc8),
    (0x5b, 0x96),
    (0x5c, 0x00),
    (0xd3, 0x02),
    (0xc3, 0xed),
    (0x7f, 0x00),
    (0xda, 0x08),
    (0xe5, 0x1f),
    (0xe1, 0x67),
    (0xe0, 0x00),
    (0xdd, 0x7f),
    (0x05, 0x00),

    (0xff, 0x00), // Bank: DSP Address
    (0xe0, 0x04),
    (0x5a, 0x50),
    (0x5b, 0x3c),
    (0x5c, 0x00),
    (0xe0, 0x00),
];

pub fn init(dvp: &DVP) {
    let (_manuf_id, _device_id) = read_id(dvp);
    // TODO: do something with the IDs (like check it against expected and fail if different?)
    // printf("manuf_id:0x%04x,device_id:0x%04x\n", v_manuf_id, v_device_id);
    for &(register, value) in CONFIG {
        dvp.sccb_send_data(OV2640_ADDR, register.into(), value.into());
    }
}

/** Return (manuf_id, device_id) */
pub fn read_id(dvp: &DVP) -> (u16, u16) {
    // 0xFF RA_DLMT - Register Bank Select: Sensor address
    dvp.sccb_send_data(OV2640_ADDR, 0xFF, 0x01);
    // 0x1C MIDH - Manufacturer ID Byte – High (Read only = 0x7F)
    // 0x1D MIDL - Manufacturer ID Byte – Low (Read only = 0xA2)
    let manuf_id = (u16::from(dvp.sccb_receive_data(OV2640_ADDR, 0x1C)) << 8) | u16::from(dvp.sccb_receive_data(OV2640_ADDR, 0x1D));
    // 0x0A PIDH - Product ID Number MSB (Read only)
    // 0x0B PIDL - Product ID Number LSB (Read only)
    let device_id = (u16::from(dvp.sccb_receive_data(OV2640_ADDR, 0x0A)) << 8) | u16::from(dvp.sccb_receive_data(OV2640_ADDR, 0x0B));
    (manuf_id, device_id)
}
