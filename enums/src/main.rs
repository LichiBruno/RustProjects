
fn main() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    }
    
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;


    // this way we can put any kind of data inside an enum variant
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


    // enum whose variants each store different amounts and types of values
    enum Message {
        Quit, // no data associated
        Move { x: i32, y: i32 }, // name fields like a struct
        Write(String), // includes a single String
        ChangeColor(i32, i32, i32), // includes i32 values
    }

    // the Message enum is the same thing under this
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}