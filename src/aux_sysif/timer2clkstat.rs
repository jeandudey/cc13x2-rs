#[doc = "Reader of register TIMER2CLKSTAT"]
pub type R = crate::R<u32, super::TIMER2CLKSTAT>;
#[doc = "Writer for register TIMER2CLKSTAT"]
pub type W = crate::W<u32, super::TIMER2CLKSTAT>;
#[doc = "Register TIMER2CLKSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER2CLKSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "2:0\\]
AUX_TIMER2 clock source status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_A {
    #[doc = "4: SCLK_HF / 2"]
    SCLK_HFDIV2 = 4,
    #[doc = "2: SCLK_MF"]
    SCLK_MF = 2,
    #[doc = "1: SCLK_LF"]
    SCLK_LF = 1,
    #[doc = "0: No clock"]
    NONE = 0,
}
impl From<STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<u8, STAT_A>;
impl STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STAT_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(STAT_A::SCLK_HFDIV2),
            2 => Val(STAT_A::SCLK_MF),
            1 => Val(STAT_A::SCLK_LF),
            0 => Val(STAT_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == STAT_A::SCLK_HFDIV2
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == STAT_A::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == STAT_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == STAT_A::NONE
    }
}
#[doc = "Write proxy for field `STAT`"]
pub struct STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCLK_HF / 2"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(STAT_A::SCLK_HFDIV2)
    }
    #[doc = "SCLK_MF"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(STAT_A::SCLK_MF)
    }
    #[doc = "SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(STAT_A::SCLK_LF)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(STAT_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
AUX_TIMER2 clock source status."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
AUX_TIMER2 clock source status."]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W {
        STAT_W { w: self }
    }
}
