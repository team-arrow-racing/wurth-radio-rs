pub use crate::constants::*;

pub struct CommandBuilder<'a, STAGE> {
    buffer: &'a mut [u8],
    index: usize,
    phantom: core::marker::PhantomData<STAGE>,
}

impl<'a> CommandBuilder<'a, Uninitialized> {
    /// Requests the firmware version. The response will be in the form
    /// SFD CMD Length Major Minor Patch CS
    /// Example, receiving
    /// 0xFF 0x0C 0x03 0x02 0x00 0x06 0x74
    /// Means that we have firmware version 2.0.6
    pub fn get_firmware_version(buffer: &'a mut [u8]) -> CommandBuilder<'a, Command> {
        let mut builder = CommandBuilder::<'a, Command> {
            buffer,
            index: 0,
            phantom: Default::default(),
        };

        builder.try_append_data(&[FRAME_IDENTIFIER]);
        builder.try_append_data(&[CMD_FWV_REQ]);
        builder.try_append_data(&[0x00]); // Length of data

        builder
    }

    /// Trigger a software reset of the module. All volatile RuntimeSettings are reset
    /// to default and any temporary changes are discarded
    pub fn reset_module(buffer: &'a mut [u8]) -> CommandBuilder<'a, Command> {
        let mut builder = CommandBuilder::<'a, Command> {
            buffer,
            index: 0,
            phantom: Default::default(),
        };

        builder.try_append_data(&[FRAME_IDENTIFIER]);
        builder.try_append_data(&[CMD_RESET_REQ]);
        builder.try_append_data(&[0x00]); // Length of data

        builder
    }

    /// Returns the RX level as determined by the transceiver IC.
    /// To receive the strength of the last received packet, you need to enable the RSSI
    /// output option in the UserSettings. The response will be in the form
    /// SFD CMD Length RX_Level CS
    pub fn get_rx_level(buffer: &'a mut [u8]) -> CommandBuilder<'a, Command> {
        let mut builder = CommandBuilder::<'a, Command> {
            buffer,
            index: 0,
            phantom: Default::default(),
        };

        builder.try_append_data(&[FRAME_IDENTIFIER]);
        builder.try_append_data(&[CMD_RSSI_REQ]);
        builder.try_append_data(&[0x00]); // Length of data

        builder
    }

    /// Radio transmission command to transfer data
    pub fn send_data(buffer: &'a mut [u8], data: &[u8]) -> CommandBuilder<'a, Command> {
        let mut builder = CommandBuilder::<'a, Command> {
            buffer,
            index: 0,
            phantom: Default::default(),
        };
        // TODO inform that data must be a minimum of 9 bytes in length
        builder.try_append_data(&[FRAME_IDENTIFIER]);
        builder.try_append_data(&[CMD_DATA_REQ]);
        builder.try_append_data(&[data.len().try_into().unwrap()]);
        builder.try_append_data(data);

        builder
    }

    /// Changes the wM-Bus mode in the volatile memory of the module
    pub fn set_mode(buffer: &'a mut [u8], mode: WMBusMode) -> CommandBuilder<'a, Command> {
        let mut builder = CommandBuilder::<'a, Command> {
            buffer,
            index: 0,
            phantom: Default::default(),
        };
        builder.try_append_data(&[FRAME_IDENTIFIER]);
        builder.try_append_data(&[CMD_SET_MODE_REQ]);
        builder.try_append_data(&[0x01]);
        builder.try_append_data(&[mode as u8]);

        builder
    }

    /// Index should be from 0 to 8
    /// Refer to Table 14 of Metis-II Manual for baudrates
    /// Common baudrates and indices:
    /// 3 = 9600
    /// 7 = 115200
    /// 8 - 9600, ACLK (Only available for Metis-II with firmware 2.8.0 and newer)
    pub fn set_uart_speed(buffer: &'a mut [u8], speed_index: u8) -> CommandBuilder<'a, Command> {
        let mut builder = CommandBuilder::<'a, Command> {
            buffer,
            index: 0,
            phantom: Default::default(),
        };
        builder.try_append_data(&[FRAME_IDENTIFIER]);
        builder.try_append_data(&[CMD_SETUARTSPEED_REQ]);
        builder.try_append_data(&[0x01]);
        builder.try_append_data(&[speed_index]);

        builder
    }
}

impl<'a, ANY> CommandBuilder<'a, ANY> {
    fn try_append_data(&mut self, data: &[u8]) {
        let data_length = data.len();

        if let Some(buffer_slice) = self.buffer.get_mut(self.index..(self.index + data_length)) {
            // Yes, zip the buffer with the data
            for (buffer, data) in buffer_slice.iter_mut().zip(data) {
                // Copy over the bytes.
                *buffer = *data;
            }
        }

        // Increment the index
        self.index += data_length;
    }
}

impl<'a, F: Finishable> CommandBuilder<'a, F> {
    /// Finishes the builder.
    ///
    /// When Ok, it returns a slice with the built command.
    /// The slice points to the same memory as the buffer,
    /// but is only as long as is required to contain the command.
    ///
    /// The command length is thus the length of the slice.
    ///
    /// If the buffer was not long enough,
    /// then an Err is returned with the size that was required for it to succeed.
    pub fn finish(mut self) -> Result<&'a [u8], usize> {
        // Append the checksum byte first
        self.append_checksum_byte();
        // if last byte is a comma, decrement index to drop it
        if let Some(c) = self.buffer.get(self.index - 1) {
            if *c == b',' {
                self.index -= 1;
            }
        }

        if self.index > self.buffer.len() {
            Err(self.index)
        } else {
            Ok(&self.buffer[0..self.index])
        }
    }

    fn calculate_checksum_byte(&mut self) -> u8 {
        // Calculates the checksum byte
        // XOR of all preceeding bytes in the array
        let mut result = self.buffer[0];
        for byte in self.buffer.iter().skip(1) {
            result ^= byte;
        }

        result
    }

    fn append_checksum_byte(&mut self) {
        // Appends the checksum byte to the payload
        let cs_byte = self.calculate_checksum_byte();
        self.try_append_data(&[cs_byte]);
    }
}

/// Marker struct for uninitialized builders.
pub struct Uninitialized;
/// Marker struct for initialized builders.
/// The T type is the type the builder will be marked after it has been named.
pub struct Initialized<T>(core::marker::PhantomData<T>);
pub struct Command;

/// A trait that can be implemented for marker structs to indicate that the command is ready to be finished.
pub trait Finishable {}
impl Finishable for Command {}
