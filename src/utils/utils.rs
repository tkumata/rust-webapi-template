pub struct ConvertedRgb {
    pub r: String,
    pub g: String,
    pub b: String
}

pub fn to_hex(value: i32) -> String {
    let value = if value > 255 {
        255
    } else if value < 0 {
        0
    } else {
        value
    };
    format!("{:02X}", value)
}

pub fn convert_rgb(r: i32, g: i32, b: i32) -> ConvertedRgb {
    let rgb = ConvertedRgb {
        r: to_hex(r).to_string(),
        g: to_hex(g).to_string(),
        b: to_hex(b).to_string()
    };
    rgb
}
