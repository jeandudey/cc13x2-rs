#[doc = "Reader of register LCRH"]
pub type R = crate::R<u32, super::LCRH>;
#[doc = "Writer for register LCRH"]
pub type W = crate::W<u32, super::LCRH>;
#[doc = "Register LCRH `reset()`'s with value 0"]
impl crate::ResetValue for super::LCRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPS`"]
pub type SPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPS`"]
pub struct SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLEN_A {
    #[doc = "3: Word Length 8 bits"]
    _8 = 3,
    #[doc = "2: Word Length 7 bits"]
    _7 = 2,
    #[doc = "1: Word Length 6 bits"]
    _6 = 1,
    #[doc = "0: Word Length 5 bits"]
    _5 = 0,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WLEN`"]
pub type WLEN_R = crate::R<u8, WLEN_A>;
impl WLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLEN_A {
        match self.bits {
            3 => WLEN_A::_8,
            2 => WLEN_A::_7,
            1 => WLEN_A::_6,
            0 => WLEN_A::_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == WLEN_A::_8
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == WLEN_A::_7
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == WLEN_A::_6
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == WLEN_A::_5
    }
}
#[doc = "Write proxy for field `WLEN`"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Word Length 8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(WLEN_A::_8)
    }
    #[doc = "Word Length 7 bits"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(WLEN_A::_7)
    }
    #[doc = "Word Length 6 bits"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(WLEN_A::_6)
    }
    #[doc = "Word Length 5 bits"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(WLEN_A::_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "4:4\\]
UART Enable FIFOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEN_A {
    #[doc = "1: Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    EN = 1,
    #[doc = "0: FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    DIS = 0,
}
impl From<FEN_A> for bool {
    #[inline(always)]
    fn from(variant: FEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEN`"]
pub type FEN_R = crate::R<bool, FEN_A>;
impl FEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEN_A {
        match self.bits {
            true => FEN_A::EN,
            false => FEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FEN_A::DIS
    }
}
#[doc = "Write proxy for field `FEN`"]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FEN_A::EN)
    }
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FEN_A::DIS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STP2`"]
pub type STP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STP2`"]
pub struct STP2_W<'a> {
    w: &'a mut W,
}
impl<'a> STP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "2:2\\]
UART Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPS_A {
    #[doc = "1: Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    EVEN = 1,
    #[doc = "0: Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    ODD = 0,
}
impl From<EPS_A> for bool {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPS`"]
pub type EPS_R = crate::R<bool, EPS_A>;
impl EPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPS_A {
        match self.bits {
            true => EPS_A::EVEN,
            false => EPS_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == EPS_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == EPS_A::ODD
    }
}
#[doc = "Write proxy for field `EPS`"]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(EPS_A::EVEN)
    }
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPS_A::ODD)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN_A {
    #[doc = "1: Parity checking and generation is enabled."]
    EN = 1,
    #[doc = "0: Parity is disabled and no parity bit is added to the data frame"]
    DIS = 0,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN`"]
pub type PEN_R = crate::R<bool, PEN_A>;
impl PEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            true => PEN_A::EN,
            false => PEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PEN_A::DIS
    }
}
#[doc = "Write proxy for field `PEN`"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Parity checking and generation is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PEN_A::EN)
    }
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PEN_A::DIS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BRK`"]
pub type BRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRK`"]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
UART Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
UART Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W {
        STP2_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
UART Even Parity Select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
}
