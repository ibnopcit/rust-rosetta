// Implements http://rosettacode.org/wiki/MD5/Implementation
/*
 * Ported from C - Simple MD5 implementation
* on Wikipedia https://en.wikipedia.org/wiki/MD5
*/
use std::iter::range_step;
use std::fmt::{Show, Formatter, Result};

#[cfg(not(test))]
fn main() {
    let inputs=
    [b"a",
    b"abc",
    b"message digest",
    b"abcdefghijklmnopqrstuvwxyz",
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
    b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"];

    for &input in inputs.iter() {
        println!("{}", md5(input));
    }
}

// Constants are the integer part of the sines of integers (in radians) * 2^32.
static k:[u32,..64] = [
0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee ,
0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501 ,
0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be ,
0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821 ,
0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa ,
0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8 ,
0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed ,
0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a ,
0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c ,
0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70 ,
0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05 ,
0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665 ,
0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039 ,
0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1 ,
0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1 ,
0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 ];

// r specifies the per-round shift amounts
static r:[u32,..64] = [7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
                      5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
                      4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
                      6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21];

// "newtype" for [u8,..16] to specify it's a MD5 hash
struct MD5([u8,..16]);
impl Show for MD5 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let MD5(md5)=*self;
        for b in md5.iter() {
            try!(write!(f, "{:02x}", *b));
        }
        Ok(())
    }
}

// leftrotate function definition
#[inline]
fn left_rotate(x: u32, c: u32) -> u32 {
    (x << c as uint) | (x >> (32 - c) as uint)
}

fn to_bytes(val: u64) -> [u8,..8]
{
    let mut tmp:[u8,..8] = [0u8,..8];
    for i in range (0u, 8) {
        tmp[i] = (val >> (8*i)) as u8;
    }
    tmp
}

fn md5(initial_msg: &[u8]) -> MD5
{
    let initial_len=initial_msg.len() as u64;

    // These vars will contain the hash
    let mut h:[u32,..4]=[0x67452301u32, 0xefcdab89, 0x98badcfe, 0x10325476];

     //Pre-processing:
    //append "1" bit to message
    //append "0" bits until message length in bits ≡ 448 (mod 512)
    //append length mod (2^64) to message
    let mut new_len=initial_len;
    while new_len % (512/8) != 448/8 {
        new_len+=1;
    }

    let mut msg = initial_msg.to_vec();
    msg.push(0x80u8); // append the "1" bit; most significant bit is "first"

    for _ in range (initial_len + 1, new_len) {
        msg.push(0); // append "0" bits
    }

    // append the len in bits at the end of the buffer.
    msg.push_all(to_bytes(initial_len << 3).as_slice());

    assert_eq!(msg.len() % 64, 0);

    let mut w:[u32,..16] = [0u32,..16];
    // Process the message in successive 512-bit chunks:
    //for each 512-bit chunk of message:
    for offset in range_step(0u64, new_len, (512/8)) {
        // break chunk into sixteen 32-bit words w[j], 0 ≤ j ≤ 15
        for i in range(0u32, 16) {
            let j = i as uint * 4 + offset as uint;
            w[i as uint] =
                    (msg[j]   as u32)      |
                    (msg[j+1] as u32) <<8  |
                    (msg[j+2] as u32) <<16 |
                     msg[j+3] as u32  <<24;
        }

        // Initialize hash value for this chunk:
        let (mut a, mut b, mut c, mut d) = (h[0], h[1], h[2], h[3]);

        // Main loop:
        for ind in range(0u, 64) {
            let (f,g) = match ind {
                i @ 0..15   => ( (b & c) | ((!b) & d), //f
                                i ),                   //g
                i @ 16..31  => ( (d & b) | ((!d) & c),
                                (5*i + 1) % 16 ),
                i @ 32..47  => ( b ^ c ^ d,
                                (3*i + 5) % 16 ),
                i           => ( c ^ (b | (!d)),
                                (7*i) % 16 )
            };

            let temp = d;
            d = c;
            c = b;
            b = b + left_rotate((a + f + k[ind] + w[g]), r[ind]);
            a = temp;
        }

        // Add this chunk's hash to result so far:
        h[0] += a;
        h[1] += b;
        h[2] += c;
        h[3] += d;
    }
    drop(msg); // cleanup, msg is freed

    //var char digest[16] := h0 append h1 append h2 append h3 //(Output is in little-endian)
    let mut digest = [0u8,..16];
    for (i, s) in h.iter().enumerate() {
        digest[i*4] = (*s ) as u8;
        digest[i*4+1] = (*s >> 8) as u8;
        digest[i*4+2] = (*s >> 16) as u8;
        digest[i*4+3] = (*s >> 24) as u8;
    }
    MD5(digest)
}

#[test]
fn helper_fns() {
    assert_eq!(64, left_rotate(8, 3));

    let exp:[u8,..8] = [64u8, 226, 1, 0, 0, 0, 0, 0];
    assert!(to_bytes(123456) == exp);
}

#[test]
fn known_hashes() {
    let in_out=
    [(b"",
        "d41d8cd98f00b204e9800998ecf8427e"),
    (b"a",
        "0cc175b9c0f1b6a831c399e269772661"),
    (b"abc",
        "900150983cd24fb0d6963f7d28e17f72"),
    (b"message digest",
        "f96b697d7cb7938d525a2f31aaf161d0"),
    (b"abcdefghijklmnopqrstuvwxyz",
        "c3fcd3d76192e4007dfb496cca67e13b"),
    (b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "d174ab98d277d9f5a5611c2c9f419d9f"),
    (b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        "57edf4a22be3c955ac49da2e2107b67a")];

    for &(i,o) in in_out.iter() {
        let m=md5(i);
        assert_eq!(format!("{}", m), o.to_string());
    }
}
