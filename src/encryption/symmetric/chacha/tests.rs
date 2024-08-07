//! Test vectors from: https://datatracker.ietf.org/doc/html/rfc8439

use chacha20::{
  cipher::{KeyIvInit, StreamCipher, StreamCipherSeek},
  ChaCha20,
};
use hex::FromHex;
use rand::{thread_rng, Rng};
use rstest::rstest;

use super::{block, quarter_round, ChaCha, Counter};
use crate::encryption::symmetric::chacha::IETFChaCha20;

#[test]
fn test_quarter_round() {
  let mut state = [
    0x879531e0, 0xc5ecf37d, 0x516461b1, 0xc9a62f8a, 0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0x2a5f714c,
    0x53372767, 0xb00a5631, 0x974c541a, 0x359e9963, 0x5c971061, 0x3d631689, 0x2098d9d6, 0x91dbd320,
  ];

  quarter_round(2, 7, 8, 13, &mut state);

  assert_eq!(state, [
    0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a, 0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0xcfacafd2,
    0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963, 0x5c971061, 0xccc07c79, 0x2098d9d6, 0x91dbd320,
  ]);
}

#[test]
fn chacha_block() {
  let key = [
    0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
  ];

  let nonce = [0x09000000, 0x4a000000, 0];
  let counter = Counter::new([1]);
  let state = block(&key, &counter, &nonce, 20);

  assert_eq!(state, [
    0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15, 0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20, 0x71, 0xc4,
    0xc7, 0xd1, 0xf4, 0xc7, 0x33, 0xc0, 0x68, 0x03, 0x04, 0x22, 0xaa, 0x9a, 0xc3, 0xd4, 0x6c, 0x4e,
    0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09, 0x14, 0xc2, 0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2,
    0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9, 0xcb, 0xd0, 0x83, 0xe8, 0xa2, 0x50, 0x3c, 0x4e,
  ]);
}

#[test]
fn chacha_block_2() {
  let key = [0u32; 8];
  let nonce = [0u32; 3];
  let counter = Counter::new([0]);
  let state = block(&key, &counter, &nonce, 20);

  assert_eq!(state, [
    0x76, 0xb8, 0xe0, 0xad, 0xa0, 0xf1, 0x3d, 0x90, 0x40, 0x5d, 0x6a, 0xe5, 0x53, 0x86, 0xbd, 0x28,
    0xbd, 0xd2, 0x19, 0xb8, 0xa0, 0x8d, 0xed, 0x1a, 0xa8, 0x36, 0xef, 0xcc, 0x8b, 0x77, 0x0d, 0xc7,
    0xda, 0x41, 0x59, 0x7c, 0x51, 0x57, 0x48, 0x8d, 0x77, 0x24, 0xe0, 0x3f, 0xb8, 0xd8, 0x4a, 0x37,
    0x6a, 0x43, 0xb8, 0xf4, 0x15, 0x18, 0xa1, 0x1c, 0xc3, 0x87, 0xb6, 0x69, 0xb2, 0xee, 0x65, 0x86
  ]);
}

#[test]
fn chacha_encrypt() {
  let key = [
    0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
  ];
  let nonce = [0, 0x4a000000, 0];
  let counter = Counter::new([1]);

  let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";

  let chacha = ChaCha::<20, 3, 1>::new(&key, &nonce);
  let ciphertext = chacha.encrypt(&counter, plaintext).unwrap();

  assert_eq!(ciphertext, [
    0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d, 0x69, 0x81,
    0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc, 0xfd, 0x9f, 0xae, 0x0b,
    0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59, 0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57,
    0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab, 0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8,
    0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d, 0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e,
    0x52, 0xbc, 0x51, 0x4d, 0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36,
    0x5a, 0xf9, 0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
    0x87, 0x4d,
  ]);

  let decrypt = chacha.decrypt(&counter, &ciphertext).unwrap();

  assert_eq!(decrypt, plaintext.to_vec());
}

#[rstest]
#[case([0, 10], [0, 11])]
#[case([1, u32::MAX], [2, 0])]
#[should_panic]
#[case([u32::MAX, u32::MAX, u32::MAX], [0, 0, 0])]
fn counter<const C: usize>(#[case] a: [u32; C], #[case] b: [u32; C]) {
  let mut counter = Counter::new(a);
  let val = counter.increment();
  assert!(val.is_ok());

  assert_eq!(counter.value, b);
}

#[test]
fn chacha_fuzz() {
  let mut rng = thread_rng();

  let key: [u32; 8] = rng.gen();
  let nonce: [u32; 3] = rng.gen();
  let plaintext = <[u8; 16]>::from_hex("000102030405060708090A0B0C0D0E0F").unwrap();

  // ronk chacha cipher
  let ronk_chacha = IETFChaCha20::new(&key, &nonce);
  let counter = Counter::new([0]);
  let ronk_ciphertext = ronk_chacha.encrypt(&counter, &plaintext).unwrap();
  let decrypted = ronk_chacha.decrypt(&counter, &ronk_ciphertext).unwrap();

  // Key and IV must be references to the `GenericArray` type.
  // Here we use the `Into` trait to convert arrays into it.
  let flat_key: [u8; 32] =
    key.iter().flat_map(|val| val.to_le_bytes()).collect::<Vec<u8>>().try_into().expect("err");
  let flat_nonce: [u8; 12] =
    nonce.iter().flat_map(|val| val.to_le_bytes()).collect::<Vec<u8>>().try_into().expect("err");
  let mut cipher = ChaCha20::new(&flat_key.into(), &flat_nonce.into());

  let mut buffer = plaintext;
  cipher.apply_keystream(&mut buffer);

  let ciphertext = buffer;

  assert_eq!(ronk_ciphertext, ciphertext.to_vec());

  // ChaCha ciphers support seeking
  cipher.seek(0u32);

  // decrypt ciphertext by applying keystream again
  cipher.apply_keystream(&mut buffer);
  assert_eq!(buffer, plaintext);
  assert_eq!(buffer.to_vec(), decrypted);
}
