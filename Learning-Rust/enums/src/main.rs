// Enums data structures can only be one of its variants. 
// Both IPv4 and IPv6 are basically the same type, but they 
// treated the same. 

/*
enum IPAddrKind {
	V4,
	V6
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

*/

// We can make the above in a more concise way as follows 
/*
enum IpAddr {
 	
	// We know that version 4 IP addresses will always have four numeric
	// components that will have values between 0 and 255. So we need
	// the V4 to be four separate u8 values. 
     	// V4(String), 
	V4(u8, u8, u8, u8),
	V6(String),
}
*/

struct Ipv4Addr {
	// --snip--
}

struct Ipv6Addr {
	// --snip-- 
}
// These are similar to the enum poriton
struct QuitMEssage; // unit struct 

struct MoveMessage { 
	x: i32,
	y: i32,
}

struct WriteMessage(String); // tuple struct 

struct ChangeColorMessage(i32, i32, i32); // tuple struct 

impl Message {
	fn call(&self) {
	// sudo method 
}
}


enum Option<T> {
	Some(T),
	None,
}
enum IpAddr { 
	V4(Ipv4Addr),
	V6(Ipv6Addr),
}

enum Message { 
	Quit, 
	Move { x: i32, y: i32 }, 
	Write(String), 
	ChangeColor(i32, i32, i32),	
}
fn main() {

let four = IpAddrKind::V4;
let six = IpAddrKind::V6; 

route(IPAddrKind::V4);
route(IpAddrKind::V6);
 
let some_number = Some(5);
let some_string = Some("a string"); 

let absent_numer: Option<i32> = None;

let m = Message::write(String::from("Some bullshit"));
m.call();
/*
 This next code will have some compile errors 

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; 

RUST DOES NOT LIKE 

we can't add i8 to Option<i8>
*/   

// The bottom is the previous version for IPv4
// let home = IpAddr::V4(String::from("127.0.0.1"));

let home = IpAddr::V4(127,0,0,1); 

let loopback = IpAddr::V6(String::from("::1"));
}
fn route(ip_kind: IpAddrKind) {} 
