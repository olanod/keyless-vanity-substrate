use base58::*;
use core::iter::repeat;

#[cfg_attr(feature = "web", wasm_bindgen::prelude::wasm_bindgen)]
pub fn vanity(input: &str, fill_c: char) -> Option<String> {
    let mut chars = input.chars().map(replace_invalid);
    let s = repeat(fill_c).take(46).map(|c| chars.next().unwrap_or(c));
    let s: String = ['1', '1'].into_iter().chain(s).collect();

    let mut bytes = s.as_str().from_base58().ok()?;
    let len = bytes.len();
    let [h1, h2]: [u8; 2] = {
        let h = ss58hash(&bytes[1..len - 2]);
        h.as_bytes()[..2].try_into().unwrap()
    };
    bytes[len - 2] = h1;
    bytes[len - 1] = h2;
    Some(bytes[1..].to_base58())
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
        assert_eq!(address, "1HeLLoWorLd1111111111111111111111111111111112kn");
    }
}
