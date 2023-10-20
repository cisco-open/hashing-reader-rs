// Copyright 2023 Cisco Systems, Inc.
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::HashingReader;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io::Cursor;

#[cfg(feature = "tokio")]
#[tokio::test]
async fn test_async_hashing() -> Result<(), Box<dyn Error>> {
    use tokio::io::AsyncReadExt;

    let data = "Hello, world!";

    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash1 = hasher.finalize();

    let cursor = Cursor::new(data);

    let (mut wrapper, wrapper_hasher) = HashingReader::<_, Sha256>::new(cursor);

    let mut buffer = String::new();
    let result = wrapper.read_to_string(&mut buffer).await?;
    assert_eq!(result, data.len());

    let hash2 = wrapper_hasher.try_recv().unwrap().unwrap();

    assert_eq!(data, buffer);
    assert_eq!(hash1.as_slice().len(), hash2.len());
    assert_eq!(hash1.as_slice(), hash2);

    Ok(())
}

#[test]
fn test_std_hashing() -> Result<(), Box<dyn Error>> {
    use std::io::Read;

    let data = "Hello, world!";

    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash1 = hasher.finalize();

    let cursor = Cursor::new(data);

    let (mut wrapper, wrapper_hasher) = HashingReader::<_, Sha256>::new(cursor);

    let mut buffer = String::new();
    let result = wrapper.read_to_string(&mut buffer);
    assert!(result.is_ok());

    let hash2 = wrapper_hasher.try_recv().unwrap().unwrap();

    assert_eq!(data, buffer);
    assert_eq!(hash1.as_slice().len(), hash2.len());
    assert_eq!(hash1.as_slice(), hash2);

    Ok(())
}

#[test]
fn test_std_hashing_eof() -> Result<(), Box<dyn Error>> {
    let data = "Hello, world!";

    let mut cursor = Cursor::new(data);

    let (mut wrapper, hasher) = HashingReader::<_, Sha256>::new(&mut cursor);

    drop(hasher);

    let mut buffer = String::new();
    let result = std::io::Read::read_to_string(&mut wrapper, &mut buffer);

    // This is an error since the HashingReader was unable to write to the channel.
    assert!(result.is_err());
    // However all the data up to the EOF is already copied.
    assert_eq!(data, buffer);

    Ok(())
}
