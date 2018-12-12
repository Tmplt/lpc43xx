#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCONFIG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ER {
    #[doc = "Channel disabled."]
    CHANNEL_DISABLED_,
    #[doc = "Channel enabled."]
    CHANNEL_ENABLED_,
}
impl ER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ER::CHANNEL_DISABLED_ => false,
            ER::CHANNEL_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ER {
        match value {
            false => ER::CHANNEL_DISABLED_,
            true => ER::CHANNEL_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_DISABLED_`"]
    #[inline]
    pub fn is_channel_disabled_(&self) -> bool {
        *self == ER::CHANNEL_DISABLED_
    }
    #[doc = "Checks if the value of the field is `CHANNEL_ENABLED_`"]
    #[inline]
    pub fn is_channel_enabled_(&self) -> bool {
        *self == ER::CHANNEL_ENABLED_
    }
}
#[doc = "Possible values of the field `SRCPERIPHERAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCPERIPHERALR {
    #[doc = "Source = SPIFI"]
    SOURCE_EQ_SPIFI,
    #[doc = "Source = Timer 0 match 0/UART0 transmit"]
    SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT,
    #[doc = "Source = Timer 0 match 1/UART0 receive"]
    SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE,
    #[doc = "Source = Timer 1 match 0/UART1 transmit"]
    SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT,
    #[doc = "Source = Timer 1 match 1/UART 1 receive"]
    SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE,
    #[doc = "Source = Timer 2 match 0/UART 2 transmit"]
    SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT,
    #[doc = "Source = Timer 2 match 1/UART 2 receive"]
    SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE,
    #[doc = "Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0"]
    SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0,
    #[doc = "Source = Timer 3 match 1/UART3 receive/SCT DMA request 1"]
    SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1,
    #[doc = "Source = SSP0 receive/I2S channel 0"]
    SOURCE_EQ_SSP0_RECEIVE_CH0,
    #[doc = "Source = SSP0 transmit/I2S channel 1"]
    SOURCE_EQ_SSP0_TRANSMIT_CH1,
    #[doc = "Source = SSP1 receive"]
    SOURCE_EQ_SSP1_RECEIVE,
    #[doc = "Source = SSP1 transmit"]
    SOURCE_EQ_SSP1_TRANSMIT,
    #[doc = "Source = ADC0"]
    SOURCE_EQ_ADC0,
    #[doc = "Source = ADC1"]
    SOURCE_EQ_ADC1,
    #[doc = "Source = DAC"]
    SOURCE_EQ_DAC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCPERIPHERALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCPERIPHERALR::SOURCE_EQ_SPIFI => 0,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT => 1,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE => 2,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT => 3,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE => 4,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT => 5,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE => 6,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0 => 7,
            SRCPERIPHERALR::SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1 => 8,
            SRCPERIPHERALR::SOURCE_EQ_SSP0_RECEIVE_CH0 => 9,
            SRCPERIPHERALR::SOURCE_EQ_SSP0_TRANSMIT_CH1 => 10,
            SRCPERIPHERALR::SOURCE_EQ_SSP1_RECEIVE => 11,
            SRCPERIPHERALR::SOURCE_EQ_SSP1_TRANSMIT => 12,
            SRCPERIPHERALR::SOURCE_EQ_ADC0 => 13,
            SRCPERIPHERALR::SOURCE_EQ_ADC1 => 14,
            SRCPERIPHERALR::SOURCE_EQ_DAC => 15,
            SRCPERIPHERALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCPERIPHERALR {
        match value {
            0 => SRCPERIPHERALR::SOURCE_EQ_SPIFI,
            1 => SRCPERIPHERALR::SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT,
            2 => SRCPERIPHERALR::SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE,
            3 => SRCPERIPHERALR::SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT,
            4 => SRCPERIPHERALR::SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE,
            5 => SRCPERIPHERALR::SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT,
            6 => SRCPERIPHERALR::SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE,
            7 => SRCPERIPHERALR::SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0,
            8 => SRCPERIPHERALR::SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1,
            9 => SRCPERIPHERALR::SOURCE_EQ_SSP0_RECEIVE_CH0,
            10 => SRCPERIPHERALR::SOURCE_EQ_SSP0_TRANSMIT_CH1,
            11 => SRCPERIPHERALR::SOURCE_EQ_SSP1_RECEIVE,
            12 => SRCPERIPHERALR::SOURCE_EQ_SSP1_TRANSMIT,
            13 => SRCPERIPHERALR::SOURCE_EQ_ADC0,
            14 => SRCPERIPHERALR::SOURCE_EQ_ADC1,
            15 => SRCPERIPHERALR::SOURCE_EQ_DAC,
            i => SRCPERIPHERALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_SPIFI`"]
    #[inline]
    pub fn is_source_eq_spifi(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_SPIFI
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT`"]
    #[inline]
    pub fn is_source_eq_timer_0_match_0_uart0_transmit(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE`"]
    #[inline]
    pub fn is_source_eq_timer_0_match_1_uart0_receive(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT`"]
    #[inline]
    pub fn is_source_eq_timer_1_match_0_uart1_transmit(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE`"]
    #[inline]
    pub fn is_source_eq_timer_1_match_1_uart1_receive(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT`"]
    #[inline]
    pub fn is_source_eq_timer_2_match_0_uart2_transmit(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE`"]
    #[inline]
    pub fn is_source_eq_timer_2_match_1_uart2_receive(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0`"]
    #[inline]
    pub fn is_source_eq_timer_3_match_0_uart3_receive_dma_req_0(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1`"]
    #[inline]
    pub fn is_source_eq_timer_3_match_1_uart3_receive_dma_req_1(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_SSP0_RECEIVE_CH0`"]
    #[inline]
    pub fn is_source_eq_ssp0_receive_ch0(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_SSP0_RECEIVE_CH0
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_SSP0_TRANSMIT_CH1`"]
    #[inline]
    pub fn is_source_eq_ssp0_transmit_ch1(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_SSP0_TRANSMIT_CH1
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_SSP1_RECEIVE`"]
    #[inline]
    pub fn is_source_eq_ssp1_receive(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_SSP1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_SSP1_TRANSMIT`"]
    #[inline]
    pub fn is_source_eq_ssp1_transmit(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_SSP1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_ADC0`"]
    #[inline]
    pub fn is_source_eq_adc0(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_ADC0
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_ADC1`"]
    #[inline]
    pub fn is_source_eq_adc1(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_ADC1
    }
    #[doc = "Checks if the value of the field is `SOURCE_EQ_DAC`"]
    #[inline]
    pub fn is_source_eq_dac(&self) -> bool {
        *self == SRCPERIPHERALR::SOURCE_EQ_DAC
    }
}
#[doc = "Possible values of the field `DESTPERIPHERAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DESTPERIPHERALR {
    #[doc = "Destination = SPIFI"]
    DESTINATION_EQ_SPIFI,
    #[doc = "Destination = Timer 0 match 0/UART0 transmit"]
    DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT,
    #[doc = "Destination = Timer 0 match 1/UART0 receive"]
    DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE,
    #[doc = "Destination = Timer 1 match 0/UART1 transmit"]
    DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT,
    #[doc = "Destination = Timer 1 match 1/UART 1 receive"]
    DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE,
    #[doc = "Destination = Timer 2 match 0/UART 2 transmit"]
    DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT,
    #[doc = "Destination = Timer 2 match 1/UART 2 receive"]
    DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE,
    #[doc = "Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0"]
    DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0,
    #[doc = "Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1"]
    DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1,
    #[doc = "Destination = SSP0 receive/I2S channel 0"]
    DESTINATION_EQ_SSP0_RECEIVE_CH0,
    #[doc = "Destination = SSP0 transmit/I2S channel 1"]
    DESTINATION_EQ_SSP0_TRANSMIT_CH1,
    #[doc = "Destination = SSP1 receive"]
    DESTINATION_EQ_SSP1_RECEIVE,
    #[doc = "Destination = SSP1 transmit"]
    DESTINATION_EQ_SSP1_TRANSMIT,
    #[doc = "Destination = ADC0"]
    DESTINATION_EQ_ADC0,
    #[doc = "Destination = ADC1"]
    DESTINATION_EQ_ADC1,
    #[doc = "Destination = DAC"]
    DESTINATION_EQ_DAC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DESTPERIPHERALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DESTPERIPHERALR::DESTINATION_EQ_SPIFI => 0,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT => 1,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE => 2,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT => 3,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE => 4,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT => 5,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE => 6,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0 => 7,
            DESTPERIPHERALR::DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1 => 8,
            DESTPERIPHERALR::DESTINATION_EQ_SSP0_RECEIVE_CH0 => 9,
            DESTPERIPHERALR::DESTINATION_EQ_SSP0_TRANSMIT_CH1 => 10,
            DESTPERIPHERALR::DESTINATION_EQ_SSP1_RECEIVE => 11,
            DESTPERIPHERALR::DESTINATION_EQ_SSP1_TRANSMIT => 12,
            DESTPERIPHERALR::DESTINATION_EQ_ADC0 => 13,
            DESTPERIPHERALR::DESTINATION_EQ_ADC1 => 14,
            DESTPERIPHERALR::DESTINATION_EQ_DAC => 15,
            DESTPERIPHERALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DESTPERIPHERALR {
        match value {
            0 => DESTPERIPHERALR::DESTINATION_EQ_SPIFI,
            1 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT,
            2 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE,
            3 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT,
            4 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE,
            5 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT,
            6 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE,
            7 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0,
            8 => DESTPERIPHERALR::DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1,
            9 => DESTPERIPHERALR::DESTINATION_EQ_SSP0_RECEIVE_CH0,
            10 => DESTPERIPHERALR::DESTINATION_EQ_SSP0_TRANSMIT_CH1,
            11 => DESTPERIPHERALR::DESTINATION_EQ_SSP1_RECEIVE,
            12 => DESTPERIPHERALR::DESTINATION_EQ_SSP1_TRANSMIT,
            13 => DESTPERIPHERALR::DESTINATION_EQ_ADC0,
            14 => DESTPERIPHERALR::DESTINATION_EQ_ADC1,
            15 => DESTPERIPHERALR::DESTINATION_EQ_DAC,
            i => DESTPERIPHERALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_SPIFI`"]
    #[inline]
    pub fn is_destination_eq_spifi(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_SPIFI
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT`"]
    #[inline]
    pub fn is_destination_eq_timer_0_match_0_uart0_transmit(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE`"]
    #[inline]
    pub fn is_destination_eq_timer_0_match_1_uart0_receieve(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT`"]
    #[inline]
    pub fn is_destination_eq_timer_1_match_0_uart1_transmit(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE`"]
    #[inline]
    pub fn is_destination_eq_timer_1_match_1_uart1_receive(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT`"]
    #[inline]
    pub fn is_destination_eq_timer_2_match_0_uart2_transmit(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE`"]
    #[inline]
    pub fn is_destination_eq_timer_2_match_1_uart2_receive(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0`"]
    #[inline]
    pub fn is_destination_eq_timer_3_match_0_uart3_transmit_dma_req_0(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1`"]
    #[inline]
    pub fn is_destination_eq_timer_3_match_1_uart3_receive_dma_req_1(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_SSP0_RECEIVE_CH0`"]
    #[inline]
    pub fn is_destination_eq_ssp0_receive_ch0(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_SSP0_RECEIVE_CH0
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_SSP0_TRANSMIT_CH1`"]
    #[inline]
    pub fn is_destination_eq_ssp0_transmit_ch1(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_SSP0_TRANSMIT_CH1
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_SSP1_RECEIVE`"]
    #[inline]
    pub fn is_destination_eq_ssp1_receive(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_SSP1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_SSP1_TRANSMIT`"]
    #[inline]
    pub fn is_destination_eq_ssp1_transmit(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_SSP1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_ADC0`"]
    #[inline]
    pub fn is_destination_eq_adc0(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_ADC0
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_ADC1`"]
    #[inline]
    pub fn is_destination_eq_adc1(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_ADC1
    }
    #[doc = "Checks if the value of the field is `DESTINATION_EQ_DAC`"]
    #[inline]
    pub fn is_destination_eq_dac(&self) -> bool {
        *self == DESTPERIPHERALR::DESTINATION_EQ_DAC
    }
}
#[doc = "Possible values of the field `FLOWCNTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLOWCNTRLR {
    #[doc = "Memory to memory (DMA control)"]
    MEMORY_TO_MEMORY,
    #[doc = "Memory to peripheral (DMA control)"]
    MEMORY_TO_PERIPHERAL_DMA_CONTROL,
    #[doc = "Peripheral to memory (DMA control)"]
    PERIPHERAL_TO_MEMORY_DMA_CONTROL,
    #[doc = "Source peripheral to destination peripheral (DMA control)"]
    SOURCE_PERIPHERAL_TO_DMA_CONTROL,
    #[doc = "Source peripheral to destination peripheral (destination control)"]
    SOURCE_PERIPHERAL_TO_DEST_CONTROL,
    #[doc = "Memory to peripheral (peripheral control)"]
    MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL,
    #[doc = "Peripheral to memory (peripheral control)"]
    PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL,
    #[doc = "Source peripheral to destination peripheral (source control)"]
    SOURCE_PERIPHERAL_TO_SOURCE_CONTROL,
}
impl FLOWCNTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLOWCNTRLR::MEMORY_TO_MEMORY => 0,
            FLOWCNTRLR::MEMORY_TO_PERIPHERAL_DMA_CONTROL => 1,
            FLOWCNTRLR::PERIPHERAL_TO_MEMORY_DMA_CONTROL => 2,
            FLOWCNTRLR::SOURCE_PERIPHERAL_TO_DMA_CONTROL => 3,
            FLOWCNTRLR::SOURCE_PERIPHERAL_TO_DEST_CONTROL => 4,
            FLOWCNTRLR::MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL => 5,
            FLOWCNTRLR::PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL => 6,
            FLOWCNTRLR::SOURCE_PERIPHERAL_TO_SOURCE_CONTROL => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLOWCNTRLR {
        match value {
            0 => FLOWCNTRLR::MEMORY_TO_MEMORY,
            1 => FLOWCNTRLR::MEMORY_TO_PERIPHERAL_DMA_CONTROL,
            2 => FLOWCNTRLR::PERIPHERAL_TO_MEMORY_DMA_CONTROL,
            3 => FLOWCNTRLR::SOURCE_PERIPHERAL_TO_DMA_CONTROL,
            4 => FLOWCNTRLR::SOURCE_PERIPHERAL_TO_DEST_CONTROL,
            5 => FLOWCNTRLR::MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL,
            6 => FLOWCNTRLR::PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL,
            7 => FLOWCNTRLR::SOURCE_PERIPHERAL_TO_SOURCE_CONTROL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEMORY_TO_MEMORY`"]
    #[inline]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == FLOWCNTRLR::MEMORY_TO_MEMORY
    }
    #[doc = "Checks if the value of the field is `MEMORY_TO_PERIPHERAL_DMA_CONTROL`"]
    #[inline]
    pub fn is_memory_to_peripheral_dma_control(&self) -> bool {
        *self == FLOWCNTRLR::MEMORY_TO_PERIPHERAL_DMA_CONTROL
    }
    #[doc = "Checks if the value of the field is `PERIPHERAL_TO_MEMORY_DMA_CONTROL`"]
    #[inline]
    pub fn is_peripheral_to_memory_dma_control(&self) -> bool {
        *self == FLOWCNTRLR::PERIPHERAL_TO_MEMORY_DMA_CONTROL
    }
    #[doc = "Checks if the value of the field is `SOURCE_PERIPHERAL_TO_DMA_CONTROL`"]
    #[inline]
    pub fn is_source_peripheral_to_dma_control(&self) -> bool {
        *self == FLOWCNTRLR::SOURCE_PERIPHERAL_TO_DMA_CONTROL
    }
    #[doc = "Checks if the value of the field is `SOURCE_PERIPHERAL_TO_DEST_CONTROL`"]
    #[inline]
    pub fn is_source_peripheral_to_dest_control(&self) -> bool {
        *self == FLOWCNTRLR::SOURCE_PERIPHERAL_TO_DEST_CONTROL
    }
    #[doc = "Checks if the value of the field is `MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL`"]
    #[inline]
    pub fn is_memory_to_peripheral_peripheral_control(&self) -> bool {
        *self == FLOWCNTRLR::MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL
    }
    #[doc = "Checks if the value of the field is `PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL`"]
    #[inline]
    pub fn is_peripheral_to_memory_peripheral_control(&self) -> bool {
        *self == FLOWCNTRLR::PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL
    }
    #[doc = "Checks if the value of the field is `SOURCE_PERIPHERAL_TO_SOURCE_CONTROL`"]
    #[inline]
    pub fn is_source_peripheral_to_source_control(&self) -> bool {
        *self == FLOWCNTRLR::SOURCE_PERIPHERAL_TO_SOURCE_CONTROL
    }
}
#[doc = r" Value of the field"]
pub struct IER {
    bits: bool,
}
impl IER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ITCR {
    bits: bool,
}
impl ITCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LR {
    bits: bool,
}
impl LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AR {
    bits: bool,
}
impl AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `H`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HR {
    #[doc = "Enable DMA requests."]
    ENABLE_DMA_REQUESTS_,
    #[doc = "Ignore further source DMA requests."]
    IGNORE_FURTHER_SOURC,
}
impl HR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HR::ENABLE_DMA_REQUESTS_ => false,
            HR::IGNORE_FURTHER_SOURC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HR {
        match value {
            false => HR::ENABLE_DMA_REQUESTS_,
            true => HR::IGNORE_FURTHER_SOURC,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_DMA_REQUESTS_`"]
    #[inline]
    pub fn is_enable_dma_requests_(&self) -> bool {
        *self == HR::ENABLE_DMA_REQUESTS_
    }
    #[doc = "Checks if the value of the field is `IGNORE_FURTHER_SOURC`"]
    #[inline]
    pub fn is_ignore_further_sourc(&self) -> bool {
        *self == HR::IGNORE_FURTHER_SOURC
    }
}
#[doc = "Values that can be written to the field `E`"]
pub enum EW {
    #[doc = "Channel disabled."]
    CHANNEL_DISABLED_,
    #[doc = "Channel enabled."]
    CHANNEL_ENABLED_,
}
impl EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EW::CHANNEL_DISABLED_ => false,
            EW::CHANNEL_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EW<'a> {
    w: &'a mut W,
}
impl<'a> _EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel disabled."]
    #[inline]
    pub fn channel_disabled_(self) -> &'a mut W {
        self.variant(EW::CHANNEL_DISABLED_)
    }
    #[doc = "Channel enabled."]
    #[inline]
    pub fn channel_enabled_(self) -> &'a mut W {
        self.variant(EW::CHANNEL_ENABLED_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRCPERIPHERAL`"]
pub enum SRCPERIPHERALW {
    #[doc = "Source = SPIFI"]
    SOURCE_EQ_SPIFI,
    #[doc = "Source = Timer 0 match 0/UART0 transmit"]
    SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT,
    #[doc = "Source = Timer 0 match 1/UART0 receive"]
    SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE,
    #[doc = "Source = Timer 1 match 0/UART1 transmit"]
    SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT,
    #[doc = "Source = Timer 1 match 1/UART 1 receive"]
    SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE,
    #[doc = "Source = Timer 2 match 0/UART 2 transmit"]
    SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT,
    #[doc = "Source = Timer 2 match 1/UART 2 receive"]
    SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE,
    #[doc = "Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0"]
    SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0,
    #[doc = "Source = Timer 3 match 1/UART3 receive/SCT DMA request 1"]
    SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1,
    #[doc = "Source = SSP0 receive/I2S channel 0"]
    SOURCE_EQ_SSP0_RECEIVE_CH0,
    #[doc = "Source = SSP0 transmit/I2S channel 1"]
    SOURCE_EQ_SSP0_TRANSMIT_CH1,
    #[doc = "Source = SSP1 receive"]
    SOURCE_EQ_SSP1_RECEIVE,
    #[doc = "Source = SSP1 transmit"]
    SOURCE_EQ_SSP1_TRANSMIT,
    #[doc = "Source = ADC0"]
    SOURCE_EQ_ADC0,
    #[doc = "Source = ADC1"]
    SOURCE_EQ_ADC1,
    #[doc = "Source = DAC"]
    SOURCE_EQ_DAC,
}
impl SRCPERIPHERALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCPERIPHERALW::SOURCE_EQ_SPIFI => 0,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT => 1,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE => 2,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT => 3,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE => 4,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT => 5,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE => 6,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0 => 7,
            SRCPERIPHERALW::SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1 => 8,
            SRCPERIPHERALW::SOURCE_EQ_SSP0_RECEIVE_CH0 => 9,
            SRCPERIPHERALW::SOURCE_EQ_SSP0_TRANSMIT_CH1 => 10,
            SRCPERIPHERALW::SOURCE_EQ_SSP1_RECEIVE => 11,
            SRCPERIPHERALW::SOURCE_EQ_SSP1_TRANSMIT => 12,
            SRCPERIPHERALW::SOURCE_EQ_ADC0 => 13,
            SRCPERIPHERALW::SOURCE_EQ_ADC1 => 14,
            SRCPERIPHERALW::SOURCE_EQ_DAC => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCPERIPHERALW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCPERIPHERALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCPERIPHERALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Source = SPIFI"]
    #[inline]
    pub fn source_eq_spifi(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_SPIFI)
    }
    #[doc = "Source = Timer 0 match 0/UART0 transmit"]
    #[inline]
    pub fn source_eq_timer_0_match_0_uart0_transmit(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT)
    }
    #[doc = "Source = Timer 0 match 1/UART0 receive"]
    #[inline]
    pub fn source_eq_timer_0_match_1_uart0_receive(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_0_MATCH_1_UART0_RECEIVE)
    }
    #[doc = "Source = Timer 1 match 0/UART1 transmit"]
    #[inline]
    pub fn source_eq_timer_1_match_0_uart1_transmit(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT)
    }
    #[doc = "Source = Timer 1 match 1/UART 1 receive"]
    #[inline]
    pub fn source_eq_timer_1_match_1_uart1_receive(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_1_MATCH_1_UART1_RECEIVE)
    }
    #[doc = "Source = Timer 2 match 0/UART 2 transmit"]
    #[inline]
    pub fn source_eq_timer_2_match_0_uart2_transmit(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT)
    }
    #[doc = "Source = Timer 2 match 1/UART 2 receive"]
    #[inline]
    pub fn source_eq_timer_2_match_1_uart2_receive(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_2_MATCH_1_UART2_RECEIVE)
    }
    #[doc = "Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0"]
    #[inline]
    pub fn source_eq_timer_3_match_0_uart3_receive_dma_req_0(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_3_MATCH_0_UART3_RECEIVE_DMA_REQ_0)
    }
    #[doc = "Source = Timer 3 match 1/UART3 receive/SCT DMA request 1"]
    #[inline]
    pub fn source_eq_timer_3_match_1_uart3_receive_dma_req_1(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1)
    }
    #[doc = "Source = SSP0 receive/I2S channel 0"]
    #[inline]
    pub fn source_eq_ssp0_receive_ch0(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_SSP0_RECEIVE_CH0)
    }
    #[doc = "Source = SSP0 transmit/I2S channel 1"]
    #[inline]
    pub fn source_eq_ssp0_transmit_ch1(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_SSP0_TRANSMIT_CH1)
    }
    #[doc = "Source = SSP1 receive"]
    #[inline]
    pub fn source_eq_ssp1_receive(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_SSP1_RECEIVE)
    }
    #[doc = "Source = SSP1 transmit"]
    #[inline]
    pub fn source_eq_ssp1_transmit(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_SSP1_TRANSMIT)
    }
    #[doc = "Source = ADC0"]
    #[inline]
    pub fn source_eq_adc0(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_ADC0)
    }
    #[doc = "Source = ADC1"]
    #[inline]
    pub fn source_eq_adc1(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_ADC1)
    }
    #[doc = "Source = DAC"]
    #[inline]
    pub fn source_eq_dac(self) -> &'a mut W {
        self.variant(SRCPERIPHERALW::SOURCE_EQ_DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DESTPERIPHERAL`"]
pub enum DESTPERIPHERALW {
    #[doc = "Destination = SPIFI"]
    DESTINATION_EQ_SPIFI,
    #[doc = "Destination = Timer 0 match 0/UART0 transmit"]
    DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT,
    #[doc = "Destination = Timer 0 match 1/UART0 receive"]
    DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE,
    #[doc = "Destination = Timer 1 match 0/UART1 transmit"]
    DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT,
    #[doc = "Destination = Timer 1 match 1/UART 1 receive"]
    DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE,
    #[doc = "Destination = Timer 2 match 0/UART 2 transmit"]
    DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT,
    #[doc = "Destination = Timer 2 match 1/UART 2 receive"]
    DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE,
    #[doc = "Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0"]
    DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0,
    #[doc = "Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1"]
    DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1,
    #[doc = "Destination = SSP0 receive/I2S channel 0"]
    DESTINATION_EQ_SSP0_RECEIVE_CH0,
    #[doc = "Destination = SSP0 transmit/I2S channel 1"]
    DESTINATION_EQ_SSP0_TRANSMIT_CH1,
    #[doc = "Destination = SSP1 receive"]
    DESTINATION_EQ_SSP1_RECEIVE,
    #[doc = "Destination = SSP1 transmit"]
    DESTINATION_EQ_SSP1_TRANSMIT,
    #[doc = "Destination = ADC0"]
    DESTINATION_EQ_ADC0,
    #[doc = "Destination = ADC1"]
    DESTINATION_EQ_ADC1,
    #[doc = "Destination = DAC"]
    DESTINATION_EQ_DAC,
}
impl DESTPERIPHERALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DESTPERIPHERALW::DESTINATION_EQ_SPIFI => 0,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT => 1,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE => 2,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT => 3,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE => 4,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT => 5,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE => 6,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0 => 7,
            DESTPERIPHERALW::DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1 => 8,
            DESTPERIPHERALW::DESTINATION_EQ_SSP0_RECEIVE_CH0 => 9,
            DESTPERIPHERALW::DESTINATION_EQ_SSP0_TRANSMIT_CH1 => 10,
            DESTPERIPHERALW::DESTINATION_EQ_SSP1_RECEIVE => 11,
            DESTPERIPHERALW::DESTINATION_EQ_SSP1_TRANSMIT => 12,
            DESTPERIPHERALW::DESTINATION_EQ_ADC0 => 13,
            DESTPERIPHERALW::DESTINATION_EQ_ADC1 => 14,
            DESTPERIPHERALW::DESTINATION_EQ_DAC => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DESTPERIPHERALW<'a> {
    w: &'a mut W,
}
impl<'a> _DESTPERIPHERALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DESTPERIPHERALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Destination = SPIFI"]
    #[inline]
    pub fn destination_eq_spifi(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_SPIFI)
    }
    #[doc = "Destination = Timer 0 match 0/UART0 transmit"]
    #[inline]
    pub fn destination_eq_timer_0_match_0_uart0_transmit(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_0_MATCH_0_UART0_TRANSMIT)
    }
    #[doc = "Destination = Timer 0 match 1/UART0 receive"]
    #[inline]
    pub fn destination_eq_timer_0_match_1_uart0_receieve(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_0_MATCH_1_UART0_RECEIEVE)
    }
    #[doc = "Destination = Timer 1 match 0/UART1 transmit"]
    #[inline]
    pub fn destination_eq_timer_1_match_0_uart1_transmit(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_1_MATCH_0_UART1_TRANSMIT)
    }
    #[doc = "Destination = Timer 1 match 1/UART 1 receive"]
    #[inline]
    pub fn destination_eq_timer_1_match_1_uart1_receive(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_1_MATCH_1_UART1_RECEIVE)
    }
    #[doc = "Destination = Timer 2 match 0/UART 2 transmit"]
    #[inline]
    pub fn destination_eq_timer_2_match_0_uart2_transmit(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_2_MATCH_0_UART2_TRANSMIT)
    }
    #[doc = "Destination = Timer 2 match 1/UART 2 receive"]
    #[inline]
    pub fn destination_eq_timer_2_match_1_uart2_receive(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_2_MATCH_1_UART2_RECEIVE)
    }
    #[doc = "Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0"]
    #[inline]
    pub fn destination_eq_timer_3_match_0_uart3_transmit_dma_req_0(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_3_MATCH_0_UART3_TRANSMIT_DMA_REQ_0)
    }
    #[doc = "Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1"]
    #[inline]
    pub fn destination_eq_timer_3_match_1_uart3_receive_dma_req_1(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_TIMER_3_MATCH_1_UART3_RECEIVE_DMA_REQ_1)
    }
    #[doc = "Destination = SSP0 receive/I2S channel 0"]
    #[inline]
    pub fn destination_eq_ssp0_receive_ch0(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_SSP0_RECEIVE_CH0)
    }
    #[doc = "Destination = SSP0 transmit/I2S channel 1"]
    #[inline]
    pub fn destination_eq_ssp0_transmit_ch1(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_SSP0_TRANSMIT_CH1)
    }
    #[doc = "Destination = SSP1 receive"]
    #[inline]
    pub fn destination_eq_ssp1_receive(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_SSP1_RECEIVE)
    }
    #[doc = "Destination = SSP1 transmit"]
    #[inline]
    pub fn destination_eq_ssp1_transmit(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_SSP1_TRANSMIT)
    }
    #[doc = "Destination = ADC0"]
    #[inline]
    pub fn destination_eq_adc0(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_ADC0)
    }
    #[doc = "Destination = ADC1"]
    #[inline]
    pub fn destination_eq_adc1(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_ADC1)
    }
    #[doc = "Destination = DAC"]
    #[inline]
    pub fn destination_eq_dac(self) -> &'a mut W {
        self.variant(DESTPERIPHERALW::DESTINATION_EQ_DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLOWCNTRL`"]
pub enum FLOWCNTRLW {
    #[doc = "Memory to memory (DMA control)"]
    MEMORY_TO_MEMORY,
    #[doc = "Memory to peripheral (DMA control)"]
    MEMORY_TO_PERIPHERAL_DMA_CONTROL,
    #[doc = "Peripheral to memory (DMA control)"]
    PERIPHERAL_TO_MEMORY_DMA_CONTROL,
    #[doc = "Source peripheral to destination peripheral (DMA control)"]
    SOURCE_PERIPHERAL_TO_DMA_CONTROL,
    #[doc = "Source peripheral to destination peripheral (destination control)"]
    SOURCE_PERIPHERAL_TO_DEST_CONTROL,
    #[doc = "Memory to peripheral (peripheral control)"]
    MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL,
    #[doc = "Peripheral to memory (peripheral control)"]
    PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL,
    #[doc = "Source peripheral to destination peripheral (source control)"]
    SOURCE_PERIPHERAL_TO_SOURCE_CONTROL,
}
impl FLOWCNTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLOWCNTRLW::MEMORY_TO_MEMORY => 0,
            FLOWCNTRLW::MEMORY_TO_PERIPHERAL_DMA_CONTROL => 1,
            FLOWCNTRLW::PERIPHERAL_TO_MEMORY_DMA_CONTROL => 2,
            FLOWCNTRLW::SOURCE_PERIPHERAL_TO_DMA_CONTROL => 3,
            FLOWCNTRLW::SOURCE_PERIPHERAL_TO_DEST_CONTROL => 4,
            FLOWCNTRLW::MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL => 5,
            FLOWCNTRLW::PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL => 6,
            FLOWCNTRLW::SOURCE_PERIPHERAL_TO_SOURCE_CONTROL => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLOWCNTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLOWCNTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLOWCNTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Memory to memory (DMA control)"]
    #[inline]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::MEMORY_TO_MEMORY)
    }
    #[doc = "Memory to peripheral (DMA control)"]
    #[inline]
    pub fn memory_to_peripheral_dma_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::MEMORY_TO_PERIPHERAL_DMA_CONTROL)
    }
    #[doc = "Peripheral to memory (DMA control)"]
    #[inline]
    pub fn peripheral_to_memory_dma_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::PERIPHERAL_TO_MEMORY_DMA_CONTROL)
    }
    #[doc = "Source peripheral to destination peripheral (DMA control)"]
    #[inline]
    pub fn source_peripheral_to_dma_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::SOURCE_PERIPHERAL_TO_DMA_CONTROL)
    }
    #[doc = "Source peripheral to destination peripheral (destination control)"]
    #[inline]
    pub fn source_peripheral_to_dest_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::SOURCE_PERIPHERAL_TO_DEST_CONTROL)
    }
    #[doc = "Memory to peripheral (peripheral control)"]
    #[inline]
    pub fn memory_to_peripheral_peripheral_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::MEMORY_TO_PERIPHERAL_PERIPHERAL_CONTROL)
    }
    #[doc = "Peripheral to memory (peripheral control)"]
    #[inline]
    pub fn peripheral_to_memory_peripheral_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::PERIPHERAL_TO_MEMORY_PERIPHERAL_CONTROL)
    }
    #[doc = "Source peripheral to destination peripheral (source control)"]
    #[inline]
    pub fn source_peripheral_to_source_control(self) -> &'a mut W {
        self.variant(FLOWCNTRLW::SOURCE_PERIPHERAL_TO_SOURCE_CONTROL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITCW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LW<'a> {
    w: &'a mut W,
}
impl<'a> _LW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AW<'a> {
    w: &'a mut W,
}
impl<'a> _AW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `H`"]
pub enum HW {
    #[doc = "Enable DMA requests."]
    ENABLE_DMA_REQUESTS_,
    #[doc = "Ignore further source DMA requests."]
    IGNORE_FURTHER_SOURC,
}
impl HW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HW::ENABLE_DMA_REQUESTS_ => false,
            HW::IGNORE_FURTHER_SOURC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HW<'a> {
    w: &'a mut W,
}
impl<'a> _HW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable DMA requests."]
    #[inline]
    pub fn enable_dma_requests_(self) -> &'a mut W {
        self.variant(HW::ENABLE_DMA_REQUESTS_)
    }
    #[doc = "Ignore further source DMA requests."]
    #[inline]
    pub fn ignore_further_sourc(self) -> &'a mut W {
        self.variant(HW::IGNORE_FURTHER_SOURC)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: The Channel Enable bit status can also be found by reading the EnbldChns Register. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
    #[inline]
    pub fn e(&self) -> ER {
        ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:5 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 136 for details."]
    #[inline]
    pub fn srcperipheral(&self) -> SRCPERIPHERALR {
        SRCPERIPHERALR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:10 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 136 for details."]
    #[inline]
    pub fn destperipheral(&self) -> DESTPERIPHERALR {
        DESTPERIPHERALR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Flow control and transfer type. This value indicates the flow controller and transfer type. The flow controller can be the DMA Controller, the source peripheral, or the destination peripheral. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Refer to Table 157 for the encoding of this field."]
    #[inline]
    pub fn flowcntrl(&self) -> FLOWCNTRLR {
        FLOWCNTRLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline]
    pub fn ie(&self) -> IER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IER { bits }
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline]
    pub fn itc(&self) -> ITCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITCR { bits }
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers."]
    #[inline]
    pub fn l(&self) -> LR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LR { bits }
    }
    #[doc = "Bit 17 - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
    #[inline]
    pub fn a(&self) -> AR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AR { bits }
    }
    #[doc = "Bit 18 - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline]
    pub fn h(&self) -> HR {
        HR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: The Channel Enable bit status can also be found by reading the EnbldChns Register. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
    #[inline]
    pub fn e(&mut self) -> _EW {
        _EW { w: self }
    }
    #[doc = "Bits 1:5 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 136 for details."]
    #[inline]
    pub fn srcperipheral(&mut self) -> _SRCPERIPHERALW {
        _SRCPERIPHERALW { w: self }
    }
    #[doc = "Bits 6:10 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 136 for details."]
    #[inline]
    pub fn destperipheral(&mut self) -> _DESTPERIPHERALW {
        _DESTPERIPHERALW { w: self }
    }
    #[doc = "Bits 11:13 - Flow control and transfer type. This value indicates the flow controller and transfer type. The flow controller can be the DMA Controller, the source peripheral, or the destination peripheral. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Refer to Table 157 for the encoding of this field."]
    #[inline]
    pub fn flowcntrl(&mut self) -> _FLOWCNTRLW {
        _FLOWCNTRLW { w: self }
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline]
    pub fn ie(&mut self) -> _IEW {
        _IEW { w: self }
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline]
    pub fn itc(&mut self) -> _ITCW {
        _ITCW { w: self }
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers."]
    #[inline]
    pub fn l(&mut self) -> _LW {
        _LW { w: self }
    }
    #[doc = "Bit 17 - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
    #[inline]
    pub fn a(&mut self) -> _AW {
        _AW { w: self }
    }
    #[doc = "Bit 18 - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline]
    pub fn h(&mut self) -> _HW {
        _HW { w: self }
    }
}
