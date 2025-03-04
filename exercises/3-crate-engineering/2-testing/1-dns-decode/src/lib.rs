pub fn decode_dns_name(mut src: &[u8]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(src.len());

    if src.is_empty() {
        return None;
    }

    while src[0] != 0 && buf.len() <= 256 {
        let len = src[0] as usize;
        if len >= 0x40 {
            return None;
        }

        buf.extend(src.get(1..=len)?);
        buf.push(b'.');
        if buf.len() > 256 {
            return None;
        }
        src = &src[len + 1..];
    }

    buf.pop();
    Some(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    fn dns_decode(buffer: impl AsRef<[u8]>) -> Option<String> {
        decode_dns_name(buffer.as_ref())
            .map(String::from_utf8)
            .transpose()
            .unwrap()
    }

    #[test]
    fn simple() {
        let inp = b"\x06google\x03com\0";
        assert_eq!(dns_decode(inp).unwrap(), "google.com");
    }

    #[test]
    fn maximum_length() {
        use std::iter::*;
        let prt = b"\x0faaaaaaaaaaaaaaa";
        let mut inp = Vec::new();
        for _ in 0..16 {
            inp.extend(prt);
        }
        inp.push(b'\0');
        let ans = repeat("a".repeat(15))
            .take(16)
            .collect::<Vec<_>>()
            .join(".");
        assert_eq!(ans.len(), 255);
        assert_eq!(dns_decode(inp), Some(ans));
    }

    // reject malformed input
    #[test]
    fn too_long_inner_part() {
        let inp = b"\x40aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\0";
        assert_eq!(dns_decode(inp), None);
    }

    #[test]
    fn too_long_domain() {
        let inp = "\x01a".repeat(150) + "\0";
        assert_eq!(dns_decode(inp), None);
    }

    #[test]
    fn too_long_run() {
        let inp = b"\x05abc\0";
        assert_eq!(dns_decode(inp), None);
    }
}
