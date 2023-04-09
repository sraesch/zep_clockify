use std::io::{BufRead, BufReader, Read};

use anyhow::{bail, Result};

/// A single returned token
#[derive(Clone, Debug)]
pub struct Token {
    /// The value of the read token
    pub value: String,

    /// Determines if there are further tokens
    pub further_tokens: bool,
}

/// A status value being returned by the reader
pub enum ReaderResult<T = ()> {
    /// Reader read more data
    Data(T),

    /// Reader reached end of file
    Eof,
}

impl<T> ReaderResult<T> {
    pub fn get_data(&self) -> Option<&T> {
        match self {
            Self::Data(d) => Some(d),
            Self::Eof => None,
        }
    }
}

pub struct CSVParser<R: Read> {
    reader: BufReader<R>,

    /// The number of columns
    num_columns: usize,

    /// The characters of the current line
    cur_line: Vec<char>,

    /// The cursor position in the current line
    cur_line_cursor: usize,

    /// The line index
    line_index: usize,
}

impl<R: Read> CSVParser<R> {
    pub fn new(r: R) -> Self {
        Self {
            reader: BufReader::new(r),
            num_columns: 0,
            cur_line: Vec::new(),
            cur_line_cursor: 0,
            line_index: 0,
        }
    }

    /// Reads and returns the header record
    pub fn read_header_record(&mut self) -> Result<Vec<String>> {
        assert!(self.num_columns == 0, "Already read header record");

        let mut result: Vec<String> = Vec::new();

        loop {
            match self.read_token()? {
                ReaderResult::Data(token) => {
                    result.push(token.value);
                    if !token.further_tokens {
                        self.num_columns = result.len();
                        return Ok(result);
                    }
                }
                ReaderResult::Eof => {
                    bail!("Line {}: Got unexpected end", self.line_index);
                }
            }
        }
    }

    /// Reads a single record and stores the result in the provided reference.
    ///
    /// # Arguments
    /// * `record` - The reference to store the record.
    pub fn read_record(&mut self, record: &mut [String]) -> Result<ReaderResult<()>> {
        assert!(self.num_columns > 0);

        for (index, column) in record.iter_mut().enumerate() {
            match self.read_token()? {
                ReaderResult::Data(t) => {
                    *column = t.value;
                    let is_last = index + 1 == self.num_columns;
                    if t.further_tokens && is_last {
                        bail!("Line {}: Got too many columns for record", self.line_index);
                    } else if !t.further_tokens && !is_last {
                        bail!("Line {}: Got too few columns for record", self.line_index);
                    }
                }
                ReaderResult::Eof => {
                    if index == 0 {
                        return Ok(ReaderResult::Eof);
                    } else {
                        bail!("Line {}: Got unexpected end", self.line_index);
                    }
                }
            }
        }

        Ok(ReaderResult::Data(()))
    }

    /// Reads a token from the internal reader
    fn read_token(&mut self) -> Result<ReaderResult<Token>> {
        let mut token = String::new();

        // determine if the token is in quotes
        let delimiter = match self.read_char()? {
            ReaderResult::Data(c) => {
                if c == '"' {
                    '"'
                } else if c == ';' {
                    let further_tokens = self.cur_line_cursor < self.cur_line.len();

                    return Ok(ReaderResult::Data(Token {
                        value: token,
                        further_tokens,
                    }));
                } else {
                    token.push(c);
                    ';'
                }
            }
            ReaderResult::Eof => {
                return Ok(ReaderResult::Eof);
            }
        };

        // read content for the token until we've reached the delimiter or a newline if
        // it is not within quotes
        loop {
            // make sure the current line buffer is non-empty and stop if we reach EOF
            match self.check_line_buffer()? {
                ReaderResult::Eof => {
                    return Ok(ReaderResult::Eof);
                }
                _ => {}
            }

            // process next chunk...
            let line = &self.cur_line[self.cur_line_cursor..];
            match line.find(delimiter) {
                Some(idx) => {
                    (&line[..idx]).add_to_str(&mut token);
                    self.cur_line_cursor += idx + 1;

                    if delimiter == ';' {
                        return Ok(ReaderResult::Data(Token {
                            value: token,
                            further_tokens: true,
                        }));
                    } else {
                        // check if we can find semicolon
                        let line = &line[(idx + 1)..];
                        match line.find(';') {
                            Some(idx) => {
                                self.cur_line_cursor += idx + 1;
                                return Ok(ReaderResult::Data(Token {
                                    value: token,
                                    further_tokens: true,
                                }));
                            }
                            None => {
                                // we've reached the end of the record
                                self.cur_line_cursor = self.cur_line.len();
                                return Ok(ReaderResult::Data(Token {
                                    value: token,
                                    further_tokens: false,
                                }));
                            }
                        }
                    }
                }
                None => {
                    line.add_to_str(&mut token);
                    self.cur_line_cursor = self.cur_line.len();

                    if delimiter != '"' {
                        return Ok(ReaderResult::Data(Token {
                            value: token,
                            further_tokens: false,
                        }));
                    } else {
                        token.push('\n');
                    }
                }
            }
        }
    }

    /// Reads and returns the next character
    fn read_char(&mut self) -> Result<ReaderResult<char>> {
        match self.check_line_buffer()? {
            ReaderResult::Data(_) => {
                let c = self.cur_line[self.cur_line_cursor];
                self.cur_line_cursor += 1;

                Ok(ReaderResult::Data(c))
            }
            ReaderResult::Eof => Ok(ReaderResult::Eof),
        }
    }

    /// Checks and updated the internal line buffer if needed.
    fn check_line_buffer(&mut self) -> Result<ReaderResult> {
        if self.cur_line.len() == self.cur_line_cursor {
            self.update_line()
        } else {
            Ok(ReaderResult::Data(()))
        }
    }

    /// Returns true if a new line could be read and false if we reached EOF
    fn update_line(&mut self) -> Result<ReaderResult> {
        // read line and remove trailing line break if available
        let mut s = String::new();
        let len = self.reader.read_line(&mut s)?;
        if s.ends_with('\n') {
            s.pop();
        }

        // update the current line buffer
        self.cur_line.clear();
        self.cur_line.extend(s.chars());

        // increase internal line index
        self.line_index += 1;

        // we've reached EOF if the returned line length is 0
        if len == 0 {
            Ok(ReaderResult::Eof)
        } else {
            self.cur_line_cursor = 0;
            Ok(ReaderResult::Data(()))
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_tokenizer_simple1() {
        let s = "Abbreviation;;\"\";Description";
        let mut csv_reader = CSVParser::new(Cursor::new(s.as_bytes()));

        {
            let t0 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t0.further_tokens);
            assert_eq!(t0.value, "Abbreviation");

            let t1 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t1.further_tokens);
            assert_eq!(t1.value, "");

            let t2 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t2.further_tokens);
            assert_eq!(t2.value, "");

            let t3 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(!t3.further_tokens);
            assert_eq!(t3.value, "Description");

            assert!(csv_reader.read_token().unwrap().get_data().is_none());
        }
    }

    #[test]
    fn test_tokenizer_simple2() {
        let s = "Abbreviation;Description;;\"ID:\n123\"";
        let mut csv_reader = CSVParser::new(Cursor::new(s.as_bytes()));

        {
            let t0 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t0.further_tokens);
            assert_eq!(t0.value, "Abbreviation");

            let t1 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t1.further_tokens);
            assert_eq!(t1.value, "Description");

            let t2 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t2.further_tokens);
            assert_eq!(t2.value, "");

            let t3 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(!t3.further_tokens);
            assert_eq!(t3.value, "ID:\n123");
        }
    }

    #[test]
    fn test_tokenizer_complex() {
        let s = "Abbreviation;\"Project\nName\";ID\nproj1;Foobar;123";
        let mut csv_reader = CSVParser::new(Cursor::new(s.as_bytes()));

        {
            let t0 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t0.further_tokens);
            assert_eq!(t0.value, "Abbreviation");

            let t1 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t1.further_tokens);
            assert_eq!(t1.value, "Project\nName");

            let t2 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(!t2.further_tokens);
            assert_eq!(t2.value, "ID");

            let t3 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t3.further_tokens);
            assert_eq!(t3.value, "proj1");

            let t4 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(t4.further_tokens);
            assert_eq!(t4.value, "Foobar");

            let t5 = csv_reader.read_token().unwrap().get_data().unwrap().clone();
            assert!(!t5.further_tokens);
            assert_eq!(t5.value, "123");
        }
    }
}

trait CharOperations {
    fn find(&self, c: char) -> Option<usize>;
    fn add_to_str(&self, rhs: &mut String);
}

impl CharOperations for &[char] {
    #[inline]
    fn find(&self, c0: char) -> Option<usize> {
        self.iter().position(|c| *c == c0)
    }

    fn add_to_str(&self, rhs: &mut String) {
        rhs.extend(self.iter());
    }
}
