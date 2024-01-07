/// Frame identifier as first byte
pub const FRAME_IDENTIFIER: u8 = 0xFF;

// Default max packet length
pub const DEFAULT_MAX_PACKET_LENGTH: u8 = 250;

/// Commands

/// Transmission of wM-BUS data
pub const CMD_DATA_REQ: u8 = 0x00;

/// Set the wM-Bus mode of operation in the volatile settings (RuntimeSettings).
/// Shall be used when the wM-BUS mode must be changed frequently
pub const CMD_SET_MODE_REQ: u8 = 0x04;

/// Reset the module
pub const CMD_RESET_REQ: u8 = 0x05;

/// Write parameters (UserSettings) to the non-volatile memory
pub const CMD_SET_REQ: u8 = 0x09;

/// Set the UART baud rate to a dedicated speed
pub const CMD_SETUARTSPEED_REQ: u8 = 0x10;

/// Read parameters (UserSettings) from the non-volatile memory
pub const CMD_GET_REQ: u8 = 0x0A;

/// Request serial number from module
pub const CMD_SERIALNO_REQ: u8 = 0x0B;

/// Request firmware version from module
pub const CMD_FWV_REQ: u8 = 0x0C;

/// Read out the module rx level
pub const CMD_RSSI_REQ: u8 = 0x0D;


/// Responses

/// Frame sent
pub const CMD_DATA_CNF: u8 = 0x80;

/// Output of received data
pub const CMD_DATA_IND: u8 = 0x03;


/// Response Status

/// Frame sent successfully
pub const FRAME_SEND_OK: u8 = 0x00;