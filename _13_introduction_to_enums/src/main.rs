// ========================================
// Enums
// a versatile tool that is used to represent a type that can take on one of several possible variants
fn main() {
    enum IpAddressKind {
        V4,
        V6,
    }
    let four: IpAddressKind = IpAddressKind::V4;
    let six: IpAddressKind = IpAddressKind::V6;

    fn route(ip_kind: IpAddressKind) {}

    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    // using enums
    enum IpAddress2 {
        V4(String),
        V6(String),
    }

    let home2 = IpAddress2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddress2::V6(String::from("::1"));

    // using enums (enhanced)
    enum IpAddress3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home3 = IpAddress3::V4(127, 0, 0, 1);
    let loopback3 = IpAddress3::V6(String::from("::1"));

    // using structs
    struct IpAddress{
        kind: IpAddressKind,
        address: String,
    }

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
}


