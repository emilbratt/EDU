#[warn(dead_code)]


// IMPLEMENT DATATYPE TO HANDLE THE TWO MAJOR IP VERSIONS

// Declare struct
struct IpAddr {
    kind: IpAddrKind, // this is an enum type.. declared below this block and can hold one of two values
    address: String,
}
// .. and add the enum type used in the struct.
enum IpAddrKind {
    V4,
    V6,
}

fn main() {

    // Instantiate the IpAddr structs for both IP versions.
    let local_v4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let local_v6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
