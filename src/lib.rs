//! # cs43l22 library
//! A rust-embedded driver for the Cirrus Logic cs43l22 digital/analog converter

#![no_std]
// #![deny(warnings)]

use embedded_hal::blocking::i2c::Write;

/// cs43l22 Error
#[derive(Debug, Copy, Clone)]
pub enum Error<E> {
    /// Arguments error.
    Param,
    /// Underlying bus error.
    Bus(E)
}

pub enum Register {
    Id = 0x01,
    PowerCtl1 = 0x02,
    PowerCtl2 = 0x04,
    ClockingCtl = 0x5,
    InterfaceCtl1 = 0x6,
    InterfaceCtl2 = 0x7
    // TODO: add all regs
}


