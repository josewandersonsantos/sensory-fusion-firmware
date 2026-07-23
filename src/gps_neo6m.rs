#![allow(dead_code)]

use crate::usart;
use crate::ccb;
/*
 * CONST
 */
const BUFFER_SIZE: usize = 512;
const LINE_SIZE: usize = 128;
/*
 * VARIABLES
 */
static mut RX_BUFFER: ccb::CircularBuffer<u8, BUFFER_SIZE> = ccb::CircularBuffer::new();
static mut LINE: [u8; LINE_SIZE] = [0; LINE_SIZE];
static mut LINE_POS: usize = 0;

/*
 * CALLBACKS
 */
static mut ON_LINE_RECEIVED: Option<fn(&str)> = None;
static mut ON_COORDS_RECEIVED: Option<fn(f32, f32, f32)> = None;

/*
 * ENUMS
 */
pub enum GPSFrequency
{
    F1Hz,
    F5Hz,
    F10Hz,
}

pub enum GPSProtocol
{
    NMEA,
    UBX,
    Both,
}

pub enum GPSBaudRate
{
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
}

pub enum GPSUpdateRate
{
    R1Hz,
    R5Hz,
    R10Hz,
    R20Hz,
    R100Hz,
}

pub enum GPSOperationMode
{
    Normal,
    PowerSaving,
    Airborne,
}

pub enum GPSNmeaSentence
{
    GGA,
    RMC,
    GLL,
    VTG,
    ZDA,
}

pub enum UBXClass
{
    NAV = 0x01,
    RXM = 0x02,
    INF = 0x04,
    ACK = 0x05,
    CFG = 0x06,
    UPD = 0x09,
    MON = 0x0A,
    AID = 0x0B,
    TIM = 0x0D,
    ESF = 0x10,
    MGA = 0x13,
    LOG = 0x21,
    SEC = 0x27,
}

pub enum UBXId
{
    NavPvt = 0x07,
    CfgPrt = 0x00,
    CfgMsg = 0x01,
    CfgRate = 0x08,
    CfgNav5 = 0x24,
}

pub fn append_byte(value: u8)
{
    unsafe
    {
        let _ = RX_BUFFER.push(value);
    }
}

pub fn process_gps()
{
    unsafe
    {
        while let Some(byte) = RX_BUFFER.pop()
        {
            if byte == b'\n'
            {
                let line_str = str::from_utf8(&LINE[..LINE_POS]).unwrap_or("");

                if line_str.starts_with("$")
                {
                    deframe_nmea(line_str);
                }

                LINE_POS = 0;
            }
            else if LINE_POS < LINE_SIZE
            {
                LINE[LINE_POS] = byte;
                LINE_POS += 1;
            }
            else
            {
                LINE_POS = 0; // Overflow de linha
            }
        }
    }
}

pub fn calculate_ubx_checksum(data: &[u8]) -> (u8, u8)
{
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;

    for &byte in data
    {
        ck_a = ck_a.wrapping_add(byte);
        ck_b = ck_b.wrapping_add(ck_a);
    }

    (ck_a, ck_b)
}

pub fn make_ubx_message(class: u8, id: u8, payload: &[u8], buffer: &mut [u8]) -> usize
{
    let payload_len = payload.len();
    let total_len = 6 + payload_len + 2;

    assert!(buffer.len() >= total_len);

    buffer[0] = 0xB5;
    buffer[1] = 0x62;
    buffer[2] = class;
    buffer[3] = id;

    let len = payload_len as u16;
    buffer[4] = (len & 0xFF) as u8;
    buffer[5] = (len >> 8) as u8;

    buffer[6..6+payload_len].copy_from_slice(payload);

    let checksum = calculate_ubx_checksum(&buffer[2..6+payload_len]);
    buffer[6+payload_len] = checksum.0;
    buffer[7+payload_len] = checksum.1;

    total_len
}

pub fn send_ubx_message(uart: usart::Usart, class: u8, id: u8, payload: &[u8])
{
    let mut buffer: [u8; 256] = [0; 256];
    let message_len = make_ubx_message(class, id, payload, &mut buffer);
    usart::write_bytes(uart, &buffer[..message_len]);
    return;

    // // Get ACK
    // let mut ack_buffer: [u8; 10] = [0; 10];
    // let mut ack_index = 0;
    // while ack_index < 10
    // {
    //     if let Some(byte) = pop_byte()
    //     {
    //         ack_buffer[ack_index] = byte;
    //         ack_index += 1;

    //         if ack_index >= 2 && &ack_buffer[ack_index-2..ack_index] == [0xB5, 0x62]
    //         {
    //             break; // Início do ACK recebido
    //         }
    //     }
    // }
}

pub fn send_ubx_cfg_msg_rate(uart: usart::Usart, rate_ms: u16)
{
    let payload = [
        (rate_ms & 0xFF) as u8,
        (rate_ms >> 8) as u8,
        0x01, // navRate
        0x00, // timeRef
    ];

    send_ubx_message(uart, UBXClass::CFG as u8, UBXId::CfgRate as u8, &payload);
}

pub fn send_ubx_cfg_msg_nav5(uart: usart::Usart, mode: GPSOperationMode)
{
    let mut payload: [u8; 36] = [0; 36];
    payload[0] = 0x05; // mask: set dynamic model
    payload[1] = 0x00;
    payload[2] = match mode
    {
        GPSOperationMode::Normal => 0x00, // Portable
        GPSOperationMode::PowerSaving => 0x02, // Stationary
        GPSOperationMode::Airborne => 0x06, // Airborne <1g
    };

    send_ubx_message(uart, UBXClass::CFG as u8, UBXId::CfgNav5 as u8, &payload);
}

pub fn send_ubx_cfg_msg_msg(uart: usart::Usart, msg_class: u8, msg_id: u8, rate: u8)
{
    let payload = [
        msg_class,
        msg_id,
        rate,
    ];

    send_ubx_message(uart, UBXClass::CFG as u8, UBXId::CfgMsg as u8, &payload);
}

pub fn send_ubx_cfg_msg_prt(uart: usart::Usart, port: u8, mode: u8)
{
    let payload = [
        port,
        mode,
    ];

    send_ubx_message(uart, UBXClass::CFG as u8, UBXId::CfgPrt as u8, &payload);
}

pub fn init(uart: usart::Usart, protocol: GPSProtocol, baud_rate: GPSBaudRate, update_rate: GPSUpdateRate, operation_mode: GPSOperationMode, nmea_sentences: &[GPSNmeaSentence], cb_line: fn(&str), cb_coords: fn(f32, f32, f32))
{
    unsafe
    { 
        ON_LINE_RECEIVED = Some(cb_line);
        ON_COORDS_RECEIVED = Some(cb_coords)
    };

    // --------------------------------------------------
    // SET UPDATE RATE (CFG-RATE)
    // --------------------------------------------------
    let rate_ms: u16 = match update_rate
    {
        GPSUpdateRate::R1Hz  => 1000,
        GPSUpdateRate::R5Hz  => 200,
        GPSUpdateRate::R10Hz => 100,
        GPSUpdateRate::R20Hz => 50,
        GPSUpdateRate::R100Hz => 10,
    };

    send_ubx_cfg_msg_rate(uart, rate_ms);
    // --------------------------------------------------
    // SET OPERATION MODE (CFG-NAV5)
    // --------------------------------------------------
    send_ubx_cfg_msg_nav5(uart, operation_mode);

    // --------------------------------------------------
    // DISABLE ALL NMEA MSGS
    // --------------------------------------------------
    let all_msgs = [
        (0xF0, 0x00), // GGA
        (0xF0, 0x01), // GLL
        (0xF0, 0x02), // GSA
        (0xF0, 0x03), // GSV
        (0xF0, 0x04), // RMC
        (0xF0, 0x05), // VTG
        (0xF0, 0x08), // ZDA
    ];

    for (class, id) in all_msgs
    {
        send_ubx_cfg_msg_msg(uart, class, id, 0); // 0 = desabilita
    }
    
    // --------------------------------------------------
    // ENABLE NME MSGS
    // --------------------------------------------------
    for nmea_sentence in nmea_sentences
    {
        match nmea_sentence
        {
            GPSNmeaSentence::GGA =>
            send_ubx_cfg_msg_msg(uart, 0xF0, 0x00, 1),
            
            GPSNmeaSentence::GLL =>
            send_ubx_cfg_msg_msg(uart, 0xF0, 0x01, 1),
            
            GPSNmeaSentence::RMC =>
            send_ubx_cfg_msg_msg(uart, 0xF0, 0x04, 1),
            
            GPSNmeaSentence::VTG =>
            send_ubx_cfg_msg_msg(uart, 0xF0, 0x05, 1),
            
            GPSNmeaSentence::ZDA =>
            send_ubx_cfg_msg_msg(uart, 0xF0, 0x08, 1),
        }
    }

    // --------------------------------------------------
    // SET PROTOCOL (UBX/NMEA)
    // --------------------------------------------------
    match protocol
    {
        GPSProtocol::NMEA =>
        {
        }

        GPSProtocol::UBX =>
        {
        }

        GPSProtocol::Both =>
        {
        }
    }

    // --------------------------------------------------
    // SET BAUD RATE (CFG-PRT)
    // --------------------------------------------------
    let baud: u32 = match baud_rate
    {
        GPSBaudRate::B9600   => 9600,
        GPSBaudRate::B19200  => 19200,
        GPSBaudRate::B38400  => 38400,
        GPSBaudRate::B57600  => 57600,
        GPSBaudRate::B115200 => 115200,
    };

    let mut payload: [u8; 20] = [0; 20];

    payload[0] = 1; // UART1
    payload[1] = 0;

    // mode (8N1)
    payload[4] = 0xD0;
    payload[5] = 0x08;

    // baudrate
    payload[8]  = (baud & 0xFF) as u8;
    payload[9]  = ((baud >> 8) & 0xFF) as u8;
    payload[10] = ((baud >> 16) & 0xFF) as u8;
    payload[11] = ((baud >> 24) & 0xFF) as u8;

    // in/out proto (UBX + NMEA)
    payload[12] = 0x07;
    payload[14] = 0x07;

    send_ubx_message(uart, UBXClass::CFG as u8, UBXId::CfgPrt as u8, &payload);

}

fn nmea_to_decimal(coord: &str) -> f32
{
    let raw: f32 = coord.parse().unwrap_or(0.0);

    let degrees = (raw / 100.0) as i32 as f32;
    let minutes = raw - (degrees * 100.0);

    degrees + (minutes / 60.0)
}

pub fn deframe_nmea(sentence: &str)
{
    unsafe
    {
        if let Some(cb) = ON_LINE_RECEIVED
        {
            cb(sentence);
        }
    }
    /*
     * NMEA GGA (Global Positioning System Fix Data)
     * 
     * Example:
     * $GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47
     * 
     * Index and fields:
     * [0]  Message ID           -> "$GPGGA"
     * [1]  UTC Time             -> "123519" (12:35:19 UTC)
     * [2]  Latitude             -> "4807.038" (48°07.038')
     * [3]  N/S                  -> "N" (North/South)
     * [4]  Longitude            -> "01131.000" (11°31.000')
     * [5]  E/W                  -> "E" (East/West)
     * [6]  Fix Quality          -> 0 = inválido, 1 = GPS fix, 2 = DGPS fix, 4 = RTK fix, 5 = RTK float
     * [7]  Number of Satellites -> "08"
     * [8]  HDOP                 -> "0.9" (Horizontal Dilution of Precision)
     * [9]  Altitude             -> "545.4" (metros)
     * [10] Altitude Unit        -> "M" (metros)
     * [11] Geoid Separation     -> "46.9" (metros)
     * [12] Geoid Unit           -> "M"
     * [13] Age of DGPS          -> (opcional)
     * [14] DGPS Station ID      -> (opcional)
     * [*]  Checksum             -> "*47"
     * 
     * Obs:
     * - Latitude/Longitude are in format DDMM.MMMM
     * - Converter to decimal: decimal = degree + (minutes / 60)
     */
    if sentence.starts_with("$GPGGA")
    {
        let mut fields: [&str; 15] = [""; 15]; // NMEA GGA 15 fields
        let mut field_count = 0;
        let mut start = 0;

        for (i, c) in sentence.char_indices()
        {
            if c == ',' || c == '*' || c == '\r' || c == '\n'
            {
                if field_count < fields.len()
                {
                    fields[field_count] = &sentence[start..i];
                    field_count += 1;
                }
                start = i + 1;
            }
        }

        if field_count > 5
        {
            let lat_str = fields[2];
            let lat_dir_str = fields[3];
            let lng_str = fields[4];
            let lng_dir_str = fields[5];
            let height_str = fields[9];

            let mut lat = nmea_to_decimal(lat_str);
            let mut lng = nmea_to_decimal(lng_str);
            let alt: f32 = height_str.parse().unwrap_or(0.0);

            if lat_dir_str == "S"
            {
                lat *= -1.0;
            }

            if lng_dir_str == "W"
            {
                lng *= -1.0;
            }

            unsafe
            {
                if let Some(cb) = ON_COORDS_RECEIVED
                {
                    cb(lat, lng, alt);
                }
            }
        }
    }
    /*
     * NMEA RMC (Recommended Minimum Navigation Data)
     *
     * Example:
     * $GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A        
     *
     * Indexes and field:
     * [0]  Message ID          -> "$GPRMC"
     * [1]  UTC Time            -> "123519" (12:35:19 UTC)
     * [2]  Status              -> "A" = válido, "V" = inválido
     * [3]  Latitude            -> "4807.038"
     * [4]  N/S                 -> "N"
     * [5]  Longitude           -> "01131.000"
     * [6]  E/W                 -> "E"
     * [7]  Speed over ground   -> "022.4" (nós)
     * [8]  Course (Track)      -> "084.4" (graus)
     * [9]  Date                -> "230394" (23/03/1994)
     * [10] Magnetic Variation  -> "003.1"
     * [11] E/W                 -> "W"
     * [*]  Checksum            -> "*6A"
     * 
     * Obs:
     * - Velocidade está em nós → converter: km/h = knots * 1.852
     * - Curso é o heading em relação ao norte verdadeiro
     */
    else if sentence.starts_with("$GPRMC")
    {
        let mut fields: [&str; 12] = [""; 12]; // NMEA RMC 12 fields
        let mut field_count = 0;
        let mut start = 0;

        for (i, c) in sentence.char_indices()
        {
            if c == ',' || c == '*' || c == '\r' || c == '\n'
            {
                if field_count < fields.len()
                {
                    fields[field_count] = &sentence[start..i];
                    field_count += 1;
                }
                start = i + 1;
            }
        }

        if field_count > 8
        {
            // let lat = fields[3];
            // let lon = fields[5];
            /*
             *
             */
        }
    }
}

