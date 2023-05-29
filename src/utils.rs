pub fn i64_to_bytes(i: i64) -> Vec<u8> {
    i.to_string().into_bytes()
}

pub fn bytes_to_i64(b: Vec<u8>) -> i64 {
    String::from_utf8(b).unwrap().parse::<i64>().unwrap()
}

pub fn str_to_bytes(s: &str) -> Vec<u8> {
    s.to_string().into_bytes()
}

pub fn bytes_to_str(b: Vec<u8>) -> String {
    String::from_utf8(b).unwrap()
}
