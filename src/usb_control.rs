// ! USB control request handling for STM32F103C8T6 (Blue Pill)
// ! This module processes standard USB control requests (e.g., GET_DESCRIPTOR, SET_ADDRESS)

#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::mcu;
use crate::utils;
use crate::usb_driver;
use crate::usb_types;
use crate::usb_endpoint;

fn get_descriptor(descriptors: &usb_types::Descriptors, wvalue: u16) -> &[u8]
{
    let desc_type  = (wvalue >> 8) as u8;
    let desc_index = (wvalue & 0xFF) as u8;
    
    match desc_type
    {
        1 => &descriptors.device_descriptor,
        2 => &descriptors.config_descriptor,
        3 => 
            match desc_index
            {
                0 => &descriptors.string0,
                1 => &descriptors.string1,
                2 => &descriptors.string2,
                3 => &descriptors.string3,
                _ => &descriptors.string0,
            },
        _ => &descriptors.string0
    }
}

fn handle_get_descriptor(ep: &mut usb_endpoint::Endpoint, descriptors: &usb_types::Descriptors, wvalue:u16, wlength: u16)
{
    let data = get_descriptor(descriptors, wvalue);

    if data != &[]
    {
        let len = core::cmp::min(data.len(), wlength as usize);
        ep.length = data.len();
        ep.state = usb_endpoint::EndpointState::DataIn;
        ep.position = 0;
        ep.data_buffer[..len].copy_from_slice(&data[..len]);

        usb_driver::send_next_packet(ep.number as usize, ep.tx_addr, len, &mut ep.position, &ep.data_buffer);
    }
    else
    {
        usb_driver::stall_ep(ep.number as usize);
    }
}

/// Handles STATUS packets (Standard Device Requests)
fn handle_get_status(ep: &mut usb_endpoint::Endpoint, wlength: u16)
{      
    // Response for GET_STATUS (Device): 00 00
    ep.data_buffer[0] = 0x00;   // bit0 = self-powered? (0 = no)
    ep.data_buffer[1] = 0x00;   // bit1 = remote wakeup? (0 = no)
    
    ep.length = core::cmp::min(wlength as usize, 2);
    ep.position = 0;
    ep.state = usb_endpoint::EndpointState::DataIn;

    usb_driver::send_next_packet(ep.number as usize, ep.tx_addr, ep.length, &mut ep.position, &ep.data_buffer);
}

fn handle_set_address(ep: &mut usb_endpoint::Endpoint, wValue: u16)
{
    // During the Data stage, the device should send a zero-length packet (ZLP) to acknowledge the request
    ep.state = usb_endpoint::EndpointState::StatusIn;        
    // The new device address will be set after the Status stage is completed
    let new_address = (wValue & 0x7F) as u8; // Device address is in wValue for SET_ADDRESS
    // Store the new address temporarily in the endpoint handler struct
    ep.address = new_address;
    
    usb_driver::write_tx_count(ep.number as usize, 0); // ZLP
    usb_driver::set_stat_tx_valid(ep.number as usize);
}

fn handle_set_configuration(epn: usize)
{
    // normalmente só aceita config 1
    //current_config = 1;

    // habilita endpoints aqui (EP1, etc)
    /*
     * <TODO>
     */

    // responde ZLP
    usb_driver::write_tx_count(epn, 0);
    usb_driver::set_stat_tx_valid(epn);
}

pub fn handle_setup(ep: &mut usb_endpoint::Endpoint, descriptors: &usb_types::Descriptors) -> u8
{
    let mut setup = [0u8; 8];
    // Read 8-byte SETUP packet from PMA
    // let base = usb_types::PMA_BASE as *const u16;
    // let addr_tx  = core::ptr::read_volatile(base.add(0));   // ADDR_TX  (offset 0x00)
    // let count_tx = core::ptr::read_volatile(base.add(2)) & 0x3FF;   // COUNT_TX (offset 0x02)
    // let addr_rx  = core::ptr::read_volatile(base.add(4));   // ADDR_RX  (offset 0x04)
    // let count_rx = core::ptr::read_volatile(base.add(6)) & 0x3FF;   // COUNT_RX (offset 0x06)

    usb_driver::pma_read(ep.rx_addr, &mut setup, 8);
    ep.state = usb_endpoint::EndpointState::Setup;

    // bRequest
    let brequesttype = setup[0];
    let brequest     = setup[1];
    let wvalue      = ((setup[3] as u16) << 8) | (setup[2] as u16);
    let windex      = ((setup[5] as u16) << 8) | (setup[4] as u16);
    let wlength     = ((setup[7] as u16) << 8) | (setup[6] as u16);

    match brequest
    {
        // GET_STATUS
        0 => 
        {
            handle_get_status(ep, wlength);
            return 0;
            // handle_get_descriptor 1, 18)
        },
        5 => 
        {
            handle_set_address(ep, wvalue);
            return 0;
        },
        // GET_DESCRIPTOR
        6 => 
        {
            handle_get_descriptor(ep, descriptors, wvalue, wlength);
            return 0;
        },
        // SET_CONFIGURATION
        9 => 
        {
            handle_set_configuration(ep.number as usize);
            return 0;
        },
        _ =>
        {
            // Indicate setup not handled, so class-specific handler can try to process it
            return 1;
        }
    }
}

/// Called when an IN transaction completes
pub fn handle_in(ep: &mut usb_endpoint::Endpoint)
{
    unsafe
    {
        match ep.state
        {
            usb_endpoint::EndpointState::DataIn =>
            {
                if ep.position < ep.length
                {
                    // More data to send
                    usb_driver::send_next_packet(ep.number as usize, ep.tx_addr, ep.length, &mut ep.position, &ep.data_buffer);
                } 
                else
                {
                    // Data stage finished → go to Status OUT stage
                    ep.state = usb_endpoint::EndpointState::StatusOut;
                    usb_driver::set_stat_tx_nak(ep.number as usize);
                    usb_driver::set_stat_rx_valid(ep.number as usize);
                }
            }
            usb_endpoint::EndpointState::StatusIn =>
            {
                if ep.address != 0
                {
                    // Set the new device address after the Status stage is completed
                    utils::write_register16(mcu::USB_DADDR as *mut u16, ep.address as u16 | (1 << usb_types::USBDADDR::EF as u8));
                    ep.address = 0;
                }
                // Status stage completed
                ep.state = usb_endpoint::EndpointState::Idle;
            }
            _ => {}
        }
    }
}

/// Called when an OUT transaction completes
pub fn handle_out(ep: &mut usb_endpoint::Endpoint)
{
    match ep.state
    {
        usb_endpoint::EndpointState::StatusOut =>
        {
            // Status stage completed
            ep.state = usb_endpoint::EndpointState::Idle;
            usb_driver::set_stat_tx_nak(ep.number as usize);
            usb_driver::set_stat_rx_valid(ep.number as usize);
        }
        _ => {}
    }
}
