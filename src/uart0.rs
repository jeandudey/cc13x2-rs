#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
    pub dr: DR,
    _reserved_1_ecr: [u8; 4usize],
    #[doc = "0x08 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved0: RESERVED0,
    _reserved3: [u8; 12usize],
    #[doc = "0x18 - Flag Reads from this register return the UART flags."]
    pub fr: FR,
    #[doc = "0x1c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved2: RESERVED2,
    _reserved5: [u8; 4usize],
    #[doc = "0x24 - Integer Baud-Rate Divisor If this register is modified while transmission or reception is on-going, the baud rate will not be updated until transmission or reception of the current character is complete."]
    pub ibrd: IBRD,
    #[doc = "0x28 - Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
    pub fbrd: FBRD,
    #[doc = "0x2c - Line Control"]
    pub lcrh: LCRH,
    #[doc = "0x30 - Control"]
    pub ctl: CTL,
    #[doc = "0x34 - Interrupt FIFO Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - Interrupt Mask Set/Clear"]
    pub imsc: IMSC,
    #[doc = "0x3c - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x40 - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    pub icr: ICR,
    #[doc = "0x48 - DMA Control"]
    pub dmactl: DMACTL,
    #[doc = "0x4c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved1: RESERVED1,
    _reserved16: [u8; 64usize],
    #[doc = "0x90 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved3: RESERVED3,
    _reserved17: [u8; 3900usize],
    #[doc = "0xfd0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved4: RESERVED4,
}
impl RegisterBlock {
    #[doc = "0x04 - Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
    #[inline(always)]
    pub fn ecr(&self) -> &ECR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const ECR) }
    }
    #[doc = "0x04 - Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
    #[inline(always)]
    pub fn ecr_mut(&self) -> &mut ECR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut ECR) }
    }
    #[doc = "0x04 - Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
    #[inline(always)]
    pub fn rsr(&self) -> &RSR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RSR) }
    }
    #[doc = "0x04 - Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
    #[inline(always)]
    pub fn rsr_mut(&self) -> &mut RSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut RSR) }
    }
}
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
pub mod dr;
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
pub mod rsr;
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
pub mod ecr;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved0](reserved0) module"]
pub type RESERVED0 = crate::Reg<u32, _RESERVED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED0;
#[doc = "`read()` method returns [reserved0::R](reserved0::R) reader structure"]
impl crate::Readable for RESERVED0 {}
#[doc = "`write(|w| ..)` method takes [reserved0::W](reserved0::W) writer structure"]
impl crate::Writable for RESERVED0 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved0;
#[doc = "Flag Reads from this register return the UART flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "`write(|w| ..)` method takes [fr::W](fr::W) writer structure"]
impl crate::Writable for FR {}
#[doc = "Flag Reads from this register return the UART flags."]
pub mod fr;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved2](reserved2) module"]
pub type RESERVED2 = crate::Reg<u32, _RESERVED2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED2;
#[doc = "`read()` method returns [reserved2::R](reserved2::R) reader structure"]
impl crate::Readable for RESERVED2 {}
#[doc = "`write(|w| ..)` method takes [reserved2::W](reserved2::W) writer structure"]
impl crate::Writable for RESERVED2 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved2;
#[doc = "Integer Baud-Rate Divisor If this register is modified while transmission or reception is on-going, the baud rate will not be updated until transmission or reception of the current character is complete.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](ibrd) module"]
pub type IBRD = crate::Reg<u32, _IBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBRD;
#[doc = "`read()` method returns [ibrd::R](ibrd::R) reader structure"]
impl crate::Readable for IBRD {}
#[doc = "`write(|w| ..)` method takes [ibrd::W](ibrd::W) writer structure"]
impl crate::Writable for IBRD {}
#[doc = "Integer Baud-Rate Divisor If this register is modified while transmission or reception is on-going, the baud rate will not be updated until transmission or reception of the current character is complete."]
pub mod ibrd;
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](fbrd) module"]
pub type FBRD = crate::Reg<u32, _FBRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBRD;
#[doc = "`read()` method returns [fbrd::R](fbrd::R) reader structure"]
impl crate::Readable for FBRD {}
#[doc = "`write(|w| ..)` method takes [fbrd::W](fbrd::W) writer structure"]
impl crate::Writable for FBRD {}
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub mod fbrd;
#[doc = "Line Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcrh](lcrh) module"]
pub type LCRH = crate::Reg<u32, _LCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCRH;
#[doc = "`read()` method returns [lcrh::R](lcrh::R) reader structure"]
impl crate::Readable for LCRH {}
#[doc = "`write(|w| ..)` method takes [lcrh::W](lcrh::W) writer structure"]
impl crate::Writable for LCRH {}
#[doc = "Line Control"]
pub mod lcrh;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Interrupt FIFO Level Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](ifls) module"]
pub type IFLS = crate::Reg<u32, _IFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLS;
#[doc = "`read()` method returns [ifls::R](ifls::R) reader structure"]
impl crate::Readable for IFLS {}
#[doc = "`write(|w| ..)` method takes [ifls::W](ifls::W) writer structure"]
impl crate::Writable for IFLS {}
#[doc = "Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "Interrupt Mask Set/Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imsc](imsc) module"]
pub type IMSC = crate::Reg<u32, _IMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMSC;
#[doc = "`read()` method returns [imsc::R](imsc::R) reader structure"]
impl crate::Readable for IMSC {}
#[doc = "`write(|w| ..)` method takes [imsc::W](imsc::W) writer structure"]
impl crate::Writable for IMSC {}
#[doc = "Interrupt Mask Set/Clear"]
pub mod imsc;
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod icr;
#[doc = "DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](dmactl) module"]
pub type DMACTL = crate::Reg<u32, _DMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL;
#[doc = "`read()` method returns [dmactl::R](dmactl::R) reader structure"]
impl crate::Readable for DMACTL {}
#[doc = "`write(|w| ..)` method takes [dmactl::W](dmactl::W) writer structure"]
impl crate::Writable for DMACTL {}
#[doc = "DMA Control"]
pub mod dmactl;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved1](reserved1) module"]
pub type RESERVED1 = crate::Reg<u32, _RESERVED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED1;
#[doc = "`read()` method returns [reserved1::R](reserved1::R) reader structure"]
impl crate::Readable for RESERVED1 {}
#[doc = "`write(|w| ..)` method takes [reserved1::W](reserved1::W) writer structure"]
impl crate::Writable for RESERVED1 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved3](reserved3) module"]
pub type RESERVED3 = crate::Reg<u32, _RESERVED3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED3;
#[doc = "`read()` method returns [reserved3::R](reserved3::R) reader structure"]
impl crate::Readable for RESERVED3 {}
#[doc = "`write(|w| ..)` method takes [reserved3::W](reserved3::W) writer structure"]
impl crate::Writable for RESERVED3 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved3;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved4](reserved4) module"]
pub type RESERVED4 = crate::Reg<u32, _RESERVED4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED4;
#[doc = "`read()` method returns [reserved4::R](reserved4::R) reader structure"]
impl crate::Readable for RESERVED4 {}
#[doc = "`write(|w| ..)` method takes [reserved4::W](reserved4::W) writer structure"]
impl crate::Writable for RESERVED4 {}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved4;
