#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved000: RESERVED000,
    #[doc = "0x04 - Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
    pub ictr: ICTR,
    #[doc = "0x08 - Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
    pub actlr: ACTLR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
    pub stcsr: STCSR,
    #[doc = "0x14 - SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
    pub strvr: STRVR,
    #[doc = "0x18 - SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
    pub stcvr: STCVR,
    #[doc = "0x1c - SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
    pub stcr: STCR,
    _reserved7: [u8; 224usize],
    #[doc = "0x100 - Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
    pub nvic_iser0: NVIC_ISER0,
    #[doc = "0x104 - Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
    pub nvic_iser1: NVIC_ISER1,
    _reserved9: [u8; 120usize],
    #[doc = "0x180 - Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
    pub nvic_icer0: NVIC_ICER0,
    #[doc = "0x184 - Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
    pub nvic_icer1: NVIC_ICER1,
    _reserved11: [u8; 120usize],
    #[doc = "0x200 - Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
    pub nvic_ispr0: NVIC_ISPR0,
    #[doc = "0x204 - Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
    pub nvic_ispr1: NVIC_ISPR1,
    _reserved13: [u8; 120usize],
    #[doc = "0x280 - Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
    pub nvic_icpr0: NVIC_ICPR0,
    #[doc = "0x284 - Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
    pub nvic_icpr1: NVIC_ICPR1,
    _reserved15: [u8; 120usize],
    #[doc = "0x300 - Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
    pub nvic_iabr0: NVIC_IABR0,
    #[doc = "0x304 - Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
    pub nvic_iabr1: NVIC_IABR1,
    _reserved17: [u8; 248usize],
    #[doc = "0x400 - Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0x404 - Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0x408 - Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0x40c - Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0x410 - Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0x414 - Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0x418 - Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0x41c - Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr7: NVIC_IPR7,
    #[doc = "0x420 - Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr8: NVIC_IPR8,
    #[doc = "0x424 - Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr9: NVIC_IPR9,
    _reserved27: [u8; 2264usize],
    #[doc = "0xd00 - CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
    pub cfsr: CFSR,
    #[doc = "0xd2c - Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
    pub dfsr: DFSR,
    #[doc = "0xd34 - Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
    pub bfar: BFAR,
    #[doc = "0xd3c - Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
    pub afsr: AFSR,
    #[doc = "0xd40 - Processor Feature 0"]
    pub id_pfr0: ID_PFR0,
    #[doc = "0xd44 - Processor Feature 1"]
    pub id_pfr1: ID_PFR1,
    #[doc = "0xd48 - Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
    pub id_dfr0: ID_DFR0,
    #[doc = "0xd4c - Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
    pub id_afr0: ID_AFR0,
    #[doc = "0xd50 - Memory Model Feature 0 General information on the memory model and memory management support."]
    pub id_mmfr0: ID_MMFR0,
    #[doc = "0xd54 - Memory Model Feature 1 General information on the memory model and memory management support."]
    pub id_mmfr1: ID_MMFR1,
    #[doc = "0xd58 - Memory Model Feature 2 General information on the memory model and memory management support."]
    pub id_mmfr2: ID_MMFR2,
    #[doc = "0xd5c - Memory Model Feature 3 General information on the memory model and memory management support."]
    pub id_mmfr3: ID_MMFR3,
    #[doc = "0xd60 - ISA Feature 0 Information on the instruction set attributes register"]
    pub id_isar0: ID_ISAR0,
    #[doc = "0xd64 - ISA Feature 1 Information on the instruction set attributes register"]
    pub id_isar1: ID_ISAR1,
    #[doc = "0xd68 - ISA Feature 2 Information on the instruction set attributes register"]
    pub id_isar2: ID_ISAR2,
    #[doc = "0xd6c - ISA Feature 3 Information on the instruction set attributes register"]
    pub id_isar3: ID_ISAR3,
    #[doc = "0xd70 - ISA Feature 4 Information on the instruction set attributes register"]
    pub id_isar4: ID_ISAR4,
    _reserved56: [u8; 20usize],
    #[doc = "0xd88 - Coprocessor Access Control This register specifies the access privileges for coprocessors."]
    pub cpacr: CPACR,
    _reserved57: [u8; 4usize],
    #[doc = "0xd90 - MPU Type This register indicates many regions the MPU supports."]
    pub mpu_type: MPU_TYPE,
    #[doc = "0xd94 - MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities."]
    pub mpu_ctrl: MPU_CTRL,
    #[doc = "0xd98 - MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register."]
    pub mpu_rnr: MPU_RNR,
    #[doc = "0xd9c - MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word."]
    pub mpu_rbar: MPU_RBAR,
    #[doc = "0xda0 - MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0."]
    pub mpu_rasr: MPU_RASR,
    #[doc = "0xda4 - MPU Alias 1 Region Base Address Alias for MPU_RBAR"]
    pub mpu_rbar_a1: MPU_RBAR_A1,
    #[doc = "0xda8 - MPU Alias 1 Region Attribute and Size Alias for MPU_RASR"]
    pub mpu_rasr_a1: MPU_RASR_A1,
    #[doc = "0xdac - MPU Alias 2 Region Base Address Alias for MPU_RBAR"]
    pub mpu_rbar_a2: MPU_RBAR_A2,
    #[doc = "0xdb0 - MPU Alias 2 Region Attribute and Size Alias for MPU_RASR"]
    pub mpu_rasr_a2: MPU_RASR_A2,
    #[doc = "0xdb4 - MPU Alias 3 Region Base Address Alias for MPU_RBAR"]
    pub mpu_rbar_a3: MPU_RBAR_A3,
    #[doc = "0xdb8 - MPU Alias 3 Region Attribute and Size Alias for MPU_RASR"]
    pub mpu_rasr_a3: MPU_RASR_A3,
    _reserved68: [u8; 52usize],
    #[doc = "0xdf0 - Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
    pub dhcsr: DHCSR,
    #[doc = "0xdf4 - Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
    pub dcrsr: DCRSR,
    #[doc = "0xdf8 - Debug Core Register Data"]
    pub dcrdr: DCRDR,
    #[doc = "0xdfc - Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    pub demcr: DEMCR,
    _reserved72: [u8; 256usize],
    #[doc = "0xf00 - Software Trigger Interrupt"]
    pub stir: STIR,
    _reserved73: [u8; 48usize],
    #[doc = "0xf34 - Floating Point Context Control This register holds control data for the floating-point unit. Accessible only by privileged software."]
    pub fpccr: FPCCR,
    #[doc = "0xf38 - Floating-Point Context Address This register holds the location of the unpopulated floating-point register space allocated on an exception stack frame."]
    pub fpcar: FPCAR,
    #[doc = "0xf3c - Floating Point Default Status Control This register holds the default values for the floating-point status control data that the processor assigns to the FPSCR when it creates a new floating-point context. Accessible only by privileged software."]
    pub fpdscr: FPDSCR,
    #[doc = "0xf40 - Media and FP Feature 0 Describes the features provided by the Floating-point extension."]
    pub mvfr0: MVFR0,
    #[doc = "0xf44 - Media and FP Feature 1 Describes the features provided by the Floating-point extension."]
    pub mvfr1: MVFR1,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved000](reserved000) module"]
pub type RESERVED000 = crate::Reg<u32, _RESERVED000>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED000;
#[doc = "`read()` method returns [reserved000::R](reserved000::R) reader structure"]
impl crate::Readable for RESERVED000 {}
#[doc = "`write(|w| ..)` method takes [reserved000::W](reserved000::W) writer structure"]
impl crate::Writable for RESERVED000 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictr](ictr) module"]
pub type ICTR = crate::Reg<u32, _ICTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICTR;
#[doc = "`read()` method returns [ictr::R](ictr::R) reader structure"]
impl crate::Readable for ICTR {}
#[doc = "`write(|w| ..)` method takes [ictr::W](ictr::W) writer structure"]
impl crate::Writable for ICTR {}
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
pub mod ictr;
#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](actlr) module"]
pub type ACTLR = crate::Reg<u32, _ACTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTLR;
#[doc = "`read()` method returns [actlr::R](actlr::R) reader structure"]
impl crate::Readable for ACTLR {}
#[doc = "`write(|w| ..)` method takes [actlr::W](actlr::W) writer structure"]
impl crate::Writable for ACTLR {}
#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
pub mod actlr;
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcsr](stcsr) module"]
pub type STCSR = crate::Reg<u32, _STCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCSR;
#[doc = "`read()` method returns [stcsr::R](stcsr::R) reader structure"]
impl crate::Readable for STCSR {}
#[doc = "`write(|w| ..)` method takes [stcsr::W](stcsr::W) writer structure"]
impl crate::Writable for STCSR {}
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
pub mod stcsr;
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strvr](strvr) module"]
pub type STRVR = crate::Reg<u32, _STRVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STRVR;
#[doc = "`read()` method returns [strvr::R](strvr::R) reader structure"]
impl crate::Readable for STRVR {}
#[doc = "`write(|w| ..)` method takes [strvr::W](strvr::W) writer structure"]
impl crate::Writable for STRVR {}
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
pub mod strvr;
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcvr](stcvr) module"]
pub type STCVR = crate::Reg<u32, _STCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCVR;
#[doc = "`read()` method returns [stcvr::R](stcvr::R) reader structure"]
impl crate::Readable for STCVR {}
#[doc = "`write(|w| ..)` method takes [stcvr::W](stcvr::W) writer structure"]
impl crate::Writable for STCVR {}
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
pub mod stcvr;
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcr](stcr) module"]
pub type STCR = crate::Reg<u32, _STCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCR;
#[doc = "`read()` method returns [stcr::R](stcr::R) reader structure"]
impl crate::Readable for STCR {}
#[doc = "`write(|w| ..)` method takes [stcr::W](stcr::W) writer structure"]
impl crate::Writable for STCR {}
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
pub mod stcr;
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser0](nvic_iser0) module"]
pub type NVIC_ISER0 = crate::Reg<u32, _NVIC_ISER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER0;
#[doc = "`read()` method returns [nvic_iser0::R](nvic_iser0::R) reader structure"]
impl crate::Readable for NVIC_ISER0 {}
#[doc = "`write(|w| ..)` method takes [nvic_iser0::W](nvic_iser0::W) writer structure"]
impl crate::Writable for NVIC_ISER0 {}
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_iser0;
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser1](nvic_iser1) module"]
pub type NVIC_ISER1 = crate::Reg<u32, _NVIC_ISER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER1;
#[doc = "`read()` method returns [nvic_iser1::R](nvic_iser1::R) reader structure"]
impl crate::Readable for NVIC_ISER1 {}
#[doc = "`write(|w| ..)` method takes [nvic_iser1::W](nvic_iser1::W) writer structure"]
impl crate::Writable for NVIC_ISER1 {}
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_iser1;
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icer0](nvic_icer0) module"]
pub type NVIC_ICER0 = crate::Reg<u32, _NVIC_ICER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER0;
#[doc = "`read()` method returns [nvic_icer0::R](nvic_icer0::R) reader structure"]
impl crate::Readable for NVIC_ICER0 {}
#[doc = "`write(|w| ..)` method takes [nvic_icer0::W](nvic_icer0::W) writer structure"]
impl crate::Writable for NVIC_ICER0 {}
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer0;
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icer1](nvic_icer1) module"]
pub type NVIC_ICER1 = crate::Reg<u32, _NVIC_ICER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER1;
#[doc = "`read()` method returns [nvic_icer1::R](nvic_icer1::R) reader structure"]
impl crate::Readable for NVIC_ICER1 {}
#[doc = "`write(|w| ..)` method takes [nvic_icer1::W](nvic_icer1::W) writer structure"]
impl crate::Writable for NVIC_ICER1 {}
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer1;
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr0](nvic_ispr0) module"]
pub type NVIC_ISPR0 = crate::Reg<u32, _NVIC_ISPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR0;
#[doc = "`read()` method returns [nvic_ispr0::R](nvic_ispr0::R) reader structure"]
impl crate::Readable for NVIC_ISPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr0::W](nvic_ispr0::W) writer structure"]
impl crate::Writable for NVIC_ISPR0 {}
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub mod nvic_ispr0;
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr1](nvic_ispr1) module"]
pub type NVIC_ISPR1 = crate::Reg<u32, _NVIC_ISPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR1;
#[doc = "`read()` method returns [nvic_ispr1::R](nvic_ispr1::R) reader structure"]
impl crate::Readable for NVIC_ISPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr1::W](nvic_ispr1::W) writer structure"]
impl crate::Writable for NVIC_ISPR1 {}
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub mod nvic_ispr1;
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icpr0](nvic_icpr0) module"]
pub type NVIC_ICPR0 = crate::Reg<u32, _NVIC_ICPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR0;
#[doc = "`read()` method returns [nvic_icpr0::R](nvic_icpr0::R) reader structure"]
impl crate::Readable for NVIC_ICPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr0::W](nvic_icpr0::W) writer structure"]
impl crate::Writable for NVIC_ICPR0 {}
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr0;
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icpr1](nvic_icpr1) module"]
pub type NVIC_ICPR1 = crate::Reg<u32, _NVIC_ICPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR1;
#[doc = "`read()` method returns [nvic_icpr1::R](nvic_icpr1::R) reader structure"]
impl crate::Readable for NVIC_ICPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr1::W](nvic_icpr1::W) writer structure"]
impl crate::Writable for NVIC_ICPR1 {}
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr1;
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iabr0](nvic_iabr0) module"]
pub type NVIC_IABR0 = crate::Reg<u32, _NVIC_IABR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IABR0;
#[doc = "`read()` method returns [nvic_iabr0::R](nvic_iabr0::R) reader structure"]
impl crate::Readable for NVIC_IABR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_iabr0::W](nvic_iabr0::W) writer structure"]
impl crate::Writable for NVIC_IABR0 {}
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub mod nvic_iabr0;
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iabr1](nvic_iabr1) module"]
pub type NVIC_IABR1 = crate::Reg<u32, _NVIC_IABR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IABR1;
#[doc = "`read()` method returns [nvic_iabr1::R](nvic_iabr1::R) reader structure"]
impl crate::Readable for NVIC_IABR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_iabr1::W](nvic_iabr1::W) writer structure"]
impl crate::Writable for NVIC_IABR1 {}
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub mod nvic_iabr1;
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr0](nvic_ipr0) module"]
pub type NVIC_IPR0 = crate::Reg<u32, _NVIC_IPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR0;
#[doc = "`read()` method returns [nvic_ipr0::R](nvic_ipr0::R) reader structure"]
impl crate::Readable for NVIC_IPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](nvic_ipr0::W) writer structure"]
impl crate::Writable for NVIC_IPR0 {}
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr0;
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr1](nvic_ipr1) module"]
pub type NVIC_IPR1 = crate::Reg<u32, _NVIC_IPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR1;
#[doc = "`read()` method returns [nvic_ipr1::R](nvic_ipr1::R) reader structure"]
impl crate::Readable for NVIC_IPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](nvic_ipr1::W) writer structure"]
impl crate::Writable for NVIC_IPR1 {}
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr1;
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr2](nvic_ipr2) module"]
pub type NVIC_IPR2 = crate::Reg<u32, _NVIC_IPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR2;
#[doc = "`read()` method returns [nvic_ipr2::R](nvic_ipr2::R) reader structure"]
impl crate::Readable for NVIC_IPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](nvic_ipr2::W) writer structure"]
impl crate::Writable for NVIC_IPR2 {}
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr2;
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr3](nvic_ipr3) module"]
pub type NVIC_IPR3 = crate::Reg<u32, _NVIC_IPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR3;
#[doc = "`read()` method returns [nvic_ipr3::R](nvic_ipr3::R) reader structure"]
impl crate::Readable for NVIC_IPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](nvic_ipr3::W) writer structure"]
impl crate::Writable for NVIC_IPR3 {}
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr3;
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr4](nvic_ipr4) module"]
pub type NVIC_IPR4 = crate::Reg<u32, _NVIC_IPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR4;
#[doc = "`read()` method returns [nvic_ipr4::R](nvic_ipr4::R) reader structure"]
impl crate::Readable for NVIC_IPR4 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](nvic_ipr4::W) writer structure"]
impl crate::Writable for NVIC_IPR4 {}
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr4;
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr5](nvic_ipr5) module"]
pub type NVIC_IPR5 = crate::Reg<u32, _NVIC_IPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR5;
#[doc = "`read()` method returns [nvic_ipr5::R](nvic_ipr5::R) reader structure"]
impl crate::Readable for NVIC_IPR5 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](nvic_ipr5::W) writer structure"]
impl crate::Writable for NVIC_IPR5 {}
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr5;
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr6](nvic_ipr6) module"]
pub type NVIC_IPR6 = crate::Reg<u32, _NVIC_IPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR6;
#[doc = "`read()` method returns [nvic_ipr6::R](nvic_ipr6::R) reader structure"]
impl crate::Readable for NVIC_IPR6 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](nvic_ipr6::W) writer structure"]
impl crate::Writable for NVIC_IPR6 {}
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr6;
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr7](nvic_ipr7) module"]
pub type NVIC_IPR7 = crate::Reg<u32, _NVIC_IPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR7;
#[doc = "`read()` method returns [nvic_ipr7::R](nvic_ipr7::R) reader structure"]
impl crate::Readable for NVIC_IPR7 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](nvic_ipr7::W) writer structure"]
impl crate::Writable for NVIC_IPR7 {}
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr7;
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr8](nvic_ipr8) module"]
pub type NVIC_IPR8 = crate::Reg<u32, _NVIC_IPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR8;
#[doc = "`read()` method returns [nvic_ipr8::R](nvic_ipr8::R) reader structure"]
impl crate::Readable for NVIC_IPR8 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr8::W](nvic_ipr8::W) writer structure"]
impl crate::Writable for NVIC_IPR8 {}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr8;
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr9](nvic_ipr9) module"]
pub type NVIC_IPR9 = crate::Reg<u32, _NVIC_IPR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR9;
#[doc = "`read()` method returns [nvic_ipr9::R](nvic_ipr9::R) reader structure"]
impl crate::Readable for NVIC_IPR9 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr9::W](nvic_ipr9::W) writer structure"]
impl crate::Writable for NVIC_IPR9 {}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr9;
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "`write(|w| ..)` method takes [cpuid::W](cpuid::W) writer structure"]
impl crate::Writable for CPUID {}
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
pub mod cpuid;
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
pub mod icsr;
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtor](vtor) module"]
pub type VTOR = crate::Reg<u32, _VTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTOR;
#[doc = "`read()` method returns [vtor::R](vtor::R) reader structure"]
impl crate::Readable for VTOR {}
#[doc = "`write(|w| ..)` method takes [vtor::W](vtor::W) writer structure"]
impl crate::Writable for VTOR {}
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
pub mod vtor;
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
pub mod aircr;
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
pub mod scr;
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
pub mod ccr;
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr1](shpr1) module"]
pub type SHPR1 = crate::Reg<u32, _SHPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR1;
#[doc = "`read()` method returns [shpr1::R](shpr1::R) reader structure"]
impl crate::Readable for SHPR1 {}
#[doc = "`write(|w| ..)` method takes [shpr1::W](shpr1::W) writer structure"]
impl crate::Writable for SHPR1 {}
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr1;
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr2;
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr3;
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](shcsr) module"]
pub type SHCSR = crate::Reg<u32, _SHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHCSR;
#[doc = "`read()` method returns [shcsr::R](shcsr::R) reader structure"]
impl crate::Readable for SHCSR {}
#[doc = "`write(|w| ..)` method takes [shcsr::W](shcsr::W) writer structure"]
impl crate::Writable for SHCSR {}
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
pub mod shcsr;
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](cfsr) module"]
pub type CFSR = crate::Reg<u32, _CFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFSR;
#[doc = "`read()` method returns [cfsr::R](cfsr::R) reader structure"]
impl crate::Readable for CFSR {}
#[doc = "`write(|w| ..)` method takes [cfsr::W](cfsr::W) writer structure"]
impl crate::Writable for CFSR {}
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
pub mod cfsr;
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](hfsr) module"]
pub type HFSR = crate::Reg<u32, _HFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFSR;
#[doc = "`read()` method returns [hfsr::R](hfsr::R) reader structure"]
impl crate::Readable for HFSR {}
#[doc = "`write(|w| ..)` method takes [hfsr::W](hfsr::W) writer structure"]
impl crate::Writable for HFSR {}
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
pub mod hfsr;
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](dfsr) module"]
pub type DFSR = crate::Reg<u32, _DFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSR;
#[doc = "`read()` method returns [dfsr::R](dfsr::R) reader structure"]
impl crate::Readable for DFSR {}
#[doc = "`write(|w| ..)` method takes [dfsr::W](dfsr::W) writer structure"]
impl crate::Writable for DFSR {}
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
pub mod dfsr;
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfar](mmfar) module"]
pub type MMFAR = crate::Reg<u32, _MMFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMFAR;
#[doc = "`read()` method returns [mmfar::R](mmfar::R) reader structure"]
impl crate::Readable for MMFAR {}
#[doc = "`write(|w| ..)` method takes [mmfar::W](mmfar::W) writer structure"]
impl crate::Writable for MMFAR {}
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
pub mod mmfar;
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfar](bfar) module"]
pub type BFAR = crate::Reg<u32, _BFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFAR;
#[doc = "`read()` method returns [bfar::R](bfar::R) reader structure"]
impl crate::Readable for BFAR {}
#[doc = "`write(|w| ..)` method takes [bfar::W](bfar::W) writer structure"]
impl crate::Writable for BFAR {}
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
pub mod bfar;
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsr](afsr) module"]
pub type AFSR = crate::Reg<u32, _AFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSR;
#[doc = "`read()` method returns [afsr::R](afsr::R) reader structure"]
impl crate::Readable for AFSR {}
#[doc = "`write(|w| ..)` method takes [afsr::W](afsr::W) writer structure"]
impl crate::Writable for AFSR {}
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
pub mod afsr;
#[doc = "Processor Feature 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr0](id_pfr0) module"]
pub type ID_PFR0 = crate::Reg<u32, _ID_PFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_PFR0;
#[doc = "`read()` method returns [id_pfr0::R](id_pfr0::R) reader structure"]
impl crate::Readable for ID_PFR0 {}
#[doc = "`write(|w| ..)` method takes [id_pfr0::W](id_pfr0::W) writer structure"]
impl crate::Writable for ID_PFR0 {}
#[doc = "Processor Feature 0"]
pub mod id_pfr0;
#[doc = "Processor Feature 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr1](id_pfr1) module"]
pub type ID_PFR1 = crate::Reg<u32, _ID_PFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_PFR1;
#[doc = "`read()` method returns [id_pfr1::R](id_pfr1::R) reader structure"]
impl crate::Readable for ID_PFR1 {}
#[doc = "`write(|w| ..)` method takes [id_pfr1::W](id_pfr1::W) writer structure"]
impl crate::Writable for ID_PFR1 {}
#[doc = "Processor Feature 1"]
pub mod id_pfr1;
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_dfr0](id_dfr0) module"]
pub type ID_DFR0 = crate::Reg<u32, _ID_DFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_DFR0;
#[doc = "`read()` method returns [id_dfr0::R](id_dfr0::R) reader structure"]
impl crate::Readable for ID_DFR0 {}
#[doc = "`write(|w| ..)` method takes [id_dfr0::W](id_dfr0::W) writer structure"]
impl crate::Writable for ID_DFR0 {}
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
pub mod id_dfr0;
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_afr0](id_afr0) module"]
pub type ID_AFR0 = crate::Reg<u32, _ID_AFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_AFR0;
#[doc = "`read()` method returns [id_afr0::R](id_afr0::R) reader structure"]
impl crate::Readable for ID_AFR0 {}
#[doc = "`write(|w| ..)` method takes [id_afr0::W](id_afr0::W) writer structure"]
impl crate::Writable for ID_AFR0 {}
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
pub mod id_afr0;
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr0](id_mmfr0) module"]
pub type ID_MMFR0 = crate::Reg<u32, _ID_MMFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR0;
#[doc = "`read()` method returns [id_mmfr0::R](id_mmfr0::R) reader structure"]
impl crate::Readable for ID_MMFR0 {}
#[doc = "`write(|w| ..)` method takes [id_mmfr0::W](id_mmfr0::W) writer structure"]
impl crate::Writable for ID_MMFR0 {}
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support."]
pub mod id_mmfr0;
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr1](id_mmfr1) module"]
pub type ID_MMFR1 = crate::Reg<u32, _ID_MMFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR1;
#[doc = "`read()` method returns [id_mmfr1::R](id_mmfr1::R) reader structure"]
impl crate::Readable for ID_MMFR1 {}
#[doc = "`write(|w| ..)` method takes [id_mmfr1::W](id_mmfr1::W) writer structure"]
impl crate::Writable for ID_MMFR1 {}
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support."]
pub mod id_mmfr1;
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr2](id_mmfr2) module"]
pub type ID_MMFR2 = crate::Reg<u32, _ID_MMFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR2;
#[doc = "`read()` method returns [id_mmfr2::R](id_mmfr2::R) reader structure"]
impl crate::Readable for ID_MMFR2 {}
#[doc = "`write(|w| ..)` method takes [id_mmfr2::W](id_mmfr2::W) writer structure"]
impl crate::Writable for ID_MMFR2 {}
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support."]
pub mod id_mmfr2;
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr3](id_mmfr3) module"]
pub type ID_MMFR3 = crate::Reg<u32, _ID_MMFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_MMFR3;
#[doc = "`read()` method returns [id_mmfr3::R](id_mmfr3::R) reader structure"]
impl crate::Readable for ID_MMFR3 {}
#[doc = "`write(|w| ..)` method takes [id_mmfr3::W](id_mmfr3::W) writer structure"]
impl crate::Writable for ID_MMFR3 {}
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support."]
pub mod id_mmfr3;
#[doc = "ISA Feature 0 Information on the instruction set attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar0](id_isar0) module"]
pub type ID_ISAR0 = crate::Reg<u32, _ID_ISAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR0;
#[doc = "`read()` method returns [id_isar0::R](id_isar0::R) reader structure"]
impl crate::Readable for ID_ISAR0 {}
#[doc = "`write(|w| ..)` method takes [id_isar0::W](id_isar0::W) writer structure"]
impl crate::Writable for ID_ISAR0 {}
#[doc = "ISA Feature 0 Information on the instruction set attributes register"]
pub mod id_isar0;
#[doc = "ISA Feature 1 Information on the instruction set attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar1](id_isar1) module"]
pub type ID_ISAR1 = crate::Reg<u32, _ID_ISAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR1;
#[doc = "`read()` method returns [id_isar1::R](id_isar1::R) reader structure"]
impl crate::Readable for ID_ISAR1 {}
#[doc = "`write(|w| ..)` method takes [id_isar1::W](id_isar1::W) writer structure"]
impl crate::Writable for ID_ISAR1 {}
#[doc = "ISA Feature 1 Information on the instruction set attributes register"]
pub mod id_isar1;
#[doc = "ISA Feature 2 Information on the instruction set attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar2](id_isar2) module"]
pub type ID_ISAR2 = crate::Reg<u32, _ID_ISAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR2;
#[doc = "`read()` method returns [id_isar2::R](id_isar2::R) reader structure"]
impl crate::Readable for ID_ISAR2 {}
#[doc = "`write(|w| ..)` method takes [id_isar2::W](id_isar2::W) writer structure"]
impl crate::Writable for ID_ISAR2 {}
#[doc = "ISA Feature 2 Information on the instruction set attributes register"]
pub mod id_isar2;
#[doc = "ISA Feature 3 Information on the instruction set attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar3](id_isar3) module"]
pub type ID_ISAR3 = crate::Reg<u32, _ID_ISAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR3;
#[doc = "`read()` method returns [id_isar3::R](id_isar3::R) reader structure"]
impl crate::Readable for ID_ISAR3 {}
#[doc = "`write(|w| ..)` method takes [id_isar3::W](id_isar3::W) writer structure"]
impl crate::Writable for ID_ISAR3 {}
#[doc = "ISA Feature 3 Information on the instruction set attributes register"]
pub mod id_isar3;
#[doc = "ISA Feature 4 Information on the instruction set attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar4](id_isar4) module"]
pub type ID_ISAR4 = crate::Reg<u32, _ID_ISAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_ISAR4;
#[doc = "`read()` method returns [id_isar4::R](id_isar4::R) reader structure"]
impl crate::Readable for ID_ISAR4 {}
#[doc = "`write(|w| ..)` method takes [id_isar4::W](id_isar4::W) writer structure"]
impl crate::Writable for ID_ISAR4 {}
#[doc = "ISA Feature 4 Information on the instruction set attributes register"]
pub mod id_isar4;
#[doc = "Coprocessor Access Control This register specifies the access privileges for coprocessors.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpacr](cpacr) module"]
pub type CPACR = crate::Reg<u32, _CPACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPACR;
#[doc = "`read()` method returns [cpacr::R](cpacr::R) reader structure"]
impl crate::Readable for CPACR {}
#[doc = "`write(|w| ..)` method takes [cpacr::W](cpacr::W) writer structure"]
impl crate::Writable for CPACR {}
#[doc = "Coprocessor Access Control This register specifies the access privileges for coprocessors."]
pub mod cpacr;
#[doc = "MPU Type This register indicates many regions the MPU supports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_type](mpu_type) module"]
pub type MPU_TYPE = crate::Reg<u32, _MPU_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_TYPE;
#[doc = "`read()` method returns [mpu_type::R](mpu_type::R) reader structure"]
impl crate::Readable for MPU_TYPE {}
#[doc = "`write(|w| ..)` method takes [mpu_type::W](mpu_type::W) writer structure"]
impl crate::Writable for MPU_TYPE {}
#[doc = "MPU Type This register indicates many regions the MPU supports."]
pub mod mpu_type;
#[doc = "MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_ctrl](mpu_ctrl) module"]
pub type MPU_CTRL = crate::Reg<u32, _MPU_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_CTRL;
#[doc = "`read()` method returns [mpu_ctrl::R](mpu_ctrl::R) reader structure"]
impl crate::Readable for MPU_CTRL {}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](mpu_ctrl::W) writer structure"]
impl crate::Writable for MPU_CTRL {}
#[doc = "MPU Control This register is used to enable the MPU, enable the default memory map (background region), and enable the MPU when in Hard Fault, Non-maskable Interrupt (NMI), and FAULTMASK escalated handlers. When the MPU is enabled, at least one region of the memory map must be enabled for the MPU to function unless the PRIVDEFENA bit is set. If the PRIVDEFENA bit is set and no regions are enabled, then only privileged code can operate. When the MPU is disabled, the default address map is used, as if no MPU is present. When the MPU is enabled, only the system partition and vector table loads are always accessible. Other areas are accessible based on regions and whether PRIVDEFENA is enabled. Unless HFNMIENA is set, the MPU is not enabled when the exception priority is -1 or -2. These priorities are only possible when in Hard fault, NMI, or when FAULTMASK is enabled. The HFNMIENA bit enables the MPU when operating with these two priorities."]
pub mod mpu_ctrl;
#[doc = "MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rnr](mpu_rnr) module"]
pub type MPU_RNR = crate::Reg<u32, _MPU_RNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RNR;
#[doc = "`read()` method returns [mpu_rnr::R](mpu_rnr::R) reader structure"]
impl crate::Readable for MPU_RNR {}
#[doc = "`write(|w| ..)` method takes [mpu_rnr::W](mpu_rnr::W) writer structure"]
impl crate::Writable for MPU_RNR {}
#[doc = "MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register."]
pub mod mpu_rnr;
#[doc = "MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar](mpu_rbar) module"]
pub type MPU_RBAR = crate::Reg<u32, _MPU_RBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR;
#[doc = "`read()` method returns [mpu_rbar::R](mpu_rbar::R) reader structure"]
impl crate::Readable for MPU_RBAR {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar::W](mpu_rbar::W) writer structure"]
impl crate::Writable for MPU_RBAR {}
#[doc = "MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word."]
pub mod mpu_rbar;
#[doc = "MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr](mpu_rasr) module"]
pub type MPU_RASR = crate::Reg<u32, _MPU_RASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR;
#[doc = "`read()` method returns [mpu_rasr::R](mpu_rasr::R) reader structure"]
impl crate::Readable for MPU_RASR {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr::W](mpu_rasr::W) writer structure"]
impl crate::Writable for MPU_RASR {}
#[doc = "MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0."]
pub mod mpu_rasr;
#[doc = "MPU Alias 1 Region Base Address Alias for MPU_RBAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar_a1](mpu_rbar_a1) module"]
pub type MPU_RBAR_A1 = crate::Reg<u32, _MPU_RBAR_A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR_A1;
#[doc = "`read()` method returns [mpu_rbar_a1::R](mpu_rbar_a1::R) reader structure"]
impl crate::Readable for MPU_RBAR_A1 {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a1::W](mpu_rbar_a1::W) writer structure"]
impl crate::Writable for MPU_RBAR_A1 {}
#[doc = "MPU Alias 1 Region Base Address Alias for MPU_RBAR"]
pub mod mpu_rbar_a1;
#[doc = "MPU Alias 1 Region Attribute and Size Alias for MPU_RASR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr_a1](mpu_rasr_a1) module"]
pub type MPU_RASR_A1 = crate::Reg<u32, _MPU_RASR_A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR_A1;
#[doc = "`read()` method returns [mpu_rasr_a1::R](mpu_rasr_a1::R) reader structure"]
impl crate::Readable for MPU_RASR_A1 {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a1::W](mpu_rasr_a1::W) writer structure"]
impl crate::Writable for MPU_RASR_A1 {}
#[doc = "MPU Alias 1 Region Attribute and Size Alias for MPU_RASR"]
pub mod mpu_rasr_a1;
#[doc = "MPU Alias 2 Region Base Address Alias for MPU_RBAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar_a2](mpu_rbar_a2) module"]
pub type MPU_RBAR_A2 = crate::Reg<u32, _MPU_RBAR_A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR_A2;
#[doc = "`read()` method returns [mpu_rbar_a2::R](mpu_rbar_a2::R) reader structure"]
impl crate::Readable for MPU_RBAR_A2 {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a2::W](mpu_rbar_a2::W) writer structure"]
impl crate::Writable for MPU_RBAR_A2 {}
#[doc = "MPU Alias 2 Region Base Address Alias for MPU_RBAR"]
pub mod mpu_rbar_a2;
#[doc = "MPU Alias 2 Region Attribute and Size Alias for MPU_RASR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr_a2](mpu_rasr_a2) module"]
pub type MPU_RASR_A2 = crate::Reg<u32, _MPU_RASR_A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR_A2;
#[doc = "`read()` method returns [mpu_rasr_a2::R](mpu_rasr_a2::R) reader structure"]
impl crate::Readable for MPU_RASR_A2 {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a2::W](mpu_rasr_a2::W) writer structure"]
impl crate::Writable for MPU_RASR_A2 {}
#[doc = "MPU Alias 2 Region Attribute and Size Alias for MPU_RASR"]
pub mod mpu_rasr_a2;
#[doc = "MPU Alias 3 Region Base Address Alias for MPU_RBAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rbar_a3](mpu_rbar_a3) module"]
pub type MPU_RBAR_A3 = crate::Reg<u32, _MPU_RBAR_A3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RBAR_A3;
#[doc = "`read()` method returns [mpu_rbar_a3::R](mpu_rbar_a3::R) reader structure"]
impl crate::Readable for MPU_RBAR_A3 {}
#[doc = "`write(|w| ..)` method takes [mpu_rbar_a3::W](mpu_rbar_a3::W) writer structure"]
impl crate::Writable for MPU_RBAR_A3 {}
#[doc = "MPU Alias 3 Region Base Address Alias for MPU_RBAR"]
pub mod mpu_rbar_a3;
#[doc = "MPU Alias 3 Region Attribute and Size Alias for MPU_RASR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr_a3](mpu_rasr_a3) module"]
pub type MPU_RASR_A3 = crate::Reg<u32, _MPU_RASR_A3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPU_RASR_A3;
#[doc = "`read()` method returns [mpu_rasr_a3::R](mpu_rasr_a3::R) reader structure"]
impl crate::Readable for MPU_RASR_A3 {}
#[doc = "`write(|w| ..)` method takes [mpu_rasr_a3::W](mpu_rasr_a3::W) writer structure"]
impl crate::Writable for MPU_RASR_A3 {}
#[doc = "MPU Alias 3 Region Attribute and Size Alias for MPU_RASR"]
pub mod mpu_rasr_a3;
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhcsr](dhcsr) module"]
pub type DHCSR = crate::Reg<u32, _DHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHCSR;
#[doc = "`read()` method returns [dhcsr::R](dhcsr::R) reader structure"]
impl crate::Readable for DHCSR {}
#[doc = "`write(|w| ..)` method takes [dhcsr::W](dhcsr::W) writer structure"]
impl crate::Writable for DHCSR {}
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
pub mod dhcsr;
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrsr](dcrsr) module"]
pub type DCRSR = crate::Reg<u32, _DCRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRSR;
#[doc = "`read()` method returns [dcrsr::R](dcrsr::R) reader structure"]
impl crate::Readable for DCRSR {}
#[doc = "`write(|w| ..)` method takes [dcrsr::W](dcrsr::W) writer structure"]
impl crate::Writable for DCRSR {}
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
pub mod dcrsr;
#[doc = "Debug Core Register Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrdr](dcrdr) module"]
pub type DCRDR = crate::Reg<u32, _DCRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRDR;
#[doc = "`read()` method returns [dcrdr::R](dcrdr::R) reader structure"]
impl crate::Readable for DCRDR {}
#[doc = "`write(|w| ..)` method takes [dcrdr::W](dcrdr::W) writer structure"]
impl crate::Writable for DCRDR {}
#[doc = "Debug Core Register Data"]
pub mod dcrdr;
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [demcr](demcr) module"]
pub type DEMCR = crate::Reg<u32, _DEMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEMCR;
#[doc = "`read()` method returns [demcr::R](demcr::R) reader structure"]
impl crate::Readable for DEMCR {}
#[doc = "`write(|w| ..)` method takes [demcr::W](demcr::W) writer structure"]
impl crate::Writable for DEMCR {}
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub mod demcr;
#[doc = "Software Trigger Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stir](stir) module"]
pub type STIR = crate::Reg<u32, _STIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIR;
#[doc = "`read()` method returns [stir::R](stir::R) reader structure"]
impl crate::Readable for STIR {}
#[doc = "`write(|w| ..)` method takes [stir::W](stir::W) writer structure"]
impl crate::Writable for STIR {}
#[doc = "Software Trigger Interrupt"]
pub mod stir;
#[doc = "Floating Point Context Control This register holds control data for the floating-point unit. Accessible only by privileged software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpccr](fpccr) module"]
pub type FPCCR = crate::Reg<u32, _FPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCCR;
#[doc = "`read()` method returns [fpccr::R](fpccr::R) reader structure"]
impl crate::Readable for FPCCR {}
#[doc = "`write(|w| ..)` method takes [fpccr::W](fpccr::W) writer structure"]
impl crate::Writable for FPCCR {}
#[doc = "Floating Point Context Control This register holds control data for the floating-point unit. Accessible only by privileged software."]
pub mod fpccr;
#[doc = "Floating-Point Context Address This register holds the location of the unpopulated floating-point register space allocated on an exception stack frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcar](fpcar) module"]
pub type FPCAR = crate::Reg<u32, _FPCAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCAR;
#[doc = "`read()` method returns [fpcar::R](fpcar::R) reader structure"]
impl crate::Readable for FPCAR {}
#[doc = "`write(|w| ..)` method takes [fpcar::W](fpcar::W) writer structure"]
impl crate::Writable for FPCAR {}
#[doc = "Floating-Point Context Address This register holds the location of the unpopulated floating-point register space allocated on an exception stack frame."]
pub mod fpcar;
#[doc = "Floating Point Default Status Control This register holds the default values for the floating-point status control data that the processor assigns to the FPSCR when it creates a new floating-point context. Accessible only by privileged software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdscr](fpdscr) module"]
pub type FPDSCR = crate::Reg<u32, _FPDSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPDSCR;
#[doc = "`read()` method returns [fpdscr::R](fpdscr::R) reader structure"]
impl crate::Readable for FPDSCR {}
#[doc = "`write(|w| ..)` method takes [fpdscr::W](fpdscr::W) writer structure"]
impl crate::Writable for FPDSCR {}
#[doc = "Floating Point Default Status Control This register holds the default values for the floating-point status control data that the processor assigns to the FPSCR when it creates a new floating-point context. Accessible only by privileged software."]
pub mod fpdscr;
#[doc = "Media and FP Feature 0 Describes the features provided by the Floating-point extension.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr0](mvfr0) module"]
pub type MVFR0 = crate::Reg<u32, _MVFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MVFR0;
#[doc = "`read()` method returns [mvfr0::R](mvfr0::R) reader structure"]
impl crate::Readable for MVFR0 {}
#[doc = "`write(|w| ..)` method takes [mvfr0::W](mvfr0::W) writer structure"]
impl crate::Writable for MVFR0 {}
#[doc = "Media and FP Feature 0 Describes the features provided by the Floating-point extension."]
pub mod mvfr0;
#[doc = "Media and FP Feature 1 Describes the features provided by the Floating-point extension.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr1](mvfr1) module"]
pub type MVFR1 = crate::Reg<u32, _MVFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MVFR1;
#[doc = "`read()` method returns [mvfr1::R](mvfr1::R) reader structure"]
impl crate::Readable for MVFR1 {}
#[doc = "`write(|w| ..)` method takes [mvfr1::W](mvfr1::W) writer structure"]
impl crate::Writable for MVFR1 {}
#[doc = "Media and FP Feature 1 Describes the features provided by the Floating-point extension."]
pub mod mvfr1;
