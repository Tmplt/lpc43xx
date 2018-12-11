#![doc = "Peripheral access API for LPC43XX microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn DAC();
    fn DMA();
    fn FLASH();
    fn ETHERNET();
    fn SDIO();
    fn LCD();
    fn USB0();
    fn USB1();
    fn SCT();
    fn RITIMER();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn MCPWM();
    fn ADC0();
    fn I2C0();
    fn I2C1();
    fn SPI_INT();
    fn ADC1();
    fn SSP0();
    fn SSP1();
    fn USART0();
    fn UART1();
    fn USART2();
    fn USART3();
    fn I2S0();
    fn I2S1();
    fn SPIFI();
    fn SGPIO_IINT();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
    fn GINT0();
    fn GINT1();
    fn EVENTROUTER();
    fn C_CAN1();
    fn ADCHS();
    fn ATIMER();
    fn RTC();
    fn WWDT();
    fn C_CAN0();
    fn QEI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 53] = [
    Vector { _handler: DAC },
    Vector { _reserved: 0 },
    Vector { _handler: DMA },
    Vector { _reserved: 0 },
    Vector { _handler: FLASH },
    Vector { _handler: ETHERNET },
    Vector { _handler: SDIO },
    Vector { _handler: LCD },
    Vector { _handler: USB0 },
    Vector { _handler: USB1 },
    Vector { _handler: SCT },
    Vector { _handler: RITIMER },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector { _handler: MCPWM },
    Vector { _handler: ADC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: SPI_INT },
    Vector { _handler: ADC1 },
    Vector { _handler: SSP0 },
    Vector { _handler: SSP1 },
    Vector { _handler: USART0 },
    Vector { _handler: UART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: I2S0 },
    Vector { _handler: I2S1 },
    Vector { _handler: SPIFI },
    Vector {
        _handler: SGPIO_IINT,
    },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
    Vector { _handler: GINT0 },
    Vector { _handler: GINT1 },
    Vector {
        _handler: EVENTROUTER,
    },
    Vector { _handler: C_CAN1 },
    Vector { _reserved: 0 },
    Vector { _handler: ADCHS },
    Vector { _handler: ATIMER },
    Vector { _handler: RTC },
    Vector { _reserved: 0 },
    Vector { _handler: WWDT },
    Vector { _reserved: 0 },
    Vector { _handler: C_CAN0 },
    Vector { _handler: QEI },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DAC"]
    DAC,
    #[doc = "2 - DMA"]
    DMA,
    #[doc = "4 - FLASH"]
    FLASH,
    #[doc = "5 - ETHERNET"]
    ETHERNET,
    #[doc = "6 - SDIO"]
    SDIO,
    #[doc = "7 - LCD"]
    LCD,
    #[doc = "8 - USB0"]
    USB0,
    #[doc = "9 - USB1"]
    USB1,
    #[doc = "10 - SCT"]
    SCT,
    #[doc = "11 - RITIMER"]
    RITIMER,
    #[doc = "12 - TIMER0"]
    TIMER0,
    #[doc = "13 - TIMER1"]
    TIMER1,
    #[doc = "14 - TIMER2"]
    TIMER2,
    #[doc = "15 - TIMER3"]
    TIMER3,
    #[doc = "16 - MCPWM"]
    MCPWM,
    #[doc = "17 - ADC0"]
    ADC0,
    #[doc = "18 - I2C0"]
    I2C0,
    #[doc = "19 - I2C1"]
    I2C1,
    #[doc = "20 - SPI_INT"]
    SPI_INT,
    #[doc = "21 - ADC1"]
    ADC1,
    #[doc = "22 - SSP0"]
    SSP0,
    #[doc = "23 - SSP1"]
    SSP1,
    #[doc = "24 - USART0"]
    USART0,
    #[doc = "25 - UART1"]
    UART1,
    #[doc = "26 - USART2"]
    USART2,
    #[doc = "27 - USART3"]
    USART3,
    #[doc = "28 - I2S0"]
    I2S0,
    #[doc = "29 - I2S1"]
    I2S1,
    #[doc = "30 - SPIFI"]
    SPIFI,
    #[doc = "31 - SGPIO_IINT"]
    SGPIO_IINT,
    #[doc = "32 - PIN_INT0"]
    PIN_INT0,
    #[doc = "33 - PIN_INT1"]
    PIN_INT1,
    #[doc = "34 - PIN_INT2"]
    PIN_INT2,
    #[doc = "35 - PIN_INT3"]
    PIN_INT3,
    #[doc = "36 - PIN_INT4"]
    PIN_INT4,
    #[doc = "37 - PIN_INT5"]
    PIN_INT5,
    #[doc = "38 - PIN_INT6"]
    PIN_INT6,
    #[doc = "39 - PIN_INT7"]
    PIN_INT7,
    #[doc = "40 - GINT0"]
    GINT0,
    #[doc = "41 - GINT1"]
    GINT1,
    #[doc = "42 - EVENTROUTER"]
    EVENTROUTER,
    #[doc = "43 - C_CAN1"]
    C_CAN1,
    #[doc = "45 - ADCHS"]
    ADCHS,
    #[doc = "46 - ATIMER"]
    ATIMER,
    #[doc = "47 - RTC"]
    RTC,
    #[doc = "49 - WWDT"]
    WWDT,
    #[doc = "51 - C_CAN0"]
    C_CAN0,
    #[doc = "52 - QEI"]
    QEI,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DAC => 0,
            Interrupt::DMA => 2,
            Interrupt::FLASH => 4,
            Interrupt::ETHERNET => 5,
            Interrupt::SDIO => 6,
            Interrupt::LCD => 7,
            Interrupt::USB0 => 8,
            Interrupt::USB1 => 9,
            Interrupt::SCT => 10,
            Interrupt::RITIMER => 11,
            Interrupt::TIMER0 => 12,
            Interrupt::TIMER1 => 13,
            Interrupt::TIMER2 => 14,
            Interrupt::TIMER3 => 15,
            Interrupt::MCPWM => 16,
            Interrupt::ADC0 => 17,
            Interrupt::I2C0 => 18,
            Interrupt::I2C1 => 19,
            Interrupt::SPI_INT => 20,
            Interrupt::ADC1 => 21,
            Interrupt::SSP0 => 22,
            Interrupt::SSP1 => 23,
            Interrupt::USART0 => 24,
            Interrupt::UART1 => 25,
            Interrupt::USART2 => 26,
            Interrupt::USART3 => 27,
            Interrupt::I2S0 => 28,
            Interrupt::I2S1 => 29,
            Interrupt::SPIFI => 30,
            Interrupt::SGPIO_IINT => 31,
            Interrupt::PIN_INT0 => 32,
            Interrupt::PIN_INT1 => 33,
            Interrupt::PIN_INT2 => 34,
            Interrupt::PIN_INT3 => 35,
            Interrupt::PIN_INT4 => 36,
            Interrupt::PIN_INT5 => 37,
            Interrupt::PIN_INT6 => 38,
            Interrupt::PIN_INT7 => 39,
            Interrupt::GINT0 => 40,
            Interrupt::GINT1 => 41,
            Interrupt::EVENTROUTER => 42,
            Interrupt::C_CAN1 => 43,
            Interrupt::ADCHS => 45,
            Interrupt::ATIMER => 46,
            Interrupt::RTC => 47,
            Interrupt::WWDT => 49,
            Interrupt::C_CAN0 => 51,
            Interrupt::QEI => 52,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "State Configurable Timer (SCT) with dither engine"]
pub struct SCT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT {}
impl SCT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sct::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for SCT {
    type Target = sct::RegisterBlock;
    fn deref(&self) -> &sct::RegisterBlock {
        unsafe { &*SCT::ptr() }
    }
}
#[doc = "State Configurable Timer (SCT) with dither engine"]
pub mod sct;
#[doc = "General Purpose DMA (GPDMA)"]
pub struct GPDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA {}
impl GPDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for GPDMA {
    type Target = gpdma::RegisterBlock;
    fn deref(&self) -> &gpdma::RegisterBlock {
        unsafe { &*GPDMA::ptr() }
    }
}
#[doc = "General Purpose DMA (GPDMA)"]
pub mod gpdma;
#[doc = "SPI Flash Interface (SPIFI)"]
pub struct SPIFI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIFI {}
impl SPIFI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spifi::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for SPIFI {
    type Target = spifi::RegisterBlock;
    fn deref(&self) -> &spifi::RegisterBlock {
        unsafe { &*SPIFI::ptr() }
    }
}
#[doc = "SPI Flash Interface (SPIFI)"]
pub mod spifi;
#[doc = "SD/MMC"]
pub struct SDMMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC {}
impl SDMMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmmc::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SDMMC {
    type Target = sdmmc::RegisterBlock;
    fn deref(&self) -> &sdmmc::RegisterBlock {
        unsafe { &*SDMMC::ptr() }
    }
}
#[doc = "SD/MMC"]
pub mod sdmmc;
#[doc = "External Memory Controller (EMC)"]
pub struct EMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMC {}
impl EMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emc::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for EMC {
    type Target = emc::RegisterBlock;
    fn deref(&self) -> &emc::RegisterBlock {
        unsafe { &*EMC::ptr() }
    }
}
#[doc = "External Memory Controller (EMC)"]
pub mod emc;
#[doc = "USB0 Host/Device/OTG controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &usb0::RegisterBlock {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "USB0 Host/Device/OTG controller"]
pub mod usb0;
#[doc = "USB1 Host/Device controller"]
pub struct USB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB1 {}
impl USB1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb1::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for USB1 {
    type Target = usb1::RegisterBlock;
    fn deref(&self) -> &usb1::RegisterBlock {
        unsafe { &*USB1::ptr() }
    }
}
#[doc = "USB1 Host/Device controller"]
pub mod usb1;
#[doc = "LCD controller"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcd::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &lcd::RegisterBlock {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "LCD controller"]
pub mod lcd;
#[doc = "EEPROM"]
pub struct EEPROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EEPROM {}
impl EEPROM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eeprom::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for EEPROM {
    type Target = eeprom::RegisterBlock;
    fn deref(&self) -> &eeprom::RegisterBlock {
        unsafe { &*EEPROM::ptr() }
    }
}
#[doc = "EEPROM"]
pub mod eeprom;
#[doc = "Ethernet"]
pub struct ETHERNET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET {}
impl ETHERNET {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for ETHERNET {
    type Target = ethernet::RegisterBlock;
    fn deref(&self) -> &ethernet::RegisterBlock {
        unsafe { &*ETHERNET::ptr() }
    }
}
#[doc = "Ethernet"]
pub mod ethernet;
#[doc = "Alarm timer"]
pub struct ATIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ATIMER {}
impl ATIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const atimer::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for ATIMER {
    type Target = atimer::RegisterBlock;
    fn deref(&self) -> &atimer::RegisterBlock {
        unsafe { &*ATIMER::ptr() }
    }
}
#[doc = "Alarm timer"]
pub mod atimer;
#[doc = "RTC REGFILE"]
pub struct REGFILE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REGFILE {}
impl REGFILE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const regfile::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for REGFILE {
    type Target = regfile::RegisterBlock;
    fn deref(&self) -> &regfile::RegisterBlock {
        unsafe { &*REGFILE::ptr() }
    }
}
#[doc = "RTC REGFILE"]
pub mod regfile;
#[doc = "Power Management Controller (PMC)"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074012160 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller (PMC)"]
pub mod pmc;
#[doc = "Configuration Registers (CREG)"]
pub struct CREG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CREG {}
impl CREG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const creg::RegisterBlock {
        1074016256 as *const _
    }
}
impl Deref for CREG {
    type Target = creg::RegisterBlock;
    fn deref(&self) -> &creg::RegisterBlock {
        unsafe { &*CREG::ptr() }
    }
}
#[doc = "Configuration Registers (CREG)"]
pub mod creg;
#[doc = "Event router"]
pub struct EVENTROUTER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVENTROUTER {}
impl EVENTROUTER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eventrouter::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for EVENTROUTER {
    type Target = eventrouter::RegisterBlock;
    fn deref(&self) -> &eventrouter::RegisterBlock {
        unsafe { &*EVENTROUTER::ptr() }
    }
}
#[doc = "Event router"]
pub mod eventrouter;
#[doc = "Real-Time Clock (RTC) and event recorder"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1074028544 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Clock (RTC) and event recorder"]
pub mod rtc;
#[doc = "Clock Generation Unit (CGU)"]
pub struct CGU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CGU {}
impl CGU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cgu::RegisterBlock {
        1074069504 as *const _
    }
}
impl Deref for CGU {
    type Target = cgu::RegisterBlock;
    fn deref(&self) -> &cgu::RegisterBlock {
        unsafe { &*CGU::ptr() }
    }
}
#[doc = "Clock Generation Unit (CGU)"]
pub mod cgu;
#[doc = "Clock Control Unit (CCU)"]
pub struct CCU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU1 {}
impl CCU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu1::RegisterBlock {
        1074073600 as *const _
    }
}
impl Deref for CCU1 {
    type Target = ccu1::RegisterBlock;
    fn deref(&self) -> &ccu1::RegisterBlock {
        unsafe { &*CCU1::ptr() }
    }
}
#[doc = "Clock Control Unit (CCU)"]
pub mod ccu1;
#[doc = "Clock Control Unit (CCU2)"]
pub struct CCU2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU2 {}
impl CCU2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu2::RegisterBlock {
        1074077696 as *const _
    }
}
impl Deref for CCU2 {
    type Target = ccu2::RegisterBlock;
    fn deref(&self) -> &ccu2::RegisterBlock {
        unsafe { &*CCU2::ptr() }
    }
}
#[doc = "Clock Control Unit (CCU2)"]
pub mod ccu2;
#[doc = "Reset Generation Unit (RGU)"]
pub struct RGU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RGU {}
impl RGU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rgu::RegisterBlock {
        1074081792 as *const _
    }
}
impl Deref for RGU {
    type Target = rgu::RegisterBlock;
    fn deref(&self) -> &rgu::RegisterBlock {
        unsafe { &*RGU::ptr() }
    }
}
#[doc = "Reset Generation Unit (RGU)"]
pub mod rgu;
#[doc = "Windowed Watchdog timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdt::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &wwdt::RegisterBlock {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog timer (WWDT)"]
pub mod wwdt;
#[doc = "USART0_2_3"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074270208 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0_2_3"]
pub mod usart0;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074532352 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074536448 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1074274304 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "SSP0/1"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp0::RegisterBlock {
        1074278400 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp0::RegisterBlock;
    fn deref(&self) -> &ssp0::RegisterBlock {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "SSP0/1"]
pub mod ssp0;
#[doc = "SSP1"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp0::RegisterBlock {
        1074548736 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp0::RegisterBlock;
    fn deref(&self) -> &ssp0::RegisterBlock {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "Timer0/1/2/3"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074282496 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer0/1/2/3"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074286592 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "TIMER2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074540544 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "TIMER3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074544640 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "System Control Unit (SCU) I/O configuration"]
pub struct SCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU {}
impl SCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu::RegisterBlock {
        1074290688 as *const _
    }
}
impl Deref for SCU {
    type Target = scu::RegisterBlock;
    fn deref(&self) -> &scu::RegisterBlock {
        unsafe { &*SCU::ptr() }
    }
}
#[doc = "System Control Unit (SCU) I/O configuration"]
pub mod scu;
#[doc = "GPIO pin interrupt"]
pub struct GPIO_PIN_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PIN_INT {}
impl GPIO_PIN_INT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_pin_int::RegisterBlock {
        1074294784 as *const _
    }
}
impl Deref for GPIO_PIN_INT {
    type Target = gpio_pin_int::RegisterBlock;
    fn deref(&self) -> &gpio_pin_int::RegisterBlock {
        unsafe { &*GPIO_PIN_INT::ptr() }
    }
}
#[doc = "GPIO pin interrupt"]
pub mod gpio_pin_int;
#[doc = "GPIO group interrupt 0"]
pub struct GPIO_GROUP_INT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_GROUP_INT0 {}
impl GPIO_GROUP_INT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_group_int0::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for GPIO_GROUP_INT0 {
    type Target = gpio_group_int0::RegisterBlock;
    fn deref(&self) -> &gpio_group_int0::RegisterBlock {
        unsafe { &*GPIO_GROUP_INT0::ptr() }
    }
}
#[doc = "GPIO group interrupt 0"]
pub mod gpio_group_int0;
#[doc = "GPIO_GROUP_INT1"]
pub struct GPIO_GROUP_INT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_GROUP_INT1 {}
impl GPIO_GROUP_INT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_group_int0::RegisterBlock {
        1074302976 as *const _
    }
}
impl Deref for GPIO_GROUP_INT1 {
    type Target = gpio_group_int0::RegisterBlock;
    fn deref(&self) -> &gpio_group_int0::RegisterBlock {
        unsafe { &*GPIO_GROUP_INT1::ptr() }
    }
}
#[doc = "Motor Control PWM (MOTOCONPWM)"]
pub struct MCPWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCPWM {}
impl MCPWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcpwm::RegisterBlock {
        1074397184 as *const _
    }
}
impl Deref for MCPWM {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &mcpwm::RegisterBlock {
        unsafe { &*MCPWM::ptr() }
    }
}
#[doc = "Motor Control PWM (MOTOCONPWM)"]
pub mod mcpwm;
#[doc = "I2C-bus interface"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074401280 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C-bus interface"]
pub mod i2c0;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074659328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1074405376 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S interface"]
pub mod i2s0;
#[doc = "I2S1"]
pub struct I2S1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S1 {}
impl I2S1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1074409472 as *const _
    }
}
impl Deref for I2S1 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S1::ptr() }
    }
}
#[doc = "C_CAN"]
pub struct C_CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for C_CAN1 {}
impl C_CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const c_can1::RegisterBlock {
        1074413568 as *const _
    }
}
impl Deref for C_CAN1 {
    type Target = c_can1::RegisterBlock;
    fn deref(&self) -> &c_can1::RegisterBlock {
        unsafe { &*C_CAN1::ptr() }
    }
}
#[doc = "C_CAN"]
pub mod c_can1;
#[doc = "Repetitive Interrupt Timer (RIT)"]
pub struct RITIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RITIMER {}
impl RITIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ritimer::RegisterBlock {
        1074528256 as *const _
    }
}
impl Deref for RITIMER {
    type Target = ritimer::RegisterBlock;
    fn deref(&self) -> &ritimer::RegisterBlock {
        unsafe { &*RITIMER::ptr() }
    }
}
#[doc = "Repetitive Interrupt Timer (RIT)"]
pub mod ritimer;
#[doc = "Quadrature Encoder Interface (QEI)"]
pub struct QEI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI {}
impl QEI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qei::RegisterBlock {
        1074552832 as *const _
    }
}
impl Deref for QEI {
    type Target = qei::RegisterBlock;
    fn deref(&self) -> &qei::RegisterBlock {
        unsafe { &*QEI::ptr() }
    }
}
#[doc = "Quadrature Encoder Interface (QEI)"]
pub mod qei;
#[doc = "Global Input Multiplexer Array (GIMA)"]
pub struct GIMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GIMA {}
impl GIMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gima::RegisterBlock {
        1074556928 as *const _
    }
}
impl Deref for GIMA {
    type Target = gima::RegisterBlock;
    fn deref(&self) -> &gima::RegisterBlock {
        unsafe { &*GIMA::ptr() }
    }
}
#[doc = "Global Input Multiplexer Array (GIMA)"]
pub mod gima;
#[doc = "Digital-to-Analog Converter (DAC)"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub mod dac;
#[doc = "C_CAN0"]
pub struct C_CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for C_CAN0 {}
impl C_CAN0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const c_can1::RegisterBlock {
        1074667520 as *const _
    }
}
impl Deref for C_CAN0 {
    type Target = c_can1::RegisterBlock;
    fn deref(&self) -> &c_can1::RegisterBlock {
        unsafe { &*C_CAN0::ptr() }
    }
}
#[doc = "10-bit Analog-to-Digital Converter (ADC)"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1074671616 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "10-bit Analog-to-Digital Converter (ADC)"]
pub mod adc0;
#[doc = "ADC1"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1074675712 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "12-bit Analog-to-Digital Converter High-Speed (ADCHS)"]
pub struct ADCHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADCHS {}
impl ADCHS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adchs::RegisterBlock {
        1074724864 as *const _
    }
}
impl Deref for ADCHS {
    type Target = adchs::RegisterBlock;
    fn deref(&self) -> &adchs::RegisterBlock {
        unsafe { &*ADCHS::ptr() }
    }
}
#[doc = "12-bit Analog-to-Digital Converter High-Speed (ADCHS)"]
pub mod adchs;
#[doc = "GPIO port"]
pub struct GPIO_PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORT {}
impl GPIO_PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_port::RegisterBlock {
        1074741248 as *const _
    }
}
impl Deref for GPIO_PORT {
    type Target = gpio_port::RegisterBlock;
    fn deref(&self) -> &gpio_port::RegisterBlock {
        unsafe { &*GPIO_PORT::ptr() }
    }
}
#[doc = "GPIO port"]
pub mod gpio_port;
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi::RegisterBlock {
        1074790400 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &spi::RegisterBlock {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "Serial GPIO (SGPIO)"]
pub struct SGPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SGPIO {}
impl SGPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sgpio::RegisterBlock {
        1074794496 as *const _
    }
}
impl Deref for SGPIO {
    type Target = sgpio::RegisterBlock;
    fn deref(&self) -> &sgpio::RegisterBlock {
        unsafe { &*SGPIO::ptr() }
    }
}
#[doc = "Serial GPIO (SGPIO)"]
pub mod sgpio;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SCT"]
    pub SCT: SCT,
    #[doc = "GPDMA"]
    pub GPDMA: GPDMA,
    #[doc = "SPIFI"]
    pub SPIFI: SPIFI,
    #[doc = "SDMMC"]
    pub SDMMC: SDMMC,
    #[doc = "EMC"]
    pub EMC: EMC,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "USB1"]
    pub USB1: USB1,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "EEPROM"]
    pub EEPROM: EEPROM,
    #[doc = "ETHERNET"]
    pub ETHERNET: ETHERNET,
    #[doc = "ATIMER"]
    pub ATIMER: ATIMER,
    #[doc = "REGFILE"]
    pub REGFILE: REGFILE,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "CREG"]
    pub CREG: CREG,
    #[doc = "EVENTROUTER"]
    pub EVENTROUTER: EVENTROUTER,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "CGU"]
    pub CGU: CGU,
    #[doc = "CCU1"]
    pub CCU1: CCU1,
    #[doc = "CCU2"]
    pub CCU2: CCU2,
    #[doc = "RGU"]
    pub RGU: RGU,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "SSP0"]
    pub SSP0: SSP0,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "SCU"]
    pub SCU: SCU,
    #[doc = "GPIO_PIN_INT"]
    pub GPIO_PIN_INT: GPIO_PIN_INT,
    #[doc = "GPIO_GROUP_INT0"]
    pub GPIO_GROUP_INT0: GPIO_GROUP_INT0,
    #[doc = "GPIO_GROUP_INT1"]
    pub GPIO_GROUP_INT1: GPIO_GROUP_INT1,
    #[doc = "MCPWM"]
    pub MCPWM: MCPWM,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "I2S1"]
    pub I2S1: I2S1,
    #[doc = "C_CAN1"]
    pub C_CAN1: C_CAN1,
    #[doc = "RITIMER"]
    pub RITIMER: RITIMER,
    #[doc = "QEI"]
    pub QEI: QEI,
    #[doc = "GIMA"]
    pub GIMA: GIMA,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "C_CAN0"]
    pub C_CAN0: C_CAN0,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADCHS"]
    pub ADCHS: ADCHS,
    #[doc = "GPIO_PORT"]
    pub GPIO_PORT: GPIO_PORT,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "SGPIO"]
    pub SGPIO: SGPIO,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SCT: SCT {
                _marker: PhantomData,
            },
            GPDMA: GPDMA {
                _marker: PhantomData,
            },
            SPIFI: SPIFI {
                _marker: PhantomData,
            },
            SDMMC: SDMMC {
                _marker: PhantomData,
            },
            EMC: EMC {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            USB1: USB1 {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            EEPROM: EEPROM {
                _marker: PhantomData,
            },
            ETHERNET: ETHERNET {
                _marker: PhantomData,
            },
            ATIMER: ATIMER {
                _marker: PhantomData,
            },
            REGFILE: REGFILE {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            CREG: CREG {
                _marker: PhantomData,
            },
            EVENTROUTER: EVENTROUTER {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            CGU: CGU {
                _marker: PhantomData,
            },
            CCU1: CCU1 {
                _marker: PhantomData,
            },
            CCU2: CCU2 {
                _marker: PhantomData,
            },
            RGU: RGU {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            SSP0: SSP0 {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            SCU: SCU {
                _marker: PhantomData,
            },
            GPIO_PIN_INT: GPIO_PIN_INT {
                _marker: PhantomData,
            },
            GPIO_GROUP_INT0: GPIO_GROUP_INT0 {
                _marker: PhantomData,
            },
            GPIO_GROUP_INT1: GPIO_GROUP_INT1 {
                _marker: PhantomData,
            },
            MCPWM: MCPWM {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            I2S1: I2S1 {
                _marker: PhantomData,
            },
            C_CAN1: C_CAN1 {
                _marker: PhantomData,
            },
            RITIMER: RITIMER {
                _marker: PhantomData,
            },
            QEI: QEI {
                _marker: PhantomData,
            },
            GIMA: GIMA {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            C_CAN0: C_CAN0 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADCHS: ADCHS {
                _marker: PhantomData,
            },
            GPIO_PORT: GPIO_PORT {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            SGPIO: SGPIO {
                _marker: PhantomData,
            },
        }
    }
}
