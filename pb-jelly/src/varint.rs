use bytes::Buf;
use std::cmp::min;
use std::io::{
    self,
    Write,
};

#[inline]
pub fn serialized_length(mut val: u64) -> usize {
    let mut ans = 1;
    while val & !0x7Fu64 != 0 {
        val >>= 7;
        ans += 1;
    }

    ans
}

#[inline]
pub fn write<W: Write>(mut val: u64, w: &mut W) -> io::Result<()> {
    loop {
        if val & !0x7Fu64 == 0 {
            w.write_all(&[val as u8])?;
            break;
        } else {
            w.write_all(&[((val & 0x7F) | 0x80) as u8])?;
            val >>= 7;
        }
    }

    Ok(())
}

#[inline]
pub fn read<B: Buf>(buf: &mut B) -> io::Result<Option<u64>> {
    if buf.remaining() == 0 {
        return Ok(None);
    }

    // Find offset of first byte with MSB not set
    let idx = match buf
        .chunk()
        .iter()
        .enumerate()
        .find(|&(_, val)| *val < 0x80 as u8)
        .map(|(idx, _)| idx)
    {
        Some(idx) => idx,
        // Fallback to per-byte read if data is span across multiple slices.
        None => return read_slow(buf),
    };

    let varint = {
        let buf = &buf.chunk()[..=idx];

        let mut r: u64 = 0;
        for byte in buf.iter().rev() {
            r = (r << 7) | u64::from(byte & 0x7f);
        }

        r
    };

    buf.advance(idx + 1);
    Ok(Some(varint))
}

fn read_slow<B: Buf>(buf: &mut B) -> io::Result<Option<u64>> {
    let mut r: u64 = 0;
    for index in 0..min(10, buf.remaining()) {
        let byte = buf.get_u8();
        r |= u64::from(byte & 0x7f) << (index * 7);
        if byte < 0x80 {
            return Ok(Some(r));
        }
    }
    Err(super::unexpected_eof())
}

#[inline]
pub fn ensure_read<B: Buf>(buf: &mut B) -> io::Result<u64> {
    match read(buf)? {
        Some(n) => Ok(n),
        None => Err(super::unexpected_eof()),
    }
}

#[test]
fn test_basic() {
    use crate::Message;
    use std::io::Cursor;

    let from_vec = |vec| read(&mut Cursor::new(vec)).unwrap().unwrap();

    let from_vec_split = |mut first_vec: Vec<u8>| {
        let at = first_vec.len() / 2;
        let second_vec = first_vec.split_off(at);
        read(&mut Cursor::new(first_vec).chain(Cursor::new(second_vec)))
            .unwrap()
            .unwrap()
    };

    assert_eq!(1000.serialize_to_vec(), vec![232, 7]);
    assert_eq!(from_vec(vec![232, 7]), 1000);
    assert_eq!(from_vec_split(vec![232, 7]), 1000);

    let minus1 = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1];
    assert_eq!((-1 as i32).serialize_to_vec(), minus1);
    assert_eq!((-1 as i64).serialize_to_vec(), minus1);
    assert_eq!(from_vec(minus1.clone()) as i32, -1);
    assert_eq!(from_vec(minus1.clone()) as i64, -1);
    assert_eq!(from_vec_split(minus1.clone()) as i32, -1);
    assert_eq!(from_vec_split(minus1) as i64, -1);

    let minus1000 = vec![152, 248, 255, 255, 255, 255, 255, 255, 255, 1];
    assert_eq!((-1000 as i32).serialize_to_vec(), minus1000);
    assert_eq!((-1000 as i64).serialize_to_vec(), minus1000);
    assert_eq!(from_vec(minus1000.clone()) as i32, -1000);
    assert_eq!(from_vec(minus1000.clone()) as i64, -1000);
    assert_eq!(from_vec_split(minus1000.clone()) as i32, -1000);
    assert_eq!(from_vec_split(minus1000) as i64, -1000);
}
