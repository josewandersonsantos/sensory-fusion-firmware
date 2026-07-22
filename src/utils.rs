#![allow(dead_code)]

use crate::{mcu, rcc, utils};

pub fn delay_ms(ms: u32)
{
    let ticks = (rcc::get_pclk1_frequency() / 1000) * ms;

    unsafe
    {
        let systick_ctrl = mcu::SYSTICK_CTRL as *mut u32;
        let systick_load = mcu::SYSTICK_LOAD as *mut u32;
        let systick_val  = mcu::SYSTICK_VAL  as *mut u32;

        utils::write_register32(systick_load, ticks - 1);
        utils::write_register32(systick_val, 0);
        utils::write_register32(systick_ctrl, 0b101); // enable + core clock

        while (utils::read_bit32(systick_ctrl, 16)) == 0 {}

        utils::write_register32(systick_ctrl, 0); // stop
    }
}

/// Lê um valor de 32 bits de um endereço de registrador
pub unsafe fn read_register32(addr: *const u32) -> u32
{
    core::ptr::read_volatile(addr)
}

/// Escreve um valor de 32 bits em um endereço de registrador
pub unsafe fn write_register32(addr: *mut u32, value: u32)
{
    core::ptr::write_volatile(addr, value)
}

pub fn read_bit32(register: *mut u32, bit: u8) -> u8
{
    unsafe
    {
        let value = read_register32(register);
        ((value >> bit) & 1) as u8
    }
}

pub fn set_bit32(register: *mut u32, bit: u8)
{
    unsafe
    {
        let value = read_register32(register);
        write_register32(register, value | (1 << bit));
    }
}

pub fn clear_bit32(register: *mut u32, bit: u8)
{
    unsafe
    {
        let value = read_register32(register);
        write_register32(register, value & !(1 << bit));
    }
}

/// Inverte os bits especificados pela máscara no registrador
pub unsafe fn toggle_register32(addr: *mut u32, mask: u32)
{
    let current = read_register32(addr as *const u32);
    let toggled = current ^ mask;
    write_register32(addr, toggled);
}

/// Escreve 4 bits no registrador, após limpar os bits naquela posição
pub unsafe fn write_bits32(addr: *mut u32, shift: u32, value: u32)
{
    let current = read_register32(addr as *const u32); // Cast necessário aqui
    let mask = !(0xF << shift);
    let updated = (current & mask) | ((value & 0xF) << shift);
    write_register32(addr, updated);
}


/// Lê um valor de 16 bits de um endereço de registrador
pub unsafe fn read_register16(addr: *const u16) -> u16
{
    core::ptr::read_volatile(addr)
}

/// Escreve um valor de 16 bits em um endereço de registrador
pub unsafe fn write_register16(addr: *mut u16, value: u16)
{
    core::ptr::write_volatile(addr, value)
}

pub fn read_bit16(register: *mut u16, bit: u8) -> u8
{
    unsafe
    {
        let value = read_register16(register);
        ((value >> bit) & 1) as u8
    }
}

pub fn set_bit16(register: *mut u16, bit: u8)
{
    unsafe
    {
        let value = read_register16(register);
        write_register16(register, value | (1 << bit));
    }
}

pub fn clear_bit16(register: *mut u16, bit: u8)
{
    unsafe
    {
        let value = read_register16(register);
        write_register16(register, value & !(1 << bit));
    }
}

/// Inverte os bits especificados pela máscara no registrador
pub unsafe fn toggle_register16(addr: *mut u16, mask: u16)
{
    let current = read_register16(addr as *const u16);
    let toggled = current ^ mask;
    write_register16(addr, toggled);
}

/// Escreve 4 bits no registrador, após limpar os bits naquela posição
pub unsafe fn write_bits16(addr: *mut u16, shift: u16, value: u16)
{
    let current = read_register16(addr as *const u16); // Cast necessário aqui
    let mask = !(0xF << shift);
    let updated = (current & mask) | ((value & 0xF) << shift);
    write_register16(addr, updated);
}

pub fn as_bytes<T>(data: &T) -> &[u8]
{
    unsafe
    {
        core::slice::from_raw_parts((data as *const T) as *const u8, core::mem::size_of::<T>(),)
    }
}