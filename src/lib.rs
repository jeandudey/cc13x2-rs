#![doc = "Peripheral access API for CC2652 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![allow(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        match *self {}
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
pub struct AUX_ADI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ADI4 {}
#[cfg(feature = "aux_adi4")]
impl AUX_ADI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_adi4::RegisterBlock {
        0x400c_b000 as *const _
    }
}
#[cfg(feature = "aux_adi4")]
impl Deref for AUX_ADI4 {
    type Target = aux_adi4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_ADI4::ptr() }
    }
}
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
#[cfg(feature = "aux_adi4")]
pub mod aux_adi4;
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_BATMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_BATMON {}
#[cfg(feature = "aon_batmon")]
impl AON_BATMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_batmon::RegisterBlock {
        0x4009_5000 as *const _
    }
}
#[cfg(feature = "aon_batmon")]
impl Deref for AON_BATMON {
    type Target = aon_batmon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_BATMON::ptr() }
    }
}
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
#[cfg(feature = "aon_batmon")]
pub mod aon_batmon;
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_EVENT {}
#[cfg(feature = "aon_event")]
impl AON_EVENT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_event::RegisterBlock {
        0x4009_3000 as *const _
    }
}
#[cfg(feature = "aon_event")]
impl Deref for AON_EVENT {
    type Target = aon_event::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_EVENT::ptr() }
    }
}
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
#[cfg(feature = "aon_event")]
pub mod aon_event;
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_IOC {}
#[cfg(feature = "aon_ioc")]
impl AON_IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_ioc::RegisterBlock {
        0x4009_4000 as *const _
    }
}
#[cfg(feature = "aon_ioc")]
impl Deref for AON_IOC {
    type Target = aon_ioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_IOC::ptr() }
    }
}
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
#[cfg(feature = "aon_ioc")]
pub mod aon_ioc;
#[doc = "This component control the Power Management controller residing in the AON domain. Note: This module is only supporting 32 bit Read Write access from MCU"]
pub struct AON_PMCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_PMCTL {}
#[cfg(feature = "aon_pmctl")]
impl AON_PMCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_pmctl::RegisterBlock {
        0x4009_0000 as *const _
    }
}
#[cfg(feature = "aon_pmctl")]
impl Deref for AON_PMCTL {
    type Target = aon_pmctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_PMCTL::ptr() }
    }
}
#[doc = "This component control the Power Management controller residing in the AON domain. Note: This module is only supporting 32 bit Read Write access from MCU"]
#[cfg(feature = "aon_pmctl")]
pub mod aon_pmctl;
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
pub struct AON_RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_RTC {}
#[cfg(feature = "aon_rtc")]
impl AON_RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_rtc::RegisterBlock {
        0x4009_2000 as *const _
    }
}
#[cfg(feature = "aon_rtc")]
impl Deref for AON_RTC {
    type Target = aon_rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_RTC::ptr() }
    }
}
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
#[cfg(feature = "aon_rtc")]
pub mod aon_rtc;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO0 {}
#[cfg(feature = "aux_aiodio0")]
impl AUX_AIODIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio0::RegisterBlock {
        0x400c_c000 as *const _
    }
}
#[cfg(feature = "aux_aiodio0")]
impl Deref for AUX_AIODIO0 {
    type Target = aux_aiodio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO0::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
#[cfg(feature = "aux_aiodio0")]
pub mod aux_aiodio0;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO1 {}
#[cfg(feature = "aux_aiodio1")]
impl AUX_AIODIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio1::RegisterBlock {
        0x400c_d000 as *const _
    }
}
#[cfg(feature = "aux_aiodio1")]
impl Deref for AUX_AIODIO1 {
    type Target = aux_aiodio1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO1::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
#[cfg(feature = "aux_aiodio1")]
pub mod aux_aiodio1;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO2 {}
#[cfg(feature = "aux_aiodio2")]
impl AUX_AIODIO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio2::RegisterBlock {
        0x400c_e000 as *const _
    }
}
#[cfg(feature = "aux_aiodio2")]
impl Deref for AUX_AIODIO2 {
    type Target = aux_aiodio2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO2::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
#[cfg(feature = "aux_aiodio2")]
pub mod aux_aiodio2;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO3 {}
#[cfg(feature = "aux_aiodio3")]
impl AUX_AIODIO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio3::RegisterBlock {
        0x400c_f000 as *const _
    }
}
#[cfg(feature = "aux_aiodio3")]
impl Deref for AUX_AIODIO3 {
    type Target = aux_aiodio3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO3::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: - be connected to analog AUX modules, such as comparators and ADC. - be used by AUX_SCE. - connect to AUX_SPIM SCLK, MISO and MOSI signals. - connect to the asynchronous AUX event bus. Enabled digital inputs are synchronized at SCE clock rate. Note that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
#[cfg(feature = "aux_aiodio3")]
pub mod aux_aiodio3;
#[doc = "AUX Analog Interface (AUX_ANAIF) encapsulates direct data and control interfaces between AUX digital and AUX analog circuits. It lets AUX_SCE, UDMA0, and system CPU: -Trigger ADC sample and conversion process. - Write ADC samples to FIFO. - Charge analog nodes by the use of the analog ISRC module. See ADI_4_AUX:ISRC and ADI_4_AUX:COMP.COMPA_REF_CURR_EN for further information. - Use the DAC to generate a programmable voltage on COMPB_REF, COMPA_REF, or COMPA_IN analog nodes. To use: - ADC : AUX_SCE must request active operational mode with AON_PMCTL:AUXSCECLK.SRC set to SCLK_HFDIV2. There are no requirements for system CPU. - ISRC : AUX_SCE must request active operational mode. There are no requirements for system CPU. - DAC : AUX_SCE must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to SCE_RATE as long as DAC state machine generates the sample clock. System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE as long as DAC state machine generates the sample clock. See DACSMPLCTL.EN for further information."]
pub struct AUX_ANAIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ANAIF {}
#[cfg(feature = "aux_anaif")]
impl AUX_ANAIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_anaif::RegisterBlock {
        0x400c_9000 as *const _
    }
}
#[cfg(feature = "aux_anaif")]
impl Deref for AUX_ANAIF {
    type Target = aux_anaif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_ANAIF::ptr() }
    }
}
#[doc = "AUX Analog Interface (AUX_ANAIF) encapsulates direct data and control interfaces between AUX digital and AUX analog circuits. It lets AUX_SCE, UDMA0, and system CPU: -Trigger ADC sample and conversion process. - Write ADC samples to FIFO. - Charge analog nodes by the use of the analog ISRC module. See ADI_4_AUX:ISRC and ADI_4_AUX:COMP.COMPA_REF_CURR_EN for further information. - Use the DAC to generate a programmable voltage on COMPB_REF, COMPA_REF, or COMPA_IN analog nodes. To use: - ADC : AUX_SCE must request active operational mode with AON_PMCTL:AUXSCECLK.SRC set to SCLK_HFDIV2. There are no requirements for system CPU. - ISRC : AUX_SCE must request active operational mode. There are no requirements for system CPU. - DAC : AUX_SCE must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to SCE_RATE as long as DAC state machine generates the sample clock. System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE as long as DAC state machine generates the sample clock. See DACSMPLCTL.EN for further information."]
#[cfg(feature = "aux_anaif")]
pub mod aux_anaif;
#[doc = "AUX Event Controller (AUX_EVCTL) assembles events originating from: - AUX submodules, including ADC and comparators. - AUXIO. - EVENT. - AON_PMCTL. - AON_RTC. - AON_BATMON. into two 64-bit event buses. One is synchronized to the AUX clock and one is left unsynchronized. The subscribers to the synchronous event bus are AUX_TIMER01, AUX_SCE and AUX_EVCTL. The subscribers to the asynchronous event bus are AUX_TIMER2, AUX_ANAIF, AUX_TDC and AUX_SYSIF. AUX_EVCTL uses the synchronous event bus to generate events to AON_EVENT and EVENT, as well as to AUX_SCE. AUX_SCE can poll event status registers and combine certain instructions like WEV0, WEV1 with one or two configurable events. The latter saves power when execution must stall until a condition is met."]
pub struct AUX_EVCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_EVCTL {}
#[cfg(feature = "aux_evctl")]
impl AUX_EVCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_evctl::RegisterBlock {
        0x400c_5000 as *const _
    }
}
#[cfg(feature = "aux_evctl")]
impl Deref for AUX_EVCTL {
    type Target = aux_evctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_EVCTL::ptr() }
    }
}
#[doc = "AUX Event Controller (AUX_EVCTL) assembles events originating from: - AUX submodules, including ADC and comparators. - AUXIO. - EVENT. - AON_PMCTL. - AON_RTC. - AON_BATMON. into two 64-bit event buses. One is synchronized to the AUX clock and one is left unsynchronized. The subscribers to the synchronous event bus are AUX_TIMER01, AUX_SCE and AUX_EVCTL. The subscribers to the asynchronous event bus are AUX_TIMER2, AUX_ANAIF, AUX_TDC and AUX_SYSIF. AUX_EVCTL uses the synchronous event bus to generate events to AON_EVENT and EVENT, as well as to AUX_SCE. AUX_SCE can poll event status registers and combine certain instructions like WEV0, WEV1 with one or two configurable events. The latter saves power when execution must stall until a condition is met."]
#[cfg(feature = "aux_evctl")]
pub mod aux_evctl;
#[doc = "The AUX Multiply-Accumulate (AUX_MAC) peripheral enables AUX_SCE with power-efficient and flexible mathematical operations: - 2's complement signed and unsigned sequential multiplication (MUL) with optional accumulation of the result (MAC). - 16 or 32-bit 2's complement signed and unsigned addition of configurable term and accumulator (ADD). - Results of ADD, MUL and MAC operations are always stored in the accumulator (ACC). Software can easily: - Access arbitrary 16-bit slice of the 40-bit accumulator. - Find the number of leading zero or sign bits. - Perform shift operations on the accumulator. AUX_SCE must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to SCE_RATE to access and use AUX_MAC. System CPU must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to BUS_RATE to access and use AUX_MAC. This guarantees constant execution times for ADD, MUL, and MAC operations. The ADD operation requires a single peripheral clock cycle to finish. MUL and MAC operations require four peripheral clock periods to finish. An unfinished ADD, MUL, or MAC operation stalls register access to this peripheral. AUX_SCE becomes clock gated if it encounters a bus stall. Software can use this to reduce power consumption during back to back accesses. Only full word access is supported by the peripheral. An attempt to write a single byte will have no effect."]
pub struct AUX_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_MAC {}
#[cfg(feature = "aux_mac")]
impl AUX_MAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_mac::RegisterBlock {
        0x400c_2000 as *const _
    }
}
#[cfg(feature = "aux_mac")]
impl Deref for AUX_MAC {
    type Target = aux_mac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_MAC::ptr() }
    }
}
#[doc = "The AUX Multiply-Accumulate (AUX_MAC) peripheral enables AUX_SCE with power-efficient and flexible mathematical operations: - 2's complement signed and unsigned sequential multiplication (MUL) with optional accumulation of the result (MAC). - 16 or 32-bit 2's complement signed and unsigned addition of configurable term and accumulator (ADD). - Results of ADD, MUL and MAC operations are always stored in the accumulator (ACC). Software can easily: - Access arbitrary 16-bit slice of the 40-bit accumulator. - Find the number of leading zero or sign bits. - Perform shift operations on the accumulator. AUX_SCE must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to SCE_RATE to access and use AUX_MAC. System CPU must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to BUS_RATE to access and use AUX_MAC. This guarantees constant execution times for ADD, MUL, and MAC operations. The ADD operation requires a single peripheral clock cycle to finish. MUL and MAC operations require four peripheral clock periods to finish. An unfinished ADD, MUL, or MAC operation stalls register access to this peripheral. AUX_SCE becomes clock gated if it encounters a bus stall. Software can use this to reduce power consumption during back to back accesses. Only full word access is supported by the peripheral. An attempt to write a single byte will have no effect."]
#[cfg(feature = "aux_mac")]
pub mod aux_mac;
#[doc = "AUX Sensor Control Engine (AUX_SCE) is a RISC-style microprocessor with separate fetch and execution cycles. It is optimized for low power and simple operations. AUX_SCE code and data segments are stored in AUX_RAM. AON_PMCTL:AUXSCECLK sets the operational frequency."]
pub struct AUX_SCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SCE {}
#[cfg(feature = "aux_sce")]
impl AUX_SCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_sce::RegisterBlock {
        0x400e_1000 as *const _
    }
}
#[cfg(feature = "aux_sce")]
impl Deref for AUX_SCE {
    type Target = aux_sce::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SCE::ptr() }
    }
}
#[doc = "AUX Sensor Control Engine (AUX_SCE) is a RISC-style microprocessor with separate fetch and execution cycles. It is optimized for low power and simple operations. AUX_SCE code and data segments are stored in AUX_RAM. AON_PMCTL:AUXSCECLK sets the operational frequency."]
#[cfg(feature = "aux_sce")]
pub mod aux_sce;
#[doc = "AUX Semaphore (AUX_SMPH) provides hardware means to share modules in AUX safely between CPUs based on resource ownership. AUX_SMPH operates at AUX bus rate."]
pub struct AUX_SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SMPH {}
#[cfg(feature = "aux_smph")]
impl AUX_SMPH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_smph::RegisterBlock {
        0x400c_8000 as *const _
    }
}
#[cfg(feature = "aux_smph")]
impl Deref for AUX_SMPH {
    type Target = aux_smph::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SMPH::ptr() }
    }
}
#[doc = "AUX Semaphore (AUX_SMPH) provides hardware means to share modules in AUX safely between CPUs based on resource ownership. AUX_SMPH operates at AUX bus rate."]
#[cfg(feature = "aux_smph")]
pub mod aux_smph;
#[doc = "The AUX Serial Peripheral Interface Master (AUX_SPIM) enables AUX_SCE with power-efficient SPI communication. It is not possible to write a register while SPI transmission occurs. An attempt to do so will stall the bus until transmission is complete. Read of RX8.DATA or RX16.DATA stalls the bus until LSB has been captured. Read of SCLKIDLE.STAT or DATAIDLE.STAT stalls the bus until condition described is met. Other read operations do not stall the bus. AUX_SCE becomes clock gated if it encounters a bus stall. This is useful as AUX_SCE can write TX8.DATA and then read RX8.DATA immediately to read a SPI slave. In such case there is no need for software to wait or to poll registers. AUX_SYSIF:PEROPRATE.SPIM_OP_RATE selects the peripheral clock frequency which is used to derive the SCLK frequency. AUX_SCE must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to SCE_RATE to access and use AUX_SPIM. System CPU must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to BUS_RATE to access and use AUX_SPIM. Failure to do so can result in incorrect SPI transmission."]
pub struct AUX_SPIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SPIM {}
#[cfg(feature = "aux_spim")]
impl AUX_SPIM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_spim::RegisterBlock {
        0x400c_1000 as *const _
    }
}
#[cfg(feature = "aux_spim")]
impl Deref for AUX_SPIM {
    type Target = aux_spim::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SPIM::ptr() }
    }
}
#[doc = "The AUX Serial Peripheral Interface Master (AUX_SPIM) enables AUX_SCE with power-efficient SPI communication. It is not possible to write a register while SPI transmission occurs. An attempt to do so will stall the bus until transmission is complete. Read of RX8.DATA or RX16.DATA stalls the bus until LSB has been captured. Read of SCLKIDLE.STAT or DATAIDLE.STAT stalls the bus until condition described is met. Other read operations do not stall the bus. AUX_SCE becomes clock gated if it encounters a bus stall. This is useful as AUX_SCE can write TX8.DATA and then read RX8.DATA immediately to read a SPI slave. In such case there is no need for software to wait or to poll registers. AUX_SYSIF:PEROPRATE.SPIM_OP_RATE selects the peripheral clock frequency which is used to derive the SCLK frequency. AUX_SCE must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to SCE_RATE to access and use AUX_SPIM. System CPU must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to BUS_RATE to access and use AUX_SPIM. Failure to do so can result in incorrect SPI transmission."]
#[cfg(feature = "aux_spim")]
pub mod aux_spim;
#[doc = "AUX System Interface (AUX_SYSIF) is responsible for: - system resource requests, such as power supply, clock and, wakeup requests. - configuration of AUX peripheral operational rates for AUX_SPIM, AUX_MAC, AUX_ANAIF DAC state machine and AUX_TIMER01. - configuration of event synchronization rate for AUX_EVCTL:EVSTAT2 and AUX_EVCTL:EVSTAT3. - configuration of AUX_SCE wakeup vectors that trigger AUX_SCE execution from sleep. Peripheral operational rate for AUX modules mentioned above can either be: - SCE rate, which is configured in AON_PMCTL:AUXSCECLK. - AUX bus rate, which equals SCE rate or SCLK_HF divided by two when MCU domain is active or AUX operational mode is active. AUX_SYSIF also interfaces AON_RTC and AON_BATMON to enable read access to data and sub-second increment control of AON_RTC."]
pub struct AUX_SYSIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SYSIF {}
#[cfg(feature = "aux_sysif")]
impl AUX_SYSIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_sysif::RegisterBlock {
        0x400c_6000 as *const _
    }
}
#[cfg(feature = "aux_sysif")]
impl Deref for AUX_SYSIF {
    type Target = aux_sysif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SYSIF::ptr() }
    }
}
#[doc = "AUX System Interface (AUX_SYSIF) is responsible for: - system resource requests, such as power supply, clock and, wakeup requests. - configuration of AUX peripheral operational rates for AUX_SPIM, AUX_MAC, AUX_ANAIF DAC state machine and AUX_TIMER01. - configuration of event synchronization rate for AUX_EVCTL:EVSTAT2 and AUX_EVCTL:EVSTAT3. - configuration of AUX_SCE wakeup vectors that trigger AUX_SCE execution from sleep. Peripheral operational rate for AUX modules mentioned above can either be: - SCE rate, which is configured in AON_PMCTL:AUXSCECLK. - AUX bus rate, which equals SCE rate or SCLK_HF divided by two when MCU domain is active or AUX operational mode is active. AUX_SYSIF also interfaces AON_RTC and AON_BATMON to enable read access to data and sub-second increment control of AON_RTC."]
#[cfg(feature = "aux_sysif")]
pub mod aux_sysif;
#[doc = "AUX Time To Digital Converter (AUX_TDC) is used to measure the time between two events with high resolution. AUX_TDC consists of a state machine that operates at AUX bus rate and an asynchronous fast-counter which is clocked by the TDC clock. DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL configures TDC clock source. The fast-counter counts on both edges of the TDC clock to double the resolution. See the Technical Reference Manual for event timing requirements."]
pub struct AUX_TDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TDC {}
#[cfg(feature = "aux_tdc")]
impl AUX_TDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_tdc::RegisterBlock {
        0x400c_4000 as *const _
    }
}
#[cfg(feature = "aux_tdc")]
impl Deref for AUX_TDC {
    type Target = aux_tdc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TDC::ptr() }
    }
}
#[doc = "AUX Time To Digital Converter (AUX_TDC) is used to measure the time between two events with high resolution. AUX_TDC consists of a state machine that operates at AUX bus rate and an asynchronous fast-counter which is clocked by the TDC clock. DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL configures TDC clock source. The fast-counter counts on both edges of the TDC clock to double the resolution. See the Technical Reference Manual for event timing requirements."]
#[cfg(feature = "aux_tdc")]
pub mod aux_tdc;
#[doc = "AUX Timer 0 and AUX Timer 1 (AUX_TIMER01) are two 16-bit timers capable of generating one event each: - AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV. - AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV. The events are described in T0TARGET and T1TARGET. Subscribers to the AUX event bus can use these events to sequence and trigger actions. AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the peripheral clock frequency used by the prescaler, timer, and event logic to SCE or AUX bus rate. To use AUX_TIMER01: - AUX_SCE must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to SCE_RATE. - System CPU must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to BUS_RATE. - The timers must only subscribe to events updated at the peripheral clock frequency or lower. Unexpected execution behavior can result if software does not obey these rules."]
pub struct AUX_TIMER01 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TIMER01 {}
#[cfg(feature = "aux_timer01")]
impl AUX_TIMER01 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_timer01::RegisterBlock {
        0x400c_7000 as *const _
    }
}
#[cfg(feature = "aux_timer01")]
impl Deref for AUX_TIMER01 {
    type Target = aux_timer01::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TIMER01::ptr() }
    }
}
#[doc = "AUX Timer 0 and AUX Timer 1 (AUX_TIMER01) are two 16-bit timers capable of generating one event each: - AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV. - AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV. The events are described in T0TARGET and T1TARGET. Subscribers to the AUX event bus can use these events to sequence and trigger actions. AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the peripheral clock frequency used by the prescaler, timer, and event logic to SCE or AUX bus rate. To use AUX_TIMER01: - AUX_SCE must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to SCE_RATE. - System CPU must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to BUS_RATE. - The timers must only subscribe to events updated at the peripheral clock frequency or lower. Unexpected execution behavior can result if software does not obey these rules."]
#[cfg(feature = "aux_timer01")]
pub mod aux_timer01;
#[doc = "AUX Timer2 (AUX_TIMER2) offers flexible: - generation of waveforms and events. - capture of signal period and duty cycle. - generation of single clock pulse. It consists of a: - 16-bit counter. - 4 capture compare channels. - 4 event outputs, which are mapped to AUX event bus, see EVCTL. Each channel subscribes to the asynchronous AUX event bus. They can control one or more event outputs in both capture and compare modes. AUX_SYSIF:TIMER2CLKCTL.SRC selects clock source for the timer."]
pub struct AUX_TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TIMER2 {}
#[cfg(feature = "aux_timer2")]
impl AUX_TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_timer2::RegisterBlock {
        0x400c_3000 as *const _
    }
}
#[cfg(feature = "aux_timer2")]
impl Deref for AUX_TIMER2 {
    type Target = aux_timer2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TIMER2::ptr() }
    }
}
#[doc = "AUX Timer2 (AUX_TIMER2) offers flexible: - generation of waveforms and events. - capture of signal period and duty cycle. - generation of single clock pulse. It consists of a: - 16-bit counter. - 4 capture compare channels. - 4 event outputs, which are mapped to AUX event bus, see EVCTL. Each channel subscribes to the asynchronous AUX event bus. They can control one or more event outputs in both capture and compare modes. AUX_SYSIF:TIMER2CLKCTL.SRC selects clock source for the timer."]
#[cfg(feature = "aux_timer2")]
pub mod aux_timer2;
#[doc = "Customer configuration area (CCFG)"]
pub struct CCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCFG {}
#[cfg(feature = "ccfg")]
impl CCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccfg::RegisterBlock {
        0x5000_3000 as *const _
    }
}
#[cfg(feature = "ccfg")]
impl Deref for CCFG {
    type Target = ccfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCFG::ptr() }
    }
}
#[doc = "Customer configuration area (CCFG)"]
#[cfg(feature = "ccfg")]
pub mod ccfg;
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
pub struct CPU_DWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_DWT {}
#[cfg(feature = "cpu_dwt")]
impl CPU_DWT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_dwt::RegisterBlock {
        0xe000_1000 as *const _
    }
}
#[cfg(feature = "cpu_dwt")]
impl Deref for CPU_DWT {
    type Target = cpu_dwt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_DWT::ptr() }
    }
}
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
#[cfg(feature = "cpu_dwt")]
pub mod cpu_dwt;
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
pub struct CPU_FPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_FPB {}
#[cfg(feature = "cpu_fpb")]
impl CPU_FPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_fpb::RegisterBlock {
        0xe000_2000 as *const _
    }
}
#[cfg(feature = "cpu_fpb")]
impl Deref for CPU_FPB {
    type Target = cpu_fpb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_FPB::ptr() }
    }
}
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
#[cfg(feature = "cpu_fpb")]
pub mod cpu_fpb;
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
pub struct CPU_ITM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_ITM {}
#[cfg(feature = "cpu_itm")]
impl CPU_ITM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_itm::RegisterBlock {
        0xe000_0000 as *const _
    }
}
#[cfg(feature = "cpu_itm")]
impl Deref for CPU_ITM {
    type Target = cpu_itm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_ITM::ptr() }
    }
}
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
#[cfg(feature = "cpu_itm")]
pub mod cpu_itm;
#[doc = "Cortex-M's System Control Space (SCS)"]
pub struct CPU_SCS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_SCS {}
#[cfg(feature = "cpu_scs")]
impl CPU_SCS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_scs::RegisterBlock {
        0xe000_e000 as *const _
    }
}
#[cfg(feature = "cpu_scs")]
impl Deref for CPU_SCS {
    type Target = cpu_scs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_SCS::ptr() }
    }
}
#[doc = "Cortex-M's System Control Space (SCS)"]
#[cfg(feature = "cpu_scs")]
pub mod cpu_scs;
#[doc = "Cortex-M's TI proprietary registers"]
pub struct CPU_TIPROP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TIPROP {}
#[cfg(feature = "cpu_tiprop")]
impl CPU_TIPROP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_tiprop::RegisterBlock {
        0xe00f_e000 as *const _
    }
}
#[cfg(feature = "cpu_tiprop")]
impl Deref for CPU_TIPROP {
    type Target = cpu_tiprop::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_TIPROP::ptr() }
    }
}
#[doc = "Cortex-M's TI proprietary registers"]
#[cfg(feature = "cpu_tiprop")]
pub mod cpu_tiprop;
#[doc = "Cortex-M's Trace Port Interface Unit (TPIU)"]
pub struct CPU_TPIU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TPIU {}
#[cfg(feature = "cpu_tpiu")]
impl CPU_TPIU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_tpiu::RegisterBlock {
        0xe004_0000 as *const _
    }
}
#[cfg(feature = "cpu_tpiu")]
impl Deref for CPU_TPIU {
    type Target = cpu_tpiu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_TPIU::ptr() }
    }
}
#[doc = "Cortex-M's Trace Port Interface Unit (TPIU)"]
#[cfg(feature = "cpu_tpiu")]
pub mod cpu_tpiu;
#[doc = "DMA Crypto Core is a low power low gate count crypto core with DMA capability and local key storage."]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
#[cfg(feature = "crypto")]
impl CRYPTO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto::RegisterBlock {
        0x4002_4000 as *const _
    }
}
#[cfg(feature = "crypto")]
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYPTO::ptr() }
    }
}
#[doc = "DMA Crypto Core is a low power low gate count crypto core with DMA capability and local key storage."]
#[cfg(feature = "crypto")]
pub mod crypto;
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
pub struct AUX_DDI0_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_DDI0_OSC {}
#[cfg(feature = "aux_ddi0_osc")]
impl AUX_DDI0_OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_ddi0_osc::RegisterBlock {
        0x400c_a000 as *const _
    }
}
#[cfg(feature = "aux_ddi0_osc")]
impl Deref for AUX_DDI0_OSC {
    type Target = aux_ddi0_osc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_DDI0_OSC::ptr() }
    }
}
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
#[cfg(feature = "aux_ddi0_osc")]
pub mod aux_ddi0_osc;
#[doc = "Event Fabric Component Definition"]
pub struct EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVENT {}
#[cfg(feature = "event")]
impl EVENT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const event::RegisterBlock {
        0x4008_3000 as *const _
    }
}
#[cfg(feature = "event")]
impl Deref for EVENT {
    type Target = event::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVENT::ptr() }
    }
}
#[doc = "Event Fabric Component Definition"]
#[cfg(feature = "event")]
pub mod event;
#[doc = "Factory configuration area (FCFG1)"]
pub struct FCFG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCFG1 {}
#[cfg(feature = "fcfg1")]
impl FCFG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fcfg1::RegisterBlock {
        0x5000_1000 as *const _
    }
}
#[cfg(feature = "fcfg1")]
impl Deref for FCFG1 {
    type Target = fcfg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCFG1::ptr() }
    }
}
#[doc = "Factory configuration area (FCFG1)"]
#[cfg(feature = "fcfg1")]
pub mod fcfg1;
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
#[cfg(feature = "flash")]
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4003_0000 as *const _
    }
}
#[cfg(feature = "flash")]
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
#[cfg(feature = "flash")]
pub mod flash;
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
#[cfg(feature = "gpio")]
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4002_2000 as *const _
    }
}
#[cfg(feature = "gpio")]
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
#[cfg(feature = "gpio")]
pub mod gpio;
#[doc = "General Purpose Timer."]
pub struct GPT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT0 {}
#[cfg(feature = "gpt0")]
impl GPT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
#[cfg(feature = "gpt0")]
impl Deref for GPT0 {
    type Target = gpt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT0::ptr() }
    }
}
#[doc = "General Purpose Timer."]
#[cfg(feature = "gpt0")]
pub mod gpt0;
#[doc = "General Purpose Timer."]
pub struct GPT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT1 {}
#[cfg(feature = "gpt1")]
impl GPT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
#[cfg(feature = "gpt1")]
impl Deref for GPT1 {
    type Target = gpt1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT1::ptr() }
    }
}
#[doc = "General Purpose Timer."]
#[cfg(feature = "gpt1")]
pub mod gpt1;
#[doc = "General Purpose Timer."]
pub struct GPT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT2 {}
#[cfg(feature = "gpt2")]
impl GPT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt2::RegisterBlock {
        0x4001_2000 as *const _
    }
}
#[cfg(feature = "gpt2")]
impl Deref for GPT2 {
    type Target = gpt2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT2::ptr() }
    }
}
#[doc = "General Purpose Timer."]
#[cfg(feature = "gpt2")]
pub mod gpt2;
#[doc = "General Purpose Timer."]
pub struct GPT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT3 {}
#[cfg(feature = "gpt3")]
impl GPT3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt3::RegisterBlock {
        0x4001_3000 as *const _
    }
}
#[cfg(feature = "gpt3")]
impl Deref for GPT3 {
    type Target = gpt3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT3::ptr() }
    }
}
#[doc = "General Purpose Timer."]
#[cfg(feature = "gpt3")]
pub mod gpt3;
#[doc = "I2CMaster/Slave Serial Controler"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
#[cfg(feature = "i2c0")]
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
#[cfg(feature = "i2c0")]
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2CMaster/Slave Serial Controler"]
#[cfg(feature = "i2c0")]
pub mod i2c0;
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
#[cfg(feature = "i2s0")]
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4002_1000 as *const _
    }
}
#[cfg(feature = "i2s0")]
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
#[cfg(feature = "i2s0")]
pub mod i2s0;
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub struct IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOC {}
#[cfg(feature = "ioc")]
impl IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ioc::RegisterBlock {
        0x4008_1000 as *const _
    }
}
#[cfg(feature = "ioc")]
impl Deref for IOC {
    type Target = ioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOC::ptr() }
    }
}
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
#[cfg(feature = "ioc")]
pub mod ioc;
#[doc = "Integrated module which combines the Public Key Acceleration module, optional True Random Gnerator, optional interrupt controller and a standard bus interface"]
pub struct PKA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA {}
#[cfg(feature = "pka")]
impl PKA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka::RegisterBlock {
        0x4002_5000 as *const _
    }
}
#[cfg(feature = "pka")]
impl Deref for PKA {
    type Target = pka::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA::ptr() }
    }
}
#[doc = "Integrated module which combines the Public Key Acceleration module, optional True Random Gnerator, optional interrupt controller and a standard bus interface"]
#[cfg(feature = "pka")]
pub mod pka;
#[doc = "Integrated module which includes the PKA K"]
pub struct PKA_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA_INT {}
#[cfg(feature = "pka_int")]
impl PKA_INT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka_int::RegisterBlock {
        0x4002_7000 as *const _
    }
}
#[cfg(feature = "pka_int")]
impl Deref for PKA_INT {
    type Target = pka_int::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA_INT::ptr() }
    }
}
#[doc = "Integrated module which includes the PKA K"]
#[cfg(feature = "pka_int")]
pub mod pka_int;
#[doc = "Power, Reset and Clock Management"]
pub struct PRCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCM {}
#[cfg(feature = "prcm")]
impl PRCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prcm::RegisterBlock {
        0x4008_2000 as *const _
    }
}
#[cfg(feature = "prcm")]
impl Deref for PRCM {
    type Target = prcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PRCM::ptr() }
    }
}
#[doc = "Power, Reset and Clock Management"]
#[cfg(feature = "prcm")]
pub mod prcm;
#[doc = "RF core doorbell The doorbell module is the main user interface to the radio sub-system. It contains the registers used for both submitting commands to the radio, and for configuring radio interrupts from the RF core."]
pub struct RFC_DBELL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_DBELL {}
#[cfg(feature = "rfc_dbell")]
impl RFC_DBELL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_dbell::RegisterBlock {
        0x4004_1000 as *const _
    }
}
#[cfg(feature = "rfc_dbell")]
impl Deref for RFC_DBELL {
    type Target = rfc_dbell::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_DBELL::ptr() }
    }
}
#[doc = "RF core doorbell The doorbell module is the main user interface to the radio sub-system. It contains the registers used for both submitting commands to the radio, and for configuring radio interrupts from the RF core."]
#[cfg(feature = "rfc_dbell")]
pub mod rfc_dbell;
#[doc = "RF core power management This module contains clock control for all RF core sub-modules."]
pub struct RFC_PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_PWR {}
#[cfg(feature = "rfc_pwr")]
impl RFC_PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_pwr::RegisterBlock {
        0x4004_0000 as *const _
    }
}
#[cfg(feature = "rfc_pwr")]
impl Deref for RFC_PWR {
    type Target = rfc_pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_PWR::ptr() }
    }
}
#[doc = "RF core power management This module contains clock control for all RF core sub-modules."]
#[cfg(feature = "rfc_pwr")]
pub mod rfc_pwr;
#[doc = "RF core radio timer"]
pub struct RFC_RAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_RAT {}
#[cfg(feature = "rfc_rat")]
impl RFC_RAT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_rat::RegisterBlock {
        0x4004_3000 as *const _
    }
}
#[cfg(feature = "rfc_rat")]
impl Deref for RFC_RAT {
    type Target = rfc_rat::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_RAT::ptr() }
    }
}
#[doc = "RF core radio timer"]
#[cfg(feature = "rfc_rat")]
pub mod rfc_rat;
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub struct SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMPH {}
#[cfg(feature = "smph")]
impl SMPH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smph::RegisterBlock {
        0x4008_4000 as *const _
    }
}
#[cfg(feature = "smph")]
impl Deref for SMPH {
    type Target = smph::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMPH::ptr() }
    }
}
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
#[cfg(feature = "smph")]
pub mod smph;
#[doc = "General Purpose RAM"]
pub struct SRAM_MMR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRAM_MMR {}
#[cfg(feature = "sram_mmr")]
impl SRAM_MMR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram_mmr::RegisterBlock {
        0x4003_5000 as *const _
    }
}
#[cfg(feature = "sram_mmr")]
impl Deref for SRAM_MMR {
    type Target = sram_mmr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRAM_MMR::ptr() }
    }
}
#[doc = "General Purpose RAM"]
#[cfg(feature = "sram_mmr")]
pub mod sram_mmr;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
#[cfg(feature = "ssi0")]
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
#[cfg(feature = "ssi0")]
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
#[cfg(feature = "ssi0")]
pub mod ssi0;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
#[cfg(feature = "ssi1")]
impl SSI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi1::RegisterBlock {
        0x4000_8000 as *const _
    }
}
#[cfg(feature = "ssi1")]
impl Deref for SSI1 {
    type Target = ssi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
#[cfg(feature = "ssi1")]
pub mod ssi1;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
#[cfg(feature = "trng")]
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4002_8000 as *const _
    }
}
#[cfg(feature = "trng")]
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
#[cfg(feature = "trng")]
pub mod trng;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
#[cfg(feature = "uart0")]
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_1000 as *const _
    }
}
#[cfg(feature = "uart0")]
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
#[cfg(feature = "uart0")]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
#[cfg(feature = "uart1")]
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4000_b000 as *const _
    }
}
#[cfg(feature = "uart1")]
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
#[cfg(feature = "uart1")]
pub mod uart1;
#[doc = "ARM Micro Direct Memory Access Controller"]
pub struct UDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA0 {}
#[cfg(feature = "udma0")]
impl UDMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
#[cfg(feature = "udma0")]
impl Deref for UDMA0 {
    type Target = udma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA0::ptr() }
    }
}
#[doc = "ARM Micro Direct Memory Access Controller"]
#[cfg(feature = "udma0")]
pub mod udma0;
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
pub struct VIMS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VIMS {}
#[cfg(feature = "vims")]
impl VIMS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vims::RegisterBlock {
        0x4003_4000 as *const _
    }
}
#[cfg(feature = "vims")]
impl Deref for VIMS {
    type Target = vims::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VIMS::ptr() }
    }
}
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
#[cfg(feature = "vims")]
pub mod vims;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
#[cfg(feature = "wdt")]
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4008_0000 as *const _
    }
}
#[cfg(feature = "wdt")]
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
#[cfg(feature = "wdt")]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AUX_ADI4"]
    pub AUX_ADI4: AUX_ADI4,
    #[doc = "AON_BATMON"]
    pub AON_BATMON: AON_BATMON,
    #[doc = "AON_EVENT"]
    pub AON_EVENT: AON_EVENT,
    #[doc = "AON_IOC"]
    pub AON_IOC: AON_IOC,
    #[doc = "AON_PMCTL"]
    pub AON_PMCTL: AON_PMCTL,
    #[doc = "AON_RTC"]
    pub AON_RTC: AON_RTC,
    #[doc = "AUX_AIODIO0"]
    pub AUX_AIODIO0: AUX_AIODIO0,
    #[doc = "AUX_AIODIO1"]
    pub AUX_AIODIO1: AUX_AIODIO1,
    #[doc = "AUX_AIODIO2"]
    pub AUX_AIODIO2: AUX_AIODIO2,
    #[doc = "AUX_AIODIO3"]
    pub AUX_AIODIO3: AUX_AIODIO3,
    #[doc = "AUX_ANAIF"]
    pub AUX_ANAIF: AUX_ANAIF,
    #[doc = "AUX_EVCTL"]
    pub AUX_EVCTL: AUX_EVCTL,
    #[doc = "AUX_MAC"]
    pub AUX_MAC: AUX_MAC,
    #[doc = "AUX_SCE"]
    pub AUX_SCE: AUX_SCE,
    #[doc = "AUX_SMPH"]
    pub AUX_SMPH: AUX_SMPH,
    #[doc = "AUX_SPIM"]
    pub AUX_SPIM: AUX_SPIM,
    #[doc = "AUX_SYSIF"]
    pub AUX_SYSIF: AUX_SYSIF,
    #[doc = "AUX_TDC"]
    pub AUX_TDC: AUX_TDC,
    #[doc = "AUX_TIMER01"]
    pub AUX_TIMER01: AUX_TIMER01,
    #[doc = "AUX_TIMER2"]
    pub AUX_TIMER2: AUX_TIMER2,
    #[doc = "CCFG"]
    pub CCFG: CCFG,
    #[doc = "CPU_DWT"]
    pub CPU_DWT: CPU_DWT,
    #[doc = "CPU_FPB"]
    pub CPU_FPB: CPU_FPB,
    #[doc = "CPU_ITM"]
    pub CPU_ITM: CPU_ITM,
    #[doc = "CPU_SCS"]
    pub CPU_SCS: CPU_SCS,
    #[doc = "CPU_TIPROP"]
    pub CPU_TIPROP: CPU_TIPROP,
    #[doc = "CPU_TPIU"]
    pub CPU_TPIU: CPU_TPIU,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "AUX_DDI0_OSC"]
    pub AUX_DDI0_OSC: AUX_DDI0_OSC,
    #[doc = "EVENT"]
    pub EVENT: EVENT,
    #[doc = "FCFG1"]
    pub FCFG1: FCFG1,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPT0"]
    pub GPT0: GPT0,
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[doc = "GPT3"]
    pub GPT3: GPT3,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "IOC"]
    pub IOC: IOC,
    #[doc = "PKA"]
    pub PKA: PKA,
    #[doc = "PKA_INT"]
    pub PKA_INT: PKA_INT,
    #[doc = "PRCM"]
    pub PRCM: PRCM,
    #[doc = "RFC_DBELL"]
    pub RFC_DBELL: RFC_DBELL,
    #[doc = "RFC_PWR"]
    pub RFC_PWR: RFC_PWR,
    #[doc = "RFC_RAT"]
    pub RFC_RAT: RFC_RAT,
    #[doc = "SMPH"]
    pub SMPH: SMPH,
    #[doc = "SRAM_MMR"]
    pub SRAM_MMR: SRAM_MMR,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UDMA0"]
    pub UDMA0: UDMA0,
    #[doc = "VIMS"]
    pub VIMS: VIMS,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AUX_ADI4: AUX_ADI4 {
                _marker: PhantomData,
            },
            AON_BATMON: AON_BATMON {
                _marker: PhantomData,
            },
            AON_EVENT: AON_EVENT {
                _marker: PhantomData,
            },
            AON_IOC: AON_IOC {
                _marker: PhantomData,
            },
            AON_PMCTL: AON_PMCTL {
                _marker: PhantomData,
            },
            AON_RTC: AON_RTC {
                _marker: PhantomData,
            },
            AUX_AIODIO0: AUX_AIODIO0 {
                _marker: PhantomData,
            },
            AUX_AIODIO1: AUX_AIODIO1 {
                _marker: PhantomData,
            },
            AUX_AIODIO2: AUX_AIODIO2 {
                _marker: PhantomData,
            },
            AUX_AIODIO3: AUX_AIODIO3 {
                _marker: PhantomData,
            },
            AUX_ANAIF: AUX_ANAIF {
                _marker: PhantomData,
            },
            AUX_EVCTL: AUX_EVCTL {
                _marker: PhantomData,
            },
            AUX_MAC: AUX_MAC {
                _marker: PhantomData,
            },
            AUX_SCE: AUX_SCE {
                _marker: PhantomData,
            },
            AUX_SMPH: AUX_SMPH {
                _marker: PhantomData,
            },
            AUX_SPIM: AUX_SPIM {
                _marker: PhantomData,
            },
            AUX_SYSIF: AUX_SYSIF {
                _marker: PhantomData,
            },
            AUX_TDC: AUX_TDC {
                _marker: PhantomData,
            },
            AUX_TIMER01: AUX_TIMER01 {
                _marker: PhantomData,
            },
            AUX_TIMER2: AUX_TIMER2 {
                _marker: PhantomData,
            },
            CCFG: CCFG {
                _marker: PhantomData,
            },
            CPU_DWT: CPU_DWT {
                _marker: PhantomData,
            },
            CPU_FPB: CPU_FPB {
                _marker: PhantomData,
            },
            CPU_ITM: CPU_ITM {
                _marker: PhantomData,
            },
            CPU_SCS: CPU_SCS {
                _marker: PhantomData,
            },
            CPU_TIPROP: CPU_TIPROP {
                _marker: PhantomData,
            },
            CPU_TPIU: CPU_TPIU {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            AUX_DDI0_OSC: AUX_DDI0_OSC {
                _marker: PhantomData,
            },
            EVENT: EVENT {
                _marker: PhantomData,
            },
            FCFG1: FCFG1 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPT0: GPT0 {
                _marker: PhantomData,
            },
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            GPT3: GPT3 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            IOC: IOC {
                _marker: PhantomData,
            },
            PKA: PKA {
                _marker: PhantomData,
            },
            PKA_INT: PKA_INT {
                _marker: PhantomData,
            },
            PRCM: PRCM {
                _marker: PhantomData,
            },
            RFC_DBELL: RFC_DBELL {
                _marker: PhantomData,
            },
            RFC_PWR: RFC_PWR {
                _marker: PhantomData,
            },
            RFC_RAT: RFC_RAT {
                _marker: PhantomData,
            },
            SMPH: SMPH {
                _marker: PhantomData,
            },
            SRAM_MMR: SRAM_MMR {
                _marker: PhantomData,
            },
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            SSI1: SSI1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UDMA0: UDMA0 {
                _marker: PhantomData,
            },
            VIMS: VIMS {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
