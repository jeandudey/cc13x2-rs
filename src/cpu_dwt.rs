#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Use the DWT Control Register to enable the DWT unit."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
    pub cyccnt: CYCCNT,
    #[doc = "0x08 - CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
    pub cpicnt: CPICNT,
    #[doc = "0x0c - Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
    pub exccnt: EXCCNT,
    #[doc = "0x10 - Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
    pub sleepcnt: SLEEPCNT,
    #[doc = "0x14 - LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
    pub lsucnt: LSUCNT,
    #[doc = "0x18 - Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
    pub foldcnt: FOLDCNT,
    #[doc = "0x1c - Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
    pub pcsr: PCSR,
    #[doc = "0x20 - Comparator 0 This register is used to write the reference value for comparator 0."]
    pub comp0: COMP0,
    #[doc = "0x24 - Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
    pub mask0: MASK0,
    #[doc = "0x28 - Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function0: FUNCTION0,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - Comparator 1 This register is used to write the reference value for comparator 1."]
    pub comp1: COMP1,
    #[doc = "0x34 - Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
    pub mask1: MASK1,
    #[doc = "0x38 - Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function1: FUNCTION1,
    _reserved14: [u8; 4usize],
    #[doc = "0x40 - Comparator 2 This register is used to write the reference value for comparator 2."]
    pub comp2: COMP2,
    #[doc = "0x44 - Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
    pub mask2: MASK2,
    #[doc = "0x48 - Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function2: FUNCTION2,
    _reserved17: [u8; 4usize],
    #[doc = "0x50 - Comparator 3 This register is used to write the reference value for comparator 3."]
    pub comp3: COMP3,
    #[doc = "0x54 - Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
    pub mask3: MASK3,
    #[doc = "0x58 - Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function3: FUNCTION3,
}
#[doc = "Control Use the DWT Control Register to enable the DWT unit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Use the DWT Control Register to enable the DWT unit."]
pub mod ctrl;
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cyccnt](cyccnt) module"]
pub type CYCCNT = crate::Reg<u32, _CYCCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CYCCNT;
#[doc = "`read()` method returns [cyccnt::R](cyccnt::R) reader structure"]
impl crate::Readable for CYCCNT {}
#[doc = "`write(|w| ..)` method takes [cyccnt::W](cyccnt::W) writer structure"]
impl crate::Writable for CYCCNT {}
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
pub mod cyccnt;
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpicnt](cpicnt) module"]
pub type CPICNT = crate::Reg<u32, _CPICNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPICNT;
#[doc = "`read()` method returns [cpicnt::R](cpicnt::R) reader structure"]
impl crate::Readable for CPICNT {}
#[doc = "`write(|w| ..)` method takes [cpicnt::W](cpicnt::W) writer structure"]
impl crate::Writable for CPICNT {}
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
pub mod cpicnt;
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exccnt](exccnt) module"]
pub type EXCCNT = crate::Reg<u32, _EXCCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXCCNT;
#[doc = "`read()` method returns [exccnt::R](exccnt::R) reader structure"]
impl crate::Readable for EXCCNT {}
#[doc = "`write(|w| ..)` method takes [exccnt::W](exccnt::W) writer structure"]
impl crate::Writable for EXCCNT {}
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
pub mod exccnt;
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcnt](sleepcnt) module"]
pub type SLEEPCNT = crate::Reg<u32, _SLEEPCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCNT;
#[doc = "`read()` method returns [sleepcnt::R](sleepcnt::R) reader structure"]
impl crate::Readable for SLEEPCNT {}
#[doc = "`write(|w| ..)` method takes [sleepcnt::W](sleepcnt::W) writer structure"]
impl crate::Writable for SLEEPCNT {}
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
pub mod sleepcnt;
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsucnt](lsucnt) module"]
pub type LSUCNT = crate::Reg<u32, _LSUCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSUCNT;
#[doc = "`read()` method returns [lsucnt::R](lsucnt::R) reader structure"]
impl crate::Readable for LSUCNT {}
#[doc = "`write(|w| ..)` method takes [lsucnt::W](lsucnt::W) writer structure"]
impl crate::Writable for LSUCNT {}
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
pub mod lsucnt;
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [foldcnt](foldcnt) module"]
pub type FOLDCNT = crate::Reg<u32, _FOLDCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOLDCNT;
#[doc = "`read()` method returns [foldcnt::R](foldcnt::R) reader structure"]
impl crate::Readable for FOLDCNT {}
#[doc = "`write(|w| ..)` method takes [foldcnt::W](foldcnt::W) writer structure"]
impl crate::Writable for FOLDCNT {}
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
pub mod foldcnt;
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr](pcsr) module"]
pub type PCSR = crate::Reg<u32, _PCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSR;
#[doc = "`read()` method returns [pcsr::R](pcsr::R) reader structure"]
impl crate::Readable for PCSR {}
#[doc = "`write(|w| ..)` method takes [pcsr::W](pcsr::W) writer structure"]
impl crate::Writable for PCSR {}
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
pub mod pcsr;
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp0](comp0) module"]
pub type COMP0 = crate::Reg<u32, _COMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP0;
#[doc = "`read()` method returns [comp0::R](comp0::R) reader structure"]
impl crate::Readable for COMP0 {}
#[doc = "`write(|w| ..)` method takes [comp0::W](comp0::W) writer structure"]
impl crate::Writable for COMP0 {}
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
pub mod comp0;
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](mask0) module"]
pub type MASK0 = crate::Reg<u32, _MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK0;
#[doc = "`read()` method returns [mask0::R](mask0::R) reader structure"]
impl crate::Readable for MASK0 {}
#[doc = "`write(|w| ..)` method takes [mask0::W](mask0::W) writer structure"]
impl crate::Writable for MASK0 {}
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
pub mod mask0;
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function0](function0) module"]
pub type FUNCTION0 = crate::Reg<u32, _FUNCTION0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION0;
#[doc = "`read()` method returns [function0::R](function0::R) reader structure"]
impl crate::Readable for FUNCTION0 {}
#[doc = "`write(|w| ..)` method takes [function0::W](function0::W) writer structure"]
impl crate::Writable for FUNCTION0 {}
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function0;
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1](comp1) module"]
pub type COMP1 = crate::Reg<u32, _COMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1;
#[doc = "`read()` method returns [comp1::R](comp1::R) reader structure"]
impl crate::Readable for COMP1 {}
#[doc = "`write(|w| ..)` method takes [comp1::W](comp1::W) writer structure"]
impl crate::Writable for COMP1 {}
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
pub mod comp1;
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask1](mask1) module"]
pub type MASK1 = crate::Reg<u32, _MASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK1;
#[doc = "`read()` method returns [mask1::R](mask1::R) reader structure"]
impl crate::Readable for MASK1 {}
#[doc = "`write(|w| ..)` method takes [mask1::W](mask1::W) writer structure"]
impl crate::Writable for MASK1 {}
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
pub mod mask1;
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function1](function1) module"]
pub type FUNCTION1 = crate::Reg<u32, _FUNCTION1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION1;
#[doc = "`read()` method returns [function1::R](function1::R) reader structure"]
impl crate::Readable for FUNCTION1 {}
#[doc = "`write(|w| ..)` method takes [function1::W](function1::W) writer structure"]
impl crate::Writable for FUNCTION1 {}
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function1;
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2](comp2) module"]
pub type COMP2 = crate::Reg<u32, _COMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2;
#[doc = "`read()` method returns [comp2::R](comp2::R) reader structure"]
impl crate::Readable for COMP2 {}
#[doc = "`write(|w| ..)` method takes [comp2::W](comp2::W) writer structure"]
impl crate::Writable for COMP2 {}
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
pub mod comp2;
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask2](mask2) module"]
pub type MASK2 = crate::Reg<u32, _MASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK2;
#[doc = "`read()` method returns [mask2::R](mask2::R) reader structure"]
impl crate::Readable for MASK2 {}
#[doc = "`write(|w| ..)` method takes [mask2::W](mask2::W) writer structure"]
impl crate::Writable for MASK2 {}
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
pub mod mask2;
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function2](function2) module"]
pub type FUNCTION2 = crate::Reg<u32, _FUNCTION2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION2;
#[doc = "`read()` method returns [function2::R](function2::R) reader structure"]
impl crate::Readable for FUNCTION2 {}
#[doc = "`write(|w| ..)` method takes [function2::W](function2::W) writer structure"]
impl crate::Writable for FUNCTION2 {}
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function2;
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp3](comp3) module"]
pub type COMP3 = crate::Reg<u32, _COMP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP3;
#[doc = "`read()` method returns [comp3::R](comp3::R) reader structure"]
impl crate::Readable for COMP3 {}
#[doc = "`write(|w| ..)` method takes [comp3::W](comp3::W) writer structure"]
impl crate::Writable for COMP3 {}
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
pub mod comp3;
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask3](mask3) module"]
pub type MASK3 = crate::Reg<u32, _MASK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK3;
#[doc = "`read()` method returns [mask3::R](mask3::R) reader structure"]
impl crate::Readable for MASK3 {}
#[doc = "`write(|w| ..)` method takes [mask3::W](mask3::W) writer structure"]
impl crate::Writable for MASK3 {}
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
pub mod mask3;
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function3](function3) module"]
pub type FUNCTION3 = crate::Reg<u32, _FUNCTION3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION3;
#[doc = "`read()` method returns [function3::R](function3::R) reader structure"]
impl crate::Readable for FUNCTION3 {}
#[doc = "`write(|w| ..)` method takes [function3::W](function3::W) writer structure"]
impl crate::Writable for FUNCTION3 {}
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function3;
