use std::net::IpAddr;

pub fn to_subnetmask(bit_length: i32) -> IpAddr {
    let bits: u32 = (!0) << (32 - bit_length);
    let subnetmask: IpAddr = IpAddr::V4(bits.into());

    subnetmask
}
