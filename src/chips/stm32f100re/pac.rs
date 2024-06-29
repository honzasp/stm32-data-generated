

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - WWDG"]
WWDG = 0 , # [doc = "1 - PVD"]
PVD = 1 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FLASH"]
FLASH = 4 , # [doc = "5 - RCC"]
RCC = 5 , # [doc = "6 - EXTI0"]
EXTI0 = 6 , # [doc = "7 - EXTI1"]
EXTI1 = 7 , # [doc = "8 - EXTI2"]
EXTI2 = 8 , # [doc = "9 - EXTI3"]
EXTI3 = 9 , # [doc = "10 - EXTI4"]
EXTI4 = 10 , # [doc = "11 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 11 , # [doc = "12 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 12 , # [doc = "13 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 13 , # [doc = "14 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 14 , # [doc = "15 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 15 , # [doc = "16 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 16 , # [doc = "17 - DMA1_CHANNEL7"]
DMA1_CHANNEL7 = 17 , # [doc = "18 - ADC1"]
ADC1 = 18 , # [doc = "23 - EXTI9_5"]
EXTI9_5 = 23 , # [doc = "24 - TIM1_BRK_TIM15"]
TIM1_BRK_TIM15 = 24 , # [doc = "25 - TIM1_UP_TIM16"]
TIM1_UP_TIM16 = 25 , # [doc = "26 - TIM1_TRG_COM_TIM17"]
TIM1_TRG_COM_TIM17 = 26 , # [doc = "27 - TIM1_CC"]
TIM1_CC = 27 , # [doc = "28 - TIM2"]
TIM2 = 28 , # [doc = "29 - TIM3"]
TIM3 = 29 , # [doc = "30 - TIM4"]
TIM4 = 30 , # [doc = "31 - I2C1_EV"]
I2C1_EV = 31 , # [doc = "32 - I2C1_ER"]
I2C1_ER = 32 , # [doc = "33 - I2C2_EV"]
I2C2_EV = 33 , # [doc = "34 - I2C2_ER"]
I2C2_ER = 34 , # [doc = "35 - SPI1"]
SPI1 = 35 , # [doc = "36 - SPI2"]
SPI2 = 36 , # [doc = "37 - USART1"]
USART1 = 37 , # [doc = "38 - USART2"]
USART2 = 38 , # [doc = "39 - USART3"]
USART3 = 39 , # [doc = "40 - EXTI15_10"]
EXTI15_10 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - CEC"]
CEC = 42 , # [doc = "43 - TIM12"]
TIM12 = 43 , # [doc = "44 - TIM13"]
TIM13 = 44 , # [doc = "45 - TIM14"]
TIM14 = 45 , # [doc = "50 - TIM5"]
TIM5 = 50 , # [doc = "51 - SPI3"]
SPI3 = 51 , # [doc = "52 - UART4"]
UART4 = 52 , # [doc = "53 - UART5"]
UART5 = 53 , # [doc = "54 - TIM6_DAC"]
TIM6_DAC = 54 , # [doc = "55 - TIM7"]
TIM7 = 55 , # [doc = "56 - DMA2_CHANNEL1"]
DMA2_CHANNEL1 = 56 , # [doc = "57 - DMA2_CHANNEL2"]
DMA2_CHANNEL2 = 57 , # [doc = "58 - DMA2_CHANNEL3"]
DMA2_CHANNEL3 = 58 , # [doc = "59 - DMA2_CHANNEL4"]
DMA2_CHANNEL4 = 59 , # [doc = "60 - DMA2_CHANNEL5"]
DMA2_CHANNEL5 = 60 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { extern "C" { fn WWDG () ; fn PVD () ; fn TAMPER () ; fn RTC () ; fn FLASH () ; fn RCC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn ADC1 () ; fn EXTI9_5 () ; fn TIM1_BRK_TIM15 () ; fn TIM1_UP_TIM16 () ; fn TIM1_TRG_COM_TIM17 () ; fn TIM1_CC () ; fn TIM2 () ; fn TIM3 () ; fn TIM4 () ; fn I2C1_EV () ; fn I2C1_ER () ; fn I2C2_EV () ; fn I2C2_ER () ; fn SPI1 () ; fn SPI2 () ; fn USART1 () ; fn USART2 () ; fn USART3 () ; fn EXTI15_10 () ; fn RTC_ALARM () ; fn CEC () ; fn TIM12 () ; fn TIM13 () ; fn TIM14 () ; fn TIM5 () ; fn SPI3 () ; fn UART4 () ; fn UART5 () ; fn TIM6_DAC () ; fn TIM7 () ; fn DMA2_CHANNEL1 () ; fn DMA2_CHANNEL2 () ; fn DMA2_CHANNEL3 () ; fn DMA2_CHANNEL4 () ; fn DMA2_CHANNEL5 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"]
# [no_mangle]
pub static __INTERRUPTS : [Vector ; 61]
= [Vector { _handler : WWDG } , Vector { _handler : PVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FLASH } , Vector { _handler : RCC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _handler : ADC1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI9_5 } , Vector { _handler : TIM1_BRK_TIM15 } , Vector { _handler : TIM1_UP_TIM16 } , Vector { _handler : TIM1_TRG_COM_TIM17 } , Vector { _handler : TIM1_CC } , Vector { _handler : TIM2 } , Vector { _handler : TIM3 } , Vector { _handler : TIM4 } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : SPI1 } , Vector { _handler : SPI2 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : USART3 } , Vector { _handler : EXTI15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : CEC } , Vector { _handler : TIM12 } , Vector { _handler : TIM13 } , Vector { _handler : TIM14 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIM5 } , Vector { _handler : SPI3 } , Vector { _handler : UART4 } , Vector { _handler : UART5 } , Vector { _handler : TIM6_DAC } , Vector { _handler : TIM7 } , Vector { _handler : DMA2_CHANNEL1 } , Vector { _handler : DMA2_CHANNEL2 } , Vector { _handler : DMA2_CHANNEL3 } , Vector { _handler : DMA2_CHANNEL4 } , Vector { _handler : DMA2_CHANNEL5 } ,]
; } pub const UID : uid :: Uid = unsafe { uid :: Uid :: from_ptr (0x1fff_f7e8usize as _) } ; pub const TIM2 : timer :: Tim4ch = unsafe { timer :: Tim4ch :: from_ptr (0x4000_0000usize as _) } ; pub const TIM3 : timer :: Tim4ch = unsafe { timer :: Tim4ch :: from_ptr (0x4000_0400usize as _) } ; pub const TIM4 : timer :: Tim4ch = unsafe { timer :: Tim4ch :: from_ptr (0x4000_0800usize as _) } ; pub const TIM5 : timer :: Tim4ch = unsafe { timer :: Tim4ch :: from_ptr (0x4000_0c00usize as _) } ; pub const TIM6 : timer :: TimBasic = unsafe { timer :: TimBasic :: from_ptr (0x4000_1000usize as _) } ; pub const TIM7 : timer :: TimBasic = unsafe { timer :: TimBasic :: from_ptr (0x4000_1400usize as _) } ; pub const TIM12 : timer :: Tim2ch = unsafe { timer :: Tim2ch :: from_ptr (0x4000_1800usize as _) } ; pub const TIM13 : timer :: Tim1ch = unsafe { timer :: Tim1ch :: from_ptr (0x4000_1c00usize as _) } ; pub const TIM14 : timer :: Tim1ch = unsafe { timer :: Tim1ch :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : rtc :: Rtc = unsafe { rtc :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDG : wwdg :: Wwdg = unsafe { wwdg :: Wwdg :: from_ptr (0x4000_2c00usize as _) } ; pub const IWDG : iwdg :: Iwdg = unsafe { iwdg :: Iwdg :: from_ptr (0x4000_3000usize as _) } ; pub const SPI2 : spi :: Spi = unsafe { spi :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI3 : spi :: Spi = unsafe { spi :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART2 : usart :: Usart = unsafe { usart :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART3 : usart :: Usart = unsafe { usart :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART4 : * mut () = 0x4000_4c00usize as _ ; pub const UART5 : * mut () = 0x4000_5000usize as _ ; pub const I2C1 : i2c :: I2c = unsafe { i2c :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C2 : i2c :: I2c = unsafe { i2c :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : bkp :: Bkp = unsafe { bkp :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PWR : pwr :: Pwr = unsafe { pwr :: Pwr :: from_ptr (0x4000_7000usize as _) } ; pub const DAC1 : dac :: Dac = unsafe { dac :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CEC : cec :: Cec = unsafe { cec :: Cec :: from_ptr (0x4000_7800usize as _) } ; pub const AFIO : afio :: Afio = unsafe { afio :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : exti :: Exti = unsafe { exti :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x4001_2000usize as _) } ; pub const ADC1 : adc :: Adc = unsafe { adc :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1_COMMON : * mut () = 0x4001_2400usize as _ ; pub const TIM1 : timer :: TimAdv4ch = unsafe { timer :: TimAdv4ch :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI1 : spi :: Spi = unsafe { spi :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART1 : usart :: Usart = unsafe { usart :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIM15 : timer :: TimAdv2ch = unsafe { timer :: TimAdv2ch :: from_ptr (0x4001_4000usize as _) } ; pub const TIM16 : timer :: TimAdv1ch = unsafe { timer :: TimAdv1ch :: from_ptr (0x4001_4400usize as _) } ; pub const TIM17 : timer :: TimAdv1ch = unsafe { timer :: TimAdv1ch :: from_ptr (0x4001_4800usize as _) } ; pub const DMA1 : bdma :: Dma = unsafe { bdma :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA2 : bdma :: Dma = unsafe { bdma :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const RCC : rcc :: Rcc = unsafe { rcc :: Rcc :: from_ptr (0x4002_1000usize as _) } ; pub const FLASH : flash :: Flash = unsafe { flash :: Flash :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : crc :: Crc = unsafe { crc :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const DBGMCU : dbgmcu :: Dbgmcu = unsafe { dbgmcu :: Dbgmcu :: from_ptr (0xe004_2000usize as _) } ; # [doc = r" Number available in the NVIC for configuring priority"]
# [cfg (feature = "rt")]
pub const NVIC_PRIO_BITS : u8 = 4 ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;pub fn GPIO(n: usize) -> gpio::Gpio {
            unsafe { gpio::Gpio::from_ptr((1073809408 + 1024*n) as _) }
        }#[path="../../peripherals/adc_f1.rs"] pub mod adc;
#[path="../../peripherals/afio_f1.rs"] pub mod afio;
#[path="../../peripherals/bdma_v1.rs"] pub mod bdma;
#[path="../../peripherals/bkp_v1.rs"] pub mod bkp;
#[path="../../peripherals/cec_v1.rs"] pub mod cec;
#[path="../../peripherals/crc_v1.rs"] pub mod crc;
#[path="../../peripherals/dac_v2.rs"] pub mod dac;
#[path="../../peripherals/dbgmcu_f1.rs"] pub mod dbgmcu;
#[path="../../peripherals/exti_v1.rs"] pub mod exti;
#[path="../../peripherals/flash_f1.rs"] pub mod flash;
#[path="../../peripherals/gpio_v1.rs"] pub mod gpio;
#[path="../../peripherals/i2c_v1.rs"] pub mod i2c;
#[path="../../peripherals/iwdg_v1.rs"] pub mod iwdg;
#[path="../../peripherals/pwr_f1.rs"] pub mod pwr;
#[path="../../peripherals/rcc_f100.rs"] pub mod rcc;
#[path="../../peripherals/rtc_v1.rs"] pub mod rtc;
#[path="../../peripherals/spi_f1.rs"] pub mod spi;
#[path="../../peripherals/timer_v1.rs"] pub mod timer;
#[path="../../peripherals/uid_v1.rs"] pub mod uid;
#[path="../../peripherals/usart_v1.rs"] pub mod usart;
#[path="../../peripherals/wwdg_v1.rs"] pub mod wwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 524288;
pub const WRITE_SIZE: usize = 4;
