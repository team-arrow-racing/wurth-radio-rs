pub use crate::constants::*;

pub struct CommandParser<'a> {
    buffer: &'a [u8],
    index: usize,
    data_valid: bool,
    payload_length: usize,
    data: &'a [u8]
}

impl<'a> CommandParser<'a> {
    pub fn parse(buffer: &'a [u8]) -> CommandParser<'a> {
        CommandParser {
            buffer,
            index: 0,
            data_valid: true,
            payload_length: 0,
            data: &[0],
        }
    }
}

impl<'a> CommandParser<'a> {
    /// Tries reading an identifier
    pub fn expect_identifier(mut self, identifier: &[u8]) -> Self {
        // If we're already not valid, then quit
        if !self.data_valid {
            return self;
        }

        // Each message starts with the frame identifier (0xFF) and the command 
        if self.buffer[0] != 0xFF {
            self.data_valid = false;
            return self;
        }

        // First byte valid, shift index to next
        self.index += 1;
        
        if self.buffer[self.index..].len() < identifier.len() {
            self.data_valid = false;
            return self;
        }

        // Check identifier is correct
        if self.buffer[self.index] != identifier[0] {
            self.data_valid = false;
            return self;
        }

        // Identifier byte matches 
        self.index += identifier.len();

        self
    }

    pub fn expect_data(mut self) -> Self {
        // Get the payloads length and shift the index
        self.payload_length = self.buffer[self.index] as usize;
        self.index += 1;

         // Get the bytes in which the data should reside.
         let data_slice = match self.buffer.get(self.index..(self.index + self.payload_length)) {
             None => {
                 return self;
             }
             Some(int_slice) => int_slice,
         };
         if data_slice.is_empty() {
             // We probably hit the end of the buffer.
             // The parameter is empty but as it is optional not invalid
             // Advance the index to the character after the parameter separator (comma) if it's there.
             return self;
         }

         // Increment index to next position
         self.index += self.payload_length + 1;

         self.data = data_slice;

         self
    }

    /// Finish parsing the command and get the results
    pub fn finish(self) -> Result<&'a [u8], ParseError> {
        if self.data_valid {
            Ok(self.data)
        } else {
            Err(ParseError(self.index))
        }
    }
}
/// Error type for parsing
///
/// The number is the index of up to where it was correctly parsed
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ParseError(usize);