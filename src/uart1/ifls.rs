#[doc = "Reader of register IFLS"]
pub type R = crate::R<u32, super::IFLS>;
#[doc = "Writer for register IFLS"]
pub type W = crate::W<u32, super::IFLS>;
#[doc = "Register IFLS `reset()`'s with value 0x12"]
impl crate::ResetValue for super::IFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x12
    }
}
#[doc = "5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXSEL_A {
    #[doc = "4: Receive FIFO becomes >= 7/8 full"]
    _7_8 = 4,
    #[doc = "3: Receive FIFO becomes >= 3/4 full"]
    _6_8 = 3,
    #[doc = "2: Receive FIFO becomes >= 1/2 full"]
    _4_8 = 2,
    #[doc = "1: Receive FIFO becomes >= 1/4 full"]
    _2_8 = 1,
    #[doc = "0: Receive FIFO becomes >= 1/8 full"]
    _1_8 = 0,
}
impl From<RXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXSEL`"]
pub type RXSEL_R = crate::R<u8, RXSEL_A>;
impl RXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(RXSEL_A::_7_8),
            3 => Val(RXSEL_A::_6_8),
            2 => Val(RXSEL_A::_4_8),
            1 => Val(RXSEL_A::_2_8),
            0 => Val(RXSEL_A::_1_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7_8`"]
    #[inline(always)]
    pub fn is_7_8(&self) -> bool {
        *self == RXSEL_A::_7_8
    }
    #[doc = "Checks if the value of the field is `_6_8`"]
    #[inline(always)]
    pub fn is_6_8(&self) -> bool {
        *self == RXSEL_A::_6_8
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline(always)]
    pub fn is_4_8(&self) -> bool {
        *self == RXSEL_A::_4_8
    }
    #[doc = "Checks if the value of the field is `_2_8`"]
    #[inline(always)]
    pub fn is_2_8(&self) -> bool {
        *self == RXSEL_A::_2_8
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == RXSEL_A::_1_8
    }
}
#[doc = "Write proxy for field `RXSEL`"]
pub struct RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Receive FIFO becomes >= 7/8 full"]
    #[inline(always)]
    pub fn _7_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_7_8)
    }
    #[doc = "Receive FIFO becomes >= 3/4 full"]
    #[inline(always)]
    pub fn _6_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_6_8)
    }
    #[doc = "Receive FIFO becomes >= 1/2 full"]
    #[inline(always)]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_4_8)
    }
    #[doc = "Receive FIFO becomes >= 1/4 full"]
    #[inline(always)]
    pub fn _2_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_2_8)
    }
    #[doc = "Receive FIFO becomes >= 1/8 full"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_1_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXSEL_A {
    #[doc = "4: Transmit FIFO becomes <= 7/8 full"]
    _7_8 = 4,
    #[doc = "3: Transmit FIFO becomes <= 3/4 full"]
    _6_8 = 3,
    #[doc = "2: Transmit FIFO becomes <= 1/2 full"]
    _4_8 = 2,
    #[doc = "1: Transmit FIFO becomes <= 1/4 full"]
    _2_8 = 1,
    #[doc = "0: Transmit FIFO becomes <= 1/8 full"]
    _1_8 = 0,
}
impl From<TXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXSEL`"]
pub type TXSEL_R = crate::R<u8, TXSEL_A>;
impl TXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(TXSEL_A::_7_8),
            3 => Val(TXSEL_A::_6_8),
            2 => Val(TXSEL_A::_4_8),
            1 => Val(TXSEL_A::_2_8),
            0 => Val(TXSEL_A::_1_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_7_8`"]
    #[inline(always)]
    pub fn is_7_8(&self) -> bool {
        *self == TXSEL_A::_7_8
    }
    #[doc = "Checks if the value of the field is `_6_8`"]
    #[inline(always)]
    pub fn is_6_8(&self) -> bool {
        *self == TXSEL_A::_6_8
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline(always)]
    pub fn is_4_8(&self) -> bool {
        *self == TXSEL_A::_4_8
    }
    #[doc = "Checks if the value of the field is `_2_8`"]
    #[inline(always)]
    pub fn is_2_8(&self) -> bool {
        *self == TXSEL_A::_2_8
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == TXSEL_A::_1_8
    }
}
#[doc = "Write proxy for field `TXSEL`"]
pub struct TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transmit FIFO becomes <= 7/8 full"]
    #[inline(always)]
    pub fn _7_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_7_8)
    }
    #[doc = "Transmit FIFO becomes <= 3/4 full"]
    #[inline(always)]
    pub fn _6_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_6_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/2 full"]
    #[inline(always)]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_4_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/4 full"]
    #[inline(always)]
    pub fn _2_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_2_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/8 full"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_1_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn txsel(&self) -> TXSEL_R {
        TXSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn rxsel(&mut self) -> RXSEL_W {
        RXSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn txsel(&mut self) -> TXSEL_W {
        TXSEL_W { w: self }
    }
}
