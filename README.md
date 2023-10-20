# Hashing Reader

The `hashing_reader` is a Rust crate providing a `std::io::Read` and `tokio::io::AsyncRead` wrapper that can calculate a checksum as the data is being read.

It works with any object that implements `std::io::Read` or `tokio::io::AsyncRead`, and any hash function that implements the `Digest` trait such as [RustCrypto/hashes](https://github.com/RustCrypto/hashes)

It sends the computed hash over a channel when it reaches EOF or encounters an error.

## Usage

Uses for this library could be cases where we want to verify the checksum of a large file that might not fit into memory.

Here is a basic example:

```rust
use std::io::Read;
use sha2::Sha256;
use hashing_reader::HashingReader;

let data = "some data to hash";
let reader = data.as_bytes();

let (hr, rx) = HashingReader::<_, Sha256>::new(reader);

let mut buf = Vec::new();
let _ = hr.read_to_end(&mut buf);

let hash = rx.recv().unwrap().unwrap();
println!("Hash: {:?}", hash);
```

## API

### `HashingReader::new(reader: R) -> (Self, Receiver<Option<Vec<u8>>>)`

Construct a new `HashingReader` with the given reader object and a hash function.

It returns a tuple consisting of the `HashingReader` and a `Receiver` for the channel to which the hash will be sent.

### `Read` for HashingReader

`HashingReader` implements `std::io::Read`. When data is read from the HashingReader, it is also read from the underlying reader, and the hash function is updated with the read data.

When the underlying reader reaches EOF or encounters an error, the computed hash is sent to the channel.

### `AsyncRead` for HashingReader

`HashingReader` also implements `tokio::io::AsyncRead`. The behavior is the same as for std::io::Read.

### Testing

This crate has been thoroughly tested to ensure that it accurately computes hashes for both blocking and non-blocking readers, and properly sends the hash to the channel.

### Note

This crate uses `std::sync::mpsc::channel` for sending the hash. When the underlying reader reaches EOF (End of File), the hash of the read data is sent through the channel.

However, there are some important considerations for using this library effectively:

1. **Never reaching EOF:** If the underlying reader never reaches EOF, for instance, if it's a network stream that stays open indefinitely, the hash will never be sent through the channel. You should consider this while designing your program, especially if your use case may involve long-running or persistent connections.

2. **Multiple EOFs:** Depending on the source of your data and the specifics of your reader implementation, it's possible to encounter multiple EOF signals. This could occur with certain types of file formats, a complex custom reader, or reading from a source that has intermittent connectivity. In such cases, a new hash is sent each time an EOF is encountered. Be prepared to handle multiple hashes coming through the channel, and understand that each hash corresponds to a separate chunk of data, from the start of reading to the encountered EOF.

3. **Error Handling:** If an error occurs while reading from the underlying reader, an `Err` result is returned by the `read` method, and `None` is sent through the channel. Be sure to check for `None` values received from the channel, as this indicates an error during the read operation.

### License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
