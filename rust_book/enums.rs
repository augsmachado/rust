// Enums allow you to define a type by enumerating its possible variants
// We can enumerate all possible variants, which is where enumeration gets its name.

#![allow(unused)]
fn main() {
    // Many people will want to define enum on this format below
    /*
    enum IpAddrKind {
        V4,
        V6,
    }
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    

    // But this is actually the better solution to use enums.
    // We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct,
    // by putting data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6
    // variants will have associated String values.
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    // We attach data to each variant of the enum directly, so there is no need for an extra struct.
    // There’s another advantage to using an enum rather than a struct: each variant can have different types and
    // amounts of associated data.
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    */


    
    // Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used,
    // but it embeds the address data inside the variants in the form of two different structs, which are defined differently
    // for each variant:
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }


    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {

}