use std::io;
use std::io::BufReader;

/// An implementation of `Read` that transforms JSON input where the outermost
/// structure is an array. The enclosing brackets and commas are removed,
/// causing the items to be adjacent to one another. This works with
/// [`serde_json::StreamDeserializer`].
pub(crate) struct ArrayStreamReader<T> {
    inner: T,
    depth: Option<usize>,
    inside_string: bool,
    escape_next: bool,
}

impl<T: io::Read> ArrayStreamReader<T> {
    pub(crate) fn new_buffered(inner: T) -> BufReader<Self> {
        BufReader::new(ArrayStreamReader {
            inner,
            depth: None,
            inside_string: false,
            escape_next: false,
        })
    }
}

#[inline]
fn do_copy(dst: &mut [u8], src: &[u8], len: usize) {
    if len == 1 {
        dst[0] = src[0]; // Avoids memcpy call.
    } else {
        dst[..len].copy_from_slice(&src[..len]);
    }
}

impl<T: io::Read> io::Read for ArrayStreamReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut tmp = vec![0u8; buf.len()];

        // The outer loop is here in case every byte was skipped, which can happen
        // easily if
        // `buf.len()` is 1. In this situation, the operation is retried until either no
        // bytes are read from the inner stream, or at least 1 byte is written to `buf`.
        loop {
            let byte_count = self.inner.read(&mut tmp)?;
            if byte_count == 0 {
                return if self.depth.is_some() {
                    Err(io::ErrorKind::UnexpectedEof.into())
                } else {
                    Ok(0)
                };
            }

            let mut tmp_pos = 0;
            let mut buf_pos = 0;
            for (i, b) in tmp.iter().cloned().enumerate() {
                if self.depth.is_none() {
                    match b {
                        b'[' => {
                            self.depth = Some(0);
                            tmp_pos = i + 1;
                        },
                        b if b.is_ascii_whitespace() => {},
                        b'\0' => break,
                        _ => return Err(io::ErrorKind::InvalidData.into()),
                    }
                    continue;
                }

                if self.inside_string {
                    match b {
                        b'\\' => self.escape_next = true,
                        b'"' if !self.escape_next => self.inside_string = false,
                        _ if self.escape_next => self.escape_next = false,
                        _ => {},
                    }
                    continue;
                }

                let depth = self.depth.unwrap();
                match b {
                    b'[' | b'{' => self.depth = Some(depth + 1),
                    b']' | b'}' if depth > 0 => self.depth = Some(depth - 1),
                    b'"' => self.inside_string = true,
                    b'}' if depth == 0 => return Err(io::ErrorKind::InvalidData.into()),
                    b',' | b']' if depth == 0 => {
                        let len = i - tmp_pos;
                        do_copy(&mut buf[buf_pos..], &tmp[tmp_pos..], len);
                        tmp_pos = i + 1;
                        buf_pos += len;
                        if b == b']' {
                            // Reached the end of outer array. If another array follows, the stream
                            // will continue.
                            self.depth = None;
                        }
                    },
                    _ => {},
                }
            }

            if tmp_pos < byte_count {
                let len = byte_count - tmp_pos;
                do_copy(&mut buf[buf_pos..], &tmp[tmp_pos..], len);
                buf_pos += len;
            }

            if buf_pos > 0 {
                // If at least some data was read, return with the amount. Otherwise, the outer
                // loop will try again.
                return Ok(buf_pos);
            }
        }
    }
}
