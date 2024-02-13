pub use crate::constants::*;

pub struct CommandParser<'a> {
    pub command: u8,
    pub payload_length: usize,
    pub data: &'a [u8],
}

impl<'a> CommandParser<'a> {
    pub fn parse(buffer: &'a [u8]) -> Result<CommandParser<'a>, ParseError> {
        // This will handle data received on UART
        let mut index = 0;
        // Each message starts with the frame identifier (0xFF) and the command
        let data_valid =
            buffer[index] == FRAME_IDENTIFIER && buffer.len() > MIN_PACKET_LENGTH.into();

        if !data_valid {
            return Err(ParseError(index));
        }

        index += 1;
        let command = buffer[index];

        index += 1;
        let payload_length: usize = buffer[index] as usize;

        // Payload length should be the number of remaining bytes in the buffer minus 1 for the CS byte
        if payload_length != buffer.len() - (index + 1) - 1 {
            return Err(ParseError(index));
        }

        index += 1;
        let data = &buffer[index..(buffer.len() - 1)];

        // TODO perform a check on valid checksum byte

        Ok(CommandParser {
            command,
            payload_length,
            data,
        })
    }
}

/// Error type for parsing
///
/// The number is the index of up to where it was correctly parsed
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ParseError(usize);
