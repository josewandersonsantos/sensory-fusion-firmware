use core::u16;

use crate::{checksum, utils};

const PAYLOAD_SIZE_MAX: usize = 128;
const SOF: u8 = 0x7E;
const HEADER_SIZE:usize = core::mem::size_of::<FrameHeader>();
static mut FRAME_ID: u8 = 1;

#[repr(u8)]
pub enum FrameType
{
    FR_TYPE_VERSION = 0,
    FR_TYPE_CFG = 1,
    FR_TYPE_GPS_DATA = 2,
    FR_TYPE_COORDS = 3,
    FR_TYPE_ACC_DATA = 4,
    FR_TYPE_GYR_DATA = 5,
    FR_TYPE_MPU_DATA = 6,
}

#[derive(Clone, Copy)]
pub struct FrameHeader
{
    pub size: u16,
    pub package_type: u8,
    pub frame_id: u8
}

pub struct FrameTx<'a>
{
    pub header: FrameHeader,
    pub payload: &'a [u8],
    pub crc: u16
}

pub fn get_header(tp: FrameType, len: u16) -> FrameHeader
{
    unsafe 
    {
        FRAME_ID = FRAME_ID.wrapping_add(1);

        FrameHeader
        {
            size: len,
            frame_id: FRAME_ID,
            package_type: tp as u8
        }
    }
}

pub fn build_frame<'a>(tp: FrameType, data: &'a [u8]) -> FrameTx<'a>
{
    let len = core::cmp::min(data.len(), PAYLOAD_SIZE_MAX);
    let payload = &data[..len];
    let header = get_header(tp, len as u16);
    let mut frame = FrameTx
    {
        header,
        payload,
        crc: 0
    };

    // CRC header + payload
    let header_bytes = utils::as_bytes(&frame.header);
    frame.crc = checksum::get_crc16(header_bytes, HEADER_SIZE as u16);
    frame.crc ^= checksum::get_crc16(payload, len as u16);

    frame
}

pub fn get_gps_coords<'a>(buf: &'a mut [u8; 12], lat: f32, lng: f32, height: f32,) -> FrameTx<'a>
{
    buf[0..4].copy_from_slice(&lat.to_le_bytes());
    buf[4..8].copy_from_slice(&lng.to_le_bytes());
    buf[8..12].copy_from_slice(&height.to_le_bytes());

    build_frame(FrameType::FR_TYPE_COORDS, buf)
}

pub fn get_gps_data<'a>(data: &'a [u8]) -> FrameTx<'a>
{
    build_frame(FrameType::FR_TYPE_GPS_DATA, data)
}

pub fn get_package_cfg<'a>(data: &'a [u8]) -> FrameTx<'a>
{
    build_frame(FrameType::FR_TYPE_CFG, data)
}

pub fn get_package_mpu_data<'a>(buf: &'a mut [u8; 28], acc_x: f32, acc_y: f32, acc_z: f32, gyr_x: f32, gyr_y: f32, gyr_z: f32, temp: f32) -> FrameTx<'a>
{
    buf[0..4].copy_from_slice(&acc_x.to_le_bytes());
    buf[4..8].copy_from_slice(&acc_y.to_le_bytes());
    buf[8..12].copy_from_slice(&acc_z.to_le_bytes());
    buf[12..16].copy_from_slice(&gyr_x.to_le_bytes());
    buf[16..20].copy_from_slice(&gyr_y.to_le_bytes());
    buf[20..24].copy_from_slice(&gyr_z.to_le_bytes());
    buf[24..28].copy_from_slice(&temp.to_le_bytes());

    build_frame(FrameType::FR_TYPE_MPU_DATA, buf)
}

pub fn get_package_acc_data<'a>(buf: &'a mut [u8; 12], x: f32, y: f32, z: f32) -> FrameTx<'a>
{
    buf[0..4].copy_from_slice(&x.to_le_bytes());
    buf[4..8].copy_from_slice(&y.to_le_bytes());
    buf[8..12].copy_from_slice(&z.to_le_bytes());

    build_frame(FrameType::FR_TYPE_ACC_DATA, buf)
}

pub fn get_package_gyr_data<'a>(buf: &'a mut [u8; 12], x: f32, y: f32, z: f32) -> FrameTx<'a>
{
    buf[0..4].copy_from_slice(&x.to_le_bytes());
    buf[4..8].copy_from_slice(&y.to_le_bytes());
    buf[8..12].copy_from_slice(&z.to_le_bytes());

    build_frame(FrameType::FR_TYPE_GYR_DATA, buf)
}

pub fn get_package_coords<'a>(buf: &'a mut [u8; 8], lat: f32, lon: f32) -> FrameTx<'a>
{
    buf[0..4].copy_from_slice(&lat.to_le_bytes());
    buf[4..8].copy_from_slice(&lon.to_le_bytes());

    build_frame(FrameType::FR_TYPE_COORDS, buf)
}

pub fn verify_package(frame: FrameTx) -> u8
{
    let crc_rx = frame.crc;
    let bytes = utils::as_bytes(&frame);
    let crc_cl = checksum::get_crc16(&bytes, frame.header.size + HEADER_SIZE as u16);
    
    (crc_cl == crc_rx) as u8
}