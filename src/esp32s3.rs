pub(crate) const ESP_ROM_SPIFLASH_READ: u32 = 0x40000a20;
pub(crate) const ESP_ROM_SPIFLASH_UNLOCK: u32 = 0x40000a2c;
pub(crate) const ESP_ROM_SPIFLASH_ERASE_SECTOR: u32 = 0x400009fc;
pub(crate) const ESP_ROM_SPIFLASH_WRITE: u32 = 0x40000a14;

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn begin() {}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn end() {}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_read(src_addr: u32, data: *const u32, len: u32) -> i32 {
    unsafe {
        let esp_rom_spiflash_read: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_READ);
        esp_rom_spiflash_read(src_addr, data, len)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_unlock() -> i32 {
    unsafe {
        let esp_rom_spiflash_unlock: unsafe extern "C" fn() -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_UNLOCK);
        esp_rom_spiflash_unlock()
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_erase_sector(sector_number: u32) -> i32 {
    unsafe {
        let esp_rom_spiflash_erase_sector: unsafe extern "C" fn(u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_ERASE_SECTOR);
        esp_rom_spiflash_erase_sector(sector_number)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_write(dest_addr: u32, data: *const u8, len: u32) -> i32 {
    unsafe {
        let esp_rom_spiflash_write: unsafe extern "C" fn(u32, *const u8, u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_WRITE);
        esp_rom_spiflash_write(dest_addr, data, len)
    }
}
