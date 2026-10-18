//! Dependency-free PNG encoder: uncompressed DEFLATE blocks inside a zlib
//! stream (`78 01`, 65535-byte stored blocks, Adler-32 tail) wrapped in
//! standard `IHDR` / `IDAT` / `IEND` chunks with IEEE 802.3 CRC-32.

const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

fn crc32_update(crc: u32, data: &[u8]) -> u32 {
    let mut crc = crc;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    crc
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in data {
        a = (a + u32::from(byte)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn push_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let crc = !crc32_update(crc32_update(0xFFFF_FFFF, kind), data);
    out.extend_from_slice(&crc.to_be_bytes());
}

/// zlib stream of stored (uncompressed) DEFLATE blocks: header `78 01`, then
/// `BFINAL|BTYPE=00` byte + `LEN`/`NLEN` pairs, closed by big-endian Adler-32.
fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + data.len() / 65535 * 5 + 32);
    out.push(0x78);
    out.push(0x01);
    let mut rest = data;
    loop {
        let n = rest.len().min(65535);
        let is_final = n == rest.len();
        out.push(u8::from(is_final));
        out.extend_from_slice(&(n as u16).to_le_bytes());
        out.extend_from_slice(&(!(n as u16)).to_le_bytes());
        out.extend_from_slice(&rest[..n]);
        if is_final {
            break;
        }
        rest = &rest[n..];
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

/// Encode packed RGBA pixels (`width * height * 4` bytes, row-major) as a PNG.
pub fn encode_rgba_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let row_bytes = width as usize * 4;
    assert_eq!(
        rgba.len(),
        row_bytes * height as usize,
        "encode_rgba_png: pixel buffer does not match width*height*4"
    );

    let mut raw = Vec::with_capacity((row_bytes + 1) * height as usize);
    for row in 0..height as usize {
        raw.push(0); // per-scanline filter type: None
        raw.extend_from_slice(&rgba[row * row_bytes..(row + 1) * row_bytes]);
    }

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8); // bit depth
    ihdr.push(6); // color type: truecolor with alpha
    ihdr.push(0); // compression: deflate
    ihdr.push(0); // filter: adaptive
    ihdr.push(0); // interlace: none

    let mut out = Vec::with_capacity(raw.len() + raw.len() / 65535 * 5 + 96);
    out.extend_from_slice(&PNG_SIGNATURE);
    push_chunk(&mut out, b"IHDR", &ihdr);
    push_chunk(&mut out, b"IDAT", &zlib_stored(&raw));
    push_chunk(&mut out, b"IEND", &[]);
    out
}
