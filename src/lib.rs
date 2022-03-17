use base58::*;
use core::iter::{self, repeat};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Address {
    prefix: u8,
    pub_key: [u8; 32],
    sum: [u8; 2],
}

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Address {
    pub fn encode(&self) -> String {
        self.decoded().to_base58()
    }

    pub fn key(&self) -> Vec<u8> {
        self.pub_key.into()
    }

    fn decoded(&self) -> Vec<u8> {
        [&[self.prefix], self.pub_key.as_slice(), self.sum.as_slice()].concat()
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.encode())
    }
}

/// The vanity function generates a Polkadot address that contains the provided text filled with a
/// character of choice and the correct checksum to make it a valid address
#[cfg_attr(feature = "web", wasm_bindgen)]
pub fn vanity(input: &str, fill_c: char) -> Option<Address> {
    // TODO: how to support other prefixes?
    const POLKADOT: char = '1';
    let mut input = input.chars().map(replace_invalid);
    let addr = repeat(fill_c).take(46).map(|c| input.next().unwrap_or(c));
    let addr: String = iter::once(POLKADOT).chain(addr).collect();

    let decoded = addr.as_str().from_base58().ok()?;
    let decoded = &decoded[..decoded.len() - 2];
    Some(Address {
        prefix: decoded[0],
        pub_key: decoded[1..].try_into().expect("key fits"),
        sum: {
            let h = ss58hash(&decoded);
            h.as_bytes()[..2].try_into().expect("bigger than 2 bytes")
        },
    })
}

const fn replace_invalid(c: char) -> char {
    match c {
        'O' => 'o',
        'I' => 'i',
        'l' => 'L',
        _ => c,
    }
}

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    const PREFIX: &[u8] = b"SS58PRE";
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(PREFIX);
    context.update(data);
    context.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanity_address() {
        let address = vanity("HelloWorld", '1').expect("valid address");
        assert_eq!(
            address.encode(),
            "1HeLLoWorLd1111111111111111111111111111111112kn"
        );
    }
}
