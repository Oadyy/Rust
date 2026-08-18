fn Enum() {
    /// call enum
    let ip_v4 = IpProtocol::V4;
    let ip_v6 = IpProtocol::V6;

    /// basic structure with enum
    let my_address = IpAddress {
        protocol: IpProtocol::V4,
        address: String::from("192.168.35.45"),
    }
    let home_address = IpAddress {
        protocol: IpProtocol::V4,
        address: String::form("127.0.0.1"),
    }

    /// advance structure with enum
    let my_address_2 = IpAddressNew::V4(192, 168, 35, 45);
    my_address_2.boardcast();

    /// option
    let none_number = Option::None; // no value
    let some_number = Option::Some(5); // have value
    /// real world check null
    _ = real_life_plus_one_1(some_number);
    _ = real_life_plus_one_2(none_number);
    _ = real_life_plus_one_3(none_number);
}

enum IpProtocol {
    V4,
    V6,
}

struct IpAddress {
    protocol: IpProtocol,
    address: String,
}

/// merge structure into enum
enum IpAddressNew {
    V4(u8, u8, u8, u8),
    V6(String),
} 

/// enum can implement like structure
impl IpAddressNew {
    fn boardcast(&self) {
        match self {
            IpAddressNew::V4(a, b, c, d) => {
                println!("ip {}.{}.{}.{} boardcasting", a, b, c, d);
            }
            IpAddressNew::V6(ip) => {
                println!("ip {} boardcasting", ip);
            }
        }
    }
}

/// option enum is null safe it only have Rust language
enum Option<T> {
    Some(T),
    None,
}

fn real_life_plus_one(data: Option<i32>) -> i32 {
    match data {
        None => {
            println!("no value");
            return 0;
        },
        Some(value) => return value + 1,
    }
}

/// improve real_life_plus_one with if let
fn real_life_plus_one_2(data: Option<i32>) -> i32 {
    if let Some(value) = data {
        return value + 1;
    } else {
        println!("no value");
        return 0;
    }
}

/// improve real_life_plus_one_2 eary return pattern
fn real_life_plus_one_3(data: Option<i32>) -> i32 {
    let Some(value) = data else {
        println!("no value");
        return 0;
    };

    value + 1
}