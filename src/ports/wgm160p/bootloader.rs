//! Concrete bootloader construction and flash bank layout for the wgm160p

use blue_hal::{drivers::efm32gg11b::{clocks, flash::{self, Flash}}, efm32pac, hal::null::{NullError, NullFlash, NullSerial, NullSystick}};
use crate::{devices::{bootloader::Bootloader}, error::{self, Error}};
use super::autogenerated;
use super::autogenerated::memory_map::{EXTERNAL_BANKS, MCU_BANKS};

#[cfg(feature="ecdsa-verify")]
use crate::devices::image::EcdsaImageReader as ImageReader;
#[cfg(not(feature="ecdsa-verify"))]
use crate::devices::image::CrcImageReader as ImageReader;
use super::update_signal::NullUpdateSignal;

impl Bootloader<NullFlash, Flash, NullSerial, NullSystick, ImageReader, NullUpdateSignal> {
    pub fn new() -> Self {
        let mut peripherals = efm32pac::Peripherals::take().unwrap();
        let clocks = clocks::Clocks::new(peripherals.CMU, &mut peripherals.MSC);
        let mcu_flash = flash::Flash::new(peripherals.MSC, &clocks);
        Bootloader {
            mcu_flash,
            external_banks: &EXTERNAL_BANKS,
            mcu_banks: &MCU_BANKS,
            external_flash: None,
            serial: None,
            boot_metrics: Default::default(),
            start_time: None,
            recovery_enabled: false,
            greeting: autogenerated::LOADSTONE_GREETING,
            _marker: Default::default(),
            update_signal: None,
        }
    }
}

impl error::Convertible for flash::Error {
    fn into(self) -> Error {
        match self {
            flash::Error::MemoryNotReachable => Error::DriverError("[MCU Flash] Memory not reachable"),
            flash::Error::MisalignedAccess => Error::DriverError("[MCU Flash] Misaligned memory access"),
            flash::Error::MemoryIsLocked => Error::DriverError("[MCU Flash] Memory is locked"),
            flash::Error::InvalidAddress => Error::DriverError("[MCU Flash] Address is invalid"),
        }
    }
}

impl error::Convertible for NullError {
    fn into(self) -> Error { panic!("This error should never happen!") }
}
