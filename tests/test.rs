use wurth_radio::builder::CommandBuilder;
use wurth_radio::parser::CommandParser;
use wurth_radio::constants::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_version() {
        let mut buffer = [0; 120];
        let value = CommandBuilder::get_firmware_version(&mut buffer)
            .finish()
            .unwrap();

        assert_eq!(value, &[0xFF, 0x0C, 0x00, 0xF3]);
    }

    #[test]
    fn test_reset_module() {
        let mut buffer = [0; 120];
        let value = CommandBuilder::reset_module(&mut buffer)
            .finish()
            .unwrap();

        assert_eq!(value, &[0xFF, 0x05, 0x00, 0xFA]);
    }

    #[test]
    fn test_rx_level() {
        let mut buffer = [0; 120];
        let value = CommandBuilder::get_rx_level(&mut buffer)
            .finish()
            .unwrap();

        assert_eq!(value, &[0xFF, 0x0D, 0x00, 0xF2]);
    }

    #[test]
    fn test_send_data() {
        let mut buffer = [0; 120];
        let data = [0, 1, 2, 3, 4, 5, 6, 7, 0];
        let value = CommandBuilder::send_data(&mut buffer, &data)
            .finish()
            .unwrap();

        assert_eq!(value, &[0xFF, 0x00, 0x09, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00, 0xF6]);

        // Response for successful transmission is
        // 0xFF 0x80 0x01 0x00 0x7E

        // Data received on receiver radio is
        // 0xFF 0x03 0x0A 0x00 0x01 0x02 0x03 0x04 0x05 0x06 0x07 0x00 0x24 0xD2
        // Frame identifier, CMD, Length, Payload, RSSI, CS
        // This has the RSSI value added (see RSSI_Enable), hence the payloads size is increased
        // by 1 byte, whihc is why the Length here is 0x0A instead of 0x09 when sending the data
    }

    #[test]
    fn test_set_mode() {
        let mut buffer = [0; 120];
        let mode =  WMBusMode::S1M;
        let value = CommandBuilder::set_mode(&mut buffer, mode)
            .finish()
            .unwrap();

        assert_eq!(value, &[0xFF, 0x04, 0x01, 0x02, 0xF8]);
    }

    #[test]
    fn test_set_uart_speed() {
        let mut buffer = [0; 120];
        let value = CommandBuilder::set_uart_speed(&mut buffer, 7)
            .finish()
            .unwrap();

        assert_eq!(value, &[0xFF, 0x10, 0x01, 0x07, 0xE9]);
    }

    #[test]
    fn test_parse_fimware() {
        let fwv = CommandParser::parse(&[0xFF, 0x8C, 0x03, 0x02, 0x00, 0x06, 0x74])
            .expect_identifier(&[CMD_FWV_CNF])
            .expect_data()
            .finish()
            .unwrap();
        assert_eq!(fwv, &[0x02, 0x00, 0x06]);
    }

    #[test]
    fn test_parse_rssi() {
        let fwv = CommandParser::parse(&[0xFF, 0x8D, 0x01, 0x99, 0x74])
            .expect_identifier(&[CMD_RSSI_CNF])
            .expect_data()
            .finish()
            .unwrap();
        assert_eq!(fwv, &[0x99]);
    }
}