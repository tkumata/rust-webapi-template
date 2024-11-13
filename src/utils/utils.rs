use std::net::IpAddr;

pub struct ConvertedRgb {
    pub r: String,
    pub g: String,
    pub b: String
}

pub fn to_hex(value: i32) -> String {
    let value: i32 = if value > 255 {
        255
    } else if value < 0 {
        0
    } else {
        value
    };

    format!("{:02X}", value)
}

pub fn to_rgb_hex(r: i32, g: i32, b: i32) -> ConvertedRgb {
    let rgb: ConvertedRgb = ConvertedRgb {
        r: to_hex(r).to_string(),
        g: to_hex(g).to_string(),
        b: to_hex(b).to_string()
    };

    rgb
}

pub fn to_subnetmask(bit_length: i32) -> IpAddr {
    let bits: u32 = (!0) << (32 - bit_length);
    let subnetmask: IpAddr = IpAddr::V4(bits.into());

    subnetmask
}
