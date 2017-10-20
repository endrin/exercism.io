#![feature(conservative_impl_trait)]

use std::iter;

/// Convert a list of numbers to a stream of bytes
/// encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(encoded_iter).collect()
}

/// Given a stream of bytes, extract all numbers
/// which are encoded in there.
pub fn from_bytes(
    bytes: &[u8],
) -> Result<Vec<u32>, &'static str> {
    let last_bit: u8 = 0b_1000_0000;

    let mut bs = bytes.as_ref();
    let mut result = vec![];
    loop {
        if bs.is_empty() {
            return Ok(result);
        }
        let bound = match bs.iter()
            .position(|b| b & last_bit == 0)
        {
            None => return Err("incomplete byte sequence"),
            Some(b) if b > 4 => {
                println!("bytes are {:?}", bytes);
                println!("bound is {:?}", b);
                return Err("overflow u32");
            }
            Some(b) => b,
        };
        let (value, rest) = bs.split_at(bound + 1);
        result.push(decode(value)?);
        bs = rest;
    }
}


// Implementation

// Get amount of bits in a given number
fn bin_len<T: Copy + Into<u32>>(n: &T) -> u32 {
    let n: u32 = (*n).into();
    n.count_zeros() + n.count_ones() - n.leading_zeros()
}

// Convert single value to iterator over encoded bytes
fn encoded_iter<'a>(
    value: &'a u32,
) -> impl Iterator<Item = u8> + 'a {
    let seven_bits: u32 = 0b_0111_1111;
    let last_bit: u32 = 0b_1000_0000;

    let num_chunks =
        (f64::from(bin_len(value)) / 7.0).ceil() as u32;

    (1..num_chunks)
        .rev()
        .map(move |offset| {
            (value >> (offset * 7) & seven_bits | last_bit)
                as u8
        })
        .chain(iter::once((value & seven_bits) as u8))
}

fn decode(bytes: &[u8]) -> Result<u32, &'static str> {
    let seven_bits: u8 = 0b_0111_1111;

    // if bytes.len() == 5
    //     && bytes.iter().last().unwrap().leading_zeros() < 4
    // {
    //     return Err("overflow u32");
    // }

    println!("bytes are {:?}", bytes);
    println!(
        "zeros at start are {:?}",
        bytes.iter().last().unwrap().leading_zeros()
    );


    Ok(
        bytes
            .iter()
            .map(|b| (b & seven_bits) as u32)
            .fold(0u32, |result, byte| (result << 7) | byte),
    )
}
