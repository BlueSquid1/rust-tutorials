struct IpV4 {
    oct0: u32,
    oct1: u32,
    oct2: u32,
    oct3: u32,
}

impl IpV4 {
    fn new(oct0: u32, oct1: u32, oct2: u32, oct3: u32) -> Self {
        return Self {
            oct0: oct0,
            oct1: oct1,
            oct2: oct2,
            oct3: oct3,
        };
    }
}

struct IpV6 {
    address: String,
}

impl IpV6 {
    fn new(address: &str) -> Self {
        return Self {
            address: String::from(address),
        };
    }
}

enum IpAddr {
    V4(IpV4),
    V6(IpV6),
}

fn get_raw_address(addr: &IpAddr) -> String {
    return match addr {
        IpAddr::V4(x) => {
            let oct0 = x.oct0;
            let oct1 = x.oct1;
            let oct2 = x.oct2;
            let oct3 = x.oct3;
            let result = format!("{}, {}, {}, {}", oct0, oct1, oct2, oct3);
            result
        }

        IpAddr::V6(x) => x.address.clone(),

        // Handle future definitions
        _ => String::from(""),
    };
}

fn main() {
    let first_addr = IpAddr::V4(IpV4::new(127, 0, 0, 1));
    let second_addr = IpAddr::V6(IpV6::new("1::::"));

    let first_raw_address = get_raw_address(&first_addr);
    let second_raw_address = get_raw_address(&second_addr);
    println!("first raw address is: {first_raw_address}");
    println!("second raw address is: {second_raw_address}");
}
