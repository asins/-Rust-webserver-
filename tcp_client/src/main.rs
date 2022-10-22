use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_push_format_macros() {
        let mut str: String = String::from("");

        let k: String = "key".into();
        let v: String = "value".into();
        str.push_str(&format!("{}: {}\r\n", k, v));

        assert_eq!(str, "key: value\r\n");
    }
}
