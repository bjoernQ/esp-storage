#![no_std]

use embedded_storage::{ReadStorage, Storage};

#[cfg_attr(feature = "esp32c3", path = "esp32c3.rs")]
#[cfg_attr(feature = "esp32", path = "esp32.rs")]
#[cfg_attr(feature = "esp32s2", path = "esp32s2.rs")]
#[cfg_attr(feature = "esp32s3", path = "esp32s3.rs")]
mod chip_specific;

const FLASH_SECTOR_SIZE: u32 = 4096;

#[derive(Debug)]
pub enum FlashStorageError {
    Other(i32),
}

#[derive(Debug)]
pub struct FlashStorage {
    capacity: usize,
    unlocked: bool,
}

impl FlashStorage {
    pub fn new() -> FlashStorage {
        let mut storage = FlashStorage {
            capacity: 0,
            unlocked: false,
        };

        #[cfg(any(feature = "esp32c3", feature = "esp32s3"))]
        const ADDR: u32 = 0x0000;
        #[cfg(not(any(feature = "esp32c3", feature = "esp32s3")))]
        const ADDR: u32 = 0x1000;

        let mut buffer = [0u8; 8];
        storage.read(ADDR, &mut buffer).ok();
        let mb = match buffer[3] & 0xf0 {
            0x00 => 1,
            0x10 => 2,
            0x20 => 4,
            0x30 => 8,
            0x40 => 16,
            _ => 0,
        };
        storage.capacity = mb * 1024 * 1024;

        storage
    }
}

#[inline(never)]
#[link_section = ".rwtext"]
fn internal_read(offset: u32, bytes: &mut [u8]) -> Result<(), FlashStorageError> {
    if bytes.len() % 4 != 0 {
        return Err(FlashStorageError::Other(9999)); // TODO make this work - shouldn't be a requirement
    }

    chip_specific::begin();

    let res = chip_specific::esp_rom_spiflash_read(
        offset,
        bytes.as_ptr() as *mut u8 as *mut u32,
        bytes.len() as u32,
    );
    chip_specific::end();

    if res != 0 {
        Err(FlashStorageError::Other(res))
    } else {
        Ok(())
    }
}

#[inline(never)]
#[link_section = ".rwtext"]
fn internal_write(
    storage: &mut FlashStorage,
    offset: u32,
    bytes: &[u8],
) -> Result<(), FlashStorageError> {
    if bytes.len() % 4 != 0 {
        return Err(FlashStorageError::Other(9999)); // TODO make this work - shouldn't be a requirement
    }

    chip_specific::begin();

    if !storage.unlocked {
        if chip_specific::esp_rom_spiflash_unlock() != 0 {
            chip_specific::end();
            return Err(FlashStorageError::Other(9998));
        }
        storage.unlocked = true;
    }

    let res = chip_specific::esp_rom_spiflash_erase_sector(offset / FLASH_SECTOR_SIZE);
    if res != 0 {
        chip_specific::end();
        return Err(FlashStorageError::Other(res));
    }

    let res = chip_specific::esp_rom_spiflash_write(
        offset,
        bytes.as_ptr() as *const u8,
        bytes.len() as u32,
    );
    chip_specific::end();

    if res != 0 {
        Err(FlashStorageError::Other(res))
    } else {
        Ok(())
    }
}

impl ReadStorage for FlashStorage {
    type Error = FlashStorageError;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        internal_read(offset, bytes)
    }

    /// The SPI flash size is configured by writing a field in the software bootloader image header.
    /// This is done during flashing in espflash / esptool.
    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Storage for FlashStorage {
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        internal_write(self, offset, bytes)
    }
}
