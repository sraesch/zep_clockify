use std::{io::{Read, BufReader, BufRead}, ops::Add};

use anyhow::Result;

/// A token being read
enum Token {
    Token(String),
    EndOfRecord,
}

/// A status value being returned by the reader
enum ReaderResult<T = ()> {
    /// Reader read more data
    Data(T),

    /// Reader reached end of file
    Eof,
}

pub trait CSVReader {
    fn headers(&mut self, headers: &[String]);
    fn record(&mut self, record: &[String]);
    fn eof(&mut self);
}

pub struct CSVParser<R: Read> {
    reader: BufReader<R>,
    cur_line: String,
    cur_line_pos: usize,
}

impl<R: Read> CSVParser<R> {
    pub fn new(r: R) -> Self {
        Self {
            reader: BufReader::new(r),
            cur_line: String::new(),
            cur_line_pos: 0,
        }
    }

    // pub fn read(&mut self) -> Result<()> {
         
    // }

    fn read_token(&mut self) -> Result<ReaderResult<Token>> {
        let mut token = String::new();

        // determine if the token is encoded with quotes
        let delimiter = match self.read_char()? {
            ReaderResult::Data(c) => {
                if c == '"' {
                    '"'
                } else {
                    token.push(c);
                    ';'
                }
            }
            ReaderResult::Eof => {
                return Ok(ReaderResult::Eof);
            }
        };

        loop {
            // read until we encounter the delimiter
            let line = &self.cur_line[self.cur_line_pos..];
            match line.find(delimiter {
                Some(idx) => {
                }
                None => {
                    if delimiter == '"'{

                    } else {

                    }
                }
            }
        }

        Ok(ReaderResult::Data(token))
    }

    /// Reads and returns the next character
    fn read_char(&mut self) -> Result<ReaderResult<char>> {
        match self.check_line_buffer()? {
            ReaderResult::Data(_) => {
                let c = self.cur_line.chars().nth(self.cur_line_pos).unwrap();
                self.cur_line_pos += 1;

                Ok(ReaderResult::Data(c))
            }
            ReaderResult::Eof => Ok(ReaderResult::Eof)
        }

    }

    /// Checks and updated the internal line buffer if needed.
    fn check_line_buffer(&mut self) -> Result<ReaderResult> {
        if self.cur_line.len() == self.cur_line_pos {
            self.update_line()
        } else {
            Ok(ReaderResult::Data(()))
        }
    }

    /// Returns true if a new line could be read and false if we reached EOF
    fn update_line(&mut self) -> Result<ReaderResult> {
        let mut line = String::new();
        let len = self.reader.read_line(&mut line)?;

        if len == 0 {
            Ok(ReaderResult::Eof)
        } else {
            self.cur_line = line;
            self.cur_line_pos = 0;

            Ok(ReaderResult::Data(()))
        }
    }
}


struct StringBuf {
    s: String,
    pos: usize,
}
