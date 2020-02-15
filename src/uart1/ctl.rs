#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0x0300"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "15:15\\]
CTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "1: CTS hardware flow control enabled"]
    EN = 1,
    #[doc = "0: CTS hardware flow control disabled"]
    DIS = 0,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, CTSEN_A>;
impl CTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            true => CTSEN_A::EN,
            false => CTSEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CTSEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CTSEN_A::DIS
    }
}
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CTS hardware flow control enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CTSEN_A::EN)
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CTSEN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "14:14\\]
RTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_A {
    #[doc = "1: RTS hardware flow control enabled"]
    EN = 1,
    #[doc = "0: RTS hardware flow control disabled"]
    DIS = 0,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTSEN`"]
pub type RTSEN_R = crate::R<bool, RTSEN_A>;
impl RTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            true => RTSEN_A::EN,
            false => RTSEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RTSEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RTSEN_A::DIS
    }
}
#[doc = "Write proxy for field `RTSEN`"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTS hardware flow control enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RTSEN_A::EN)
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RTSEN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `RTS`"]
pub type RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTS`"]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXE_A {
    #[doc = "1: UART Receive enabled"]
    EN = 1,
    #[doc = "0: UART Receive disabled"]
    DIS = 0,
}
impl From<RXE_A> for bool {
    #[inline(always)]
    fn from(variant: RXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, RXE_A>;
impl RXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXE_A {
        match self.bits {
            true => RXE_A::EN,
            false => RXE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RXE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RXE_A::DIS
    }
}
#[doc = "Write proxy for field `RXE`"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART Receive enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RXE_A::EN)
    }
    #[doc = "UART Receive disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RXE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "1: UART Transmit enabled"]
    EN = 1,
    #[doc = "0: UART Transmit disabled"]
    DIS = 0,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, TXE_A>;
impl TXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            true => TXE_A::EN,
            false => TXE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TXE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TXE_A::DIS
    }
}
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART Transmit enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TXE_A::EN)
    }
    #[doc = "UART Transmit disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TXE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBE_A {
    #[doc = "1: Loop Back enabled"]
    EN = 1,
    #[doc = "0: Loop Back disabled"]
    DIS = 0,
}
impl From<LBE_A> for bool {
    #[inline(always)]
    fn from(variant: LBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBE`"]
pub type LBE_R = crate::R<bool, LBE_A>;
impl LBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBE_A {
        match self.bits {
            true => LBE_A::EN,
            false => LBE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LBE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LBE_A::DIS
    }
}
#[doc = "Write proxy for field `LBE`"]
pub struct LBE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LBE_A::EN)
    }
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LBE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
#[doc = "0:0\\]
UART Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTEN_A {
    #[doc = "1: UART enabled"]
    EN = 1,
    #[doc = "0: UART disabled"]
    DIS = 0,
}
impl From<UARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UARTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UARTEN`"]
pub type UARTEN_R = crate::R<bool, UARTEN_A>;
impl UARTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTEN_A {
        match self.bits {
            true => UARTEN_A::EN,
            false => UARTEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UARTEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UARTEN_A::DIS
    }
}
#[doc = "Write proxy for field `UARTEN`"]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UARTEN_A::EN)
    }
    #[doc = "UART disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UARTEN_A::DIS)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
    #[inline(always)]
    pub fn lbe(&self) -> LBE_R {
        LBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
UART Enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
    #[inline(always)]
    pub fn lbe(&mut self) -> LBE_W {
        LBE_W { w: self }
    }
    #[doc = "Bits 1:6 - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
UART Enable"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
}
