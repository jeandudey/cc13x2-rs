#[doc = "Reader of register AUXSCECLK"]
pub type R = crate::R<u32, super::AUXSCECLK>;
#[doc = "Writer for register AUXSCECLK"]
pub type W = crate::W<u32, super::AUXSCECLK>;
#[doc = "Register AUXSCECLK `reset()`'s with value 0"]
impl crate::ResetValue for super::AUXSCECLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_SRC_A {
    #[doc = "1: LF clock (SCLK_LF )"]
    SCLK_LF = 1,
    #[doc = "0: No clock"]
    NO_CLOCK = 0,
}
impl From<PD_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PD_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PD_SRC`"]
pub type PD_SRC_R = crate::R<bool, PD_SRC_A>;
impl PD_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_SRC_A {
        match self.bits {
            true => PD_SRC_A::SCLK_LF,
            false => PD_SRC_A::NO_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PD_SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PD_SRC_A::NO_CLOCK
    }
}
#[doc = "Write proxy for field `PD_SRC`"]
pub struct PD_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LF clock (SCLK_LF )"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PD_SRC_A::SCLK_LF)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(PD_SRC_A::NO_CLOCK)
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
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "1: MF Clock (SCLK_MF)"]
    SCLK_MF = 1,
    #[doc = "0: HF Clock divided by 2 (SCLK_HFDIV2)"]
    SCLK_HFDIV2 = 0,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            true => SRC_A::SCLK_MF,
            false => SRC_A::SCLK_HFDIV2,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_MF`"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == SRC_A::SCLK_MF
    }
    #[doc = "Checks if the value of the field is `SCLK_HFDIV2`"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == SRC_A::SCLK_HFDIV2
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MF Clock (SCLK_MF)"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_MF)
    }
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_HFDIV2)
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
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn pd_src(&self) -> PD_SRC_R {
        PD_SRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn pd_src(&mut self) -> PD_SRC_W {
        PD_SRC_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
