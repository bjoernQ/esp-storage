pub(crate) const ESP_ROM_SPIFLASH_READ: u32 = 0x40062ed8;
pub(crate) const ESP_ROM_SPIFLASH_ERASE_SECTOR: u32 = 0x40062ccc;
pub(crate) const ESP_ROM_SPIFLASH_WRITE: u32 = 0x40062d50;
pub(crate) const SPI_READ_STATUS_HIGH: u32 = 0x40062448;
pub(crate) const SPI_WRITE_STATUS: u32 = 0x400622f0;

pub(crate) const CACHE_READ_DISABLE_ROM: u32 = 0x40009ab8;
pub(crate) const CACHE_FLUSH_ROM: u32 = 0x40009a14;
pub(crate) const CACHE_READ_ENABLE_ROM: u32 = 0x40009a84;

pub(crate) const SPI_BASE_REG: u32 = 0x3ff42000; /* SPI peripheral 1, used for SPI flash */
pub(crate) const SPI0_BASE_REG: u32 = 0x3ff43000; /* SPI peripheral 0, inner state machine */
pub(crate) const SPI_EXT2_REG: u32 = SPI_BASE_REG + 0xF8;
pub(crate) const SPI0_EXT2_REG: u32 = SPI0_BASE_REG + 0xF8;
pub(crate) const SPI_RD_STATUS_REG: u32 = SPI_BASE_REG + 0x10;
pub(crate) const SPI_ST: u32 = 0x7;
pub(crate) const SPI_CMD_REG: u32 = SPI_BASE_REG + 0x00;
pub(crate) const SPI_FLASH_WREN: u32 = 1 << 30;
pub(crate) const SPI_FLASH_RDSR: u32 = 1 << 27;
pub(crate) const STATUS_WIP_BIT: u32 = 1 << 0;
pub(crate) const STATUS_QIE_BIT: u32 = 1 << 9; /* Quad Enable */
pub(crate) const SPI_CTRL_REG: u32 = SPI_BASE_REG + 0x08;
pub(crate) const SPI_WRSR_2B: u32 = 1 << 22;
pub(crate) const SPI_FLASH_WRDI: u32 = 1 << 29;

pub(crate) const FLASH_CHIP_ADDR: u32 = 0x3ffae270;

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn cache_read_disable_rom(cpu_num: u32) {
    unsafe {
        let cache_read_disable_rom: unsafe extern "C" fn(u32) =
            core::mem::transmute(CACHE_READ_DISABLE_ROM);
        cache_read_disable_rom(cpu_num)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn cache_flush_rom(cpu_num: u32) {
    unsafe {
        let cache_flush_rom: unsafe extern "C" fn(u32) = core::mem::transmute(CACHE_FLUSH_ROM);
        cache_flush_rom(cpu_num)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn cache_read_enable_rom(cpu_num: u32) {
    unsafe {
        let cache_read_enable_rom: unsafe extern "C" fn(u32) =
            core::mem::transmute(CACHE_READ_ENABLE_ROM);
        cache_read_enable_rom(cpu_num)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn spi_read_status_high(
    flash_chip: *const EspRomSpiflashChipT,
    status: &mut u32,
) -> i32 {
    unsafe {
        let spi_read_status_high: unsafe extern "C" fn(
            *const EspRomSpiflashChipT,
            *mut u32,
        ) -> i32 = core::mem::transmute(SPI_READ_STATUS_HIGH);
        spi_read_status_high(flash_chip, status as *mut u32)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn spi_write_status(flash_chip: *const EspRomSpiflashChipT, status_value: u32) -> i32 {
    unsafe {
        let spi_write_status: unsafe extern "C" fn(*const EspRomSpiflashChipT, u32) -> i32 =
            core::mem::transmute(SPI_WRITE_STATUS);
        spi_write_status(flash_chip, status_value)
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn begin() {
    cache_read_disable_rom(0);
    cache_read_disable_rom(1);
}

#[inline(always)]
#[link_section = ".rwtext"]
pub(crate) fn end() {
    cache_flush_rom(0);
    cache_flush_rom(1);
    cache_read_enable_rom(0);
    cache_read_enable_rom(1);
}

#[derive(Debug)]
#[repr(C)]
pub struct EspRomSpiflashChipT {
    device_id: u32,
    chip_size: u32, // chip size in bytes
    block_size: u32,
    sector_size: u32,
    page_size: u32,
    status_mask: u32,
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_read(src_addr: u32, data: *const u32, len: u32) -> i32 {
    spiflash_wait_for_ready();
    unsafe {
        let esp_rom_spiflash_read: unsafe extern "C" fn(u32, *const u32, u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_READ);
        esp_rom_spiflash_read(src_addr, data, len)
    }
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_erase_sector(sector_number: u32) -> i32 {
    spiflash_wait_for_ready();
    let res = unsafe {
        let esp_rom_spiflash_erase_sector: unsafe extern "C" fn(u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_ERASE_SECTOR);
        esp_rom_spiflash_erase_sector(sector_number)
    };
    spiflash_wait_for_ready();
    res
}

#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_write(dest_addr: u32, data: *const u8, len: u32) -> i32 {
    let res = unsafe {
        let esp_rom_spiflash_write: unsafe extern "C" fn(u32, *const u8, u32) -> i32 =
            core::mem::transmute(ESP_ROM_SPIFLASH_WRITE);
        esp_rom_spiflash_write(dest_addr, data, len)
    };
    spiflash_wait_for_ready();
    res
}

#[inline(always)]
#[link_section = ".rwtext"]
pub fn read_register(address: u32) -> u32 {
    unsafe { (address as *const u32).read_volatile() }
}

#[inline(always)]
#[link_section = ".rwtext"]
pub fn write_register(address: u32, value: u32) {
    unsafe {
        (address as *mut u32).write_volatile(value);
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
fn wait_for_ready() {
    while (read_register(SPI_EXT2_REG) & SPI_ST) != 0 {}
    while (read_register(SPI0_EXT2_REG) & SPI_ST) != 0 {} // ESP32_OR_LATER
}

#[inline(always)]
#[link_section = ".rwtext"]
fn spiflash_wait_for_ready() {
    loop {
        wait_for_ready();

        write_register(SPI_RD_STATUS_REG, 0);
        write_register(SPI_CMD_REG, SPI_FLASH_RDSR);
        while read_register(SPI_CMD_REG) != 0 {}
        if read_register(SPI_RD_STATUS_REG) & STATUS_WIP_BIT == 0 {
            return;
        }
    }
}

#[inline(always)]
#[link_section = ".rwtext"]
fn spi_write_enable() {
    spiflash_wait_for_ready();

    write_register(SPI_RD_STATUS_REG, 0);
    // Write flash enable.  Write enable command will be sent when the bit is set. The bit will be cleared once the operation done.
    write_register(SPI_CMD_REG, SPI_FLASH_WREN);
    while read_register(SPI_CMD_REG) != 0 {}
}

/* Stub version of SPIUnlock() that replaces version in ROM.

  This works around a bug where SPIUnlock sometimes reads the wrong
  high status byte (RDSR2 result) and then copies it back to the
  flash status, causing lock bit CMP or Status Register Protect ` to
  become set.
*/
#[inline(never)]
#[link_section = ".rwtext"]
pub(crate) fn esp_rom_spiflash_unlock() -> i32 {
    let flashchip = FLASH_CHIP_ADDR as *const EspRomSpiflashChipT;
    let mut status: u32 = 0;

    spiflash_wait_for_ready(); /* ROM SPI_read_status_high() doesn't wait for this */
    if spi_read_status_high(flashchip, &mut status) != 0 {
        return -1;
    }

    spiflash_wait_for_ready();
    /* Clear all bits except QIE, if it is set.
        (This is different from ROM SPIUnlock, which keeps all bits as-is.)
    */
    status &= STATUS_QIE_BIT;

    // two bytes data will be written to status register when it is set
    write_register(SPI_CTRL_REG, read_register(SPI_CTRL_REG) | SPI_WRSR_2B);

    spi_write_enable();
    if spi_write_status(flashchip, status) != 0 {
        return -1;
    }

    // WEL bit should be cleared after operations regardless of writing succeed or not.
    spiflash_wait_for_ready();
    write_register(SPI_CMD_REG, SPI_FLASH_WRDI);
    while read_register(SPI_CMD_REG) != 0 {}
    spiflash_wait_for_ready();

    0
}
