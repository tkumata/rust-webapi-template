pub struct ConvertedRgb {
    pub r: String,
    pub g: String,
    pub b: String
}

pub fn calc_hex(value: i32) -> String {
    let value: i32 = value.clamp(0, 255);

    format!("{:02X}", value)
}

pub fn to_hex(r: i32, g: i32, b: i32) -> ConvertedRgb {
    let rgb: ConvertedRgb = ConvertedRgb {
        r: calc_hex(r).to_string(),
        g: calc_hex(g).to_string(),
        b: calc_hex(b).to_string()
    };

    rgb
}