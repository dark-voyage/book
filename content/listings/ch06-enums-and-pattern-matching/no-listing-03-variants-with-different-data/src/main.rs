fn main() {
    // ANCHOR: here
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let asosiy = IpAddr::V4(127, 0, 0, 1);

    let orqaga_qaytish = IpAddr::V6(String::from("::1"));
    // ANCHOR_END: here
}
