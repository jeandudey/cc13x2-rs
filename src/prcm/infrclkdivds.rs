#[doc = "Reader of register INFRCLKDIVDS"]
pub type R = crate::R<u32, super::INFRCLKDIVDS>;
#[doc = "Writer for register INFRCLKDIVDS"]
pub type W = crate::W<u32, super::INFRCLKDIVDS>;
#[doc = "Register INFRCLKDIVDS `reset()`'s with value 0"]
impl crate::ResetValue for super::INFRCLKDIVDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "3: Divide by 32"]
    DIV32 = 3,
    #[doc = "2: Divide by 8"]
    DIV8 = 2,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "0: Divide by 1"]
    DIV1 = 0,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<u8, RATIO_A>;
impl RATIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            3 => RATIO_A::DIV32,
            2 => RATIO_A::DIV8,
            1 => RATIO_A::DIV2,
            0 => RATIO_A::DIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RATIO_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RATIO_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RATIO_A::DIV1
    }
}
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIO_A::DIV32)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIO_A::DIV8)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV2)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
