#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct INTPND16_1R {
    bits: u16,
}
impl INTPND16_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Interrupt pending bits of message objects 16 to 1. 0 = This message object is ignored by the message handler. 1 = This message object is the source of an interrupt."]
    #[inline]
    pub fn intpnd16_1(&self) -> INTPND16_1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INTPND16_1R { bits }
    }
}
