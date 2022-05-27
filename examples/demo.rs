#![no_std]
#![no_main]

use embedded_storage::{ReadStorage, Storage};
#[cfg(feature = "esp32")]
use esp32_hal::{pac::Peripherals, prelude::*, RtcCntl, Timer};

#[cfg(feature = "esp32s2")]
use esp32s2_hal::{pac::Peripherals, prelude::*, RtcCntl, Timer};

#[cfg(feature = "esp32s3")]
use esp32s3_hal::{pac::Peripherals, prelude::*, RtcCntl, Timer};

#[cfg(feature = "esp32c3")]
use esp32c3_hal::{pac::Peripherals, prelude::*, RtcCntl, Timer};

use esp_storage::FlashStorage;
#[cfg(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3"))]
use xtensa_lx_rt::entry;

#[cfg(feature = "esp32c3")]
use riscv_rt::entry;

use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    #[cfg(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3"))]
    {
        let mut timer0 = Timer::new(peripherals.TIMG0);
        let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);

        // Disable MWDT and RWDT (Watchdog) flash boot protection
        timer0.disable();
        rtc_cntl.set_wdt_global_enable(false);
    }

    #[cfg(feature = "esp32c3")]
    {
        // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
        // the RTC WDT, and the TIMG WDTs.
        let mut rtc_cntl = RtcCntl::new(peripherals.RTC_CNTL);
        let mut timer0 = Timer::new(peripherals.TIMG0);
        let mut timer1 = Timer::new(peripherals.TIMG1);

        rtc_cntl.set_super_wdt_enable(false);
        rtc_cntl.set_wdt_enable(false);
        timer0.disable();
        timer1.disable();
    }

    let mut bytes = [0u8; 32];

    let mut flash = FlashStorage::new();

    let flash_addr = 0x9000;
    println!("Flash size = {}", flash.capacity());

    flash.read(flash_addr, &mut bytes).unwrap();
    println!("Read from {:x}:  {:02x?}", flash_addr, &bytes[..32]);

    bytes[0x00] = bytes[0x00].wrapping_add(1);
    bytes[0x01] = bytes[0x01].wrapping_add(2);
    bytes[0x02] = bytes[0x02].wrapping_add(3);
    bytes[0x03] = bytes[0x03].wrapping_add(4);
    bytes[0x04] = bytes[0x04].wrapping_add(1);
    bytes[0x05] = bytes[0x05].wrapping_add(2);
    bytes[0x06] = bytes[0x06].wrapping_add(3);
    bytes[0x07] = bytes[0x07].wrapping_add(4);

    flash.write(flash_addr, &bytes).unwrap();
    println!("Written to {:x}: {:02x?}", flash_addr, &bytes[..32]);

    let mut reread_bytes = [0u8; 32];
    flash.read(flash_addr, &mut reread_bytes).unwrap();
    println!("Read from {:x}:  {:02x?}", flash_addr, &reread_bytes[..32]);

    loop {}
}
