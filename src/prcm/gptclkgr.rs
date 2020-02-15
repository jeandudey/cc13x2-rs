#[doc = "Reader of register GPTCLKGR"]
pub type R = crate::R<u32, super::GPTCLKGR>;
#[doc = "Writer for register GPTCLKGR"]
pub type W = crate::W<u32, super::GPTCLKGR>;
#[doc = "Register GPTCLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTCLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AM_CLK_EN_A {
    #[doc = "8: Enable clock for GPT3  in all modes"]
    AM_GPT3 = 8,
    #[doc = "4: Enable clock for GPT2  in all modes"]
    AM_GPT2 = 4,
    #[doc = "2: Enable clock for GPT1  in all modes"]
    AM_GPT1 = 2,
    #[doc = "1: Enable clock for GPT0 in all modes"]
    AM_GPT0 = 1,
}
impl From<AM_CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_CLK_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AM_CLK_EN`"]
pub type AM_CLK_EN_R = crate::R<u8, AM_CLK_EN_A>;
impl AM_CLK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AM_CLK_EN_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(AM_CLK_EN_A::AM_GPT3),
            4 => Val(AM_CLK_EN_A::AM_GPT2),
            2 => Val(AM_CLK_EN_A::AM_GPT1),
            1 => Val(AM_CLK_EN_A::AM_GPT0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AM_GPT3`"]
    #[inline(always)]
    pub fn is_am_gpt3(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT3
    }
    #[doc = "Checks if the value of the field is `AM_GPT2`"]
    #[inline(always)]
    pub fn is_am_gpt2(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT2
    }
    #[doc = "Checks if the value of the field is `AM_GPT1`"]
    #[inline(always)]
    pub fn is_am_gpt1(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT1
    }
    #[doc = "Checks if the value of the field is `AM_GPT0`"]
    #[inline(always)]
    pub fn is_am_gpt0(&self) -> bool {
        *self == AM_CLK_EN_A::AM_GPT0
    }
}
#[doc = "Write proxy for field `AM_CLK_EN`"]
pub struct AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AM_CLK_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable clock for GPT3 in all modes"]
    #[inline(always)]
    pub fn am_gpt3(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT3)
    }
    #[doc = "Enable clock for GPT2 in all modes"]
    #[inline(always)]
    pub fn am_gpt2(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT2)
    }
    #[doc = "Enable clock for GPT1 in all modes"]
    #[inline(always)]
    pub fn am_gpt1(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT1)
    }
    #[doc = "Enable clock for GPT0 in all modes"]
    #[inline(always)]
    pub fn am_gpt0(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::AM_GPT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "8: Enable clock for GPT3"]
    GPT3 = 8,
    #[doc = "4: Enable clock for GPT2"]
    GPT2 = 4,
    #[doc = "2: Enable clock for GPT1"]
    GPT1 = 2,
    #[doc = "1: Enable clock for GPT0"]
    GPT0 = 1,
}
impl From<CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<u8, CLK_EN_A>;
impl CLK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_EN_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(CLK_EN_A::GPT3),
            4 => Val(CLK_EN_A::GPT2),
            2 => Val(CLK_EN_A::GPT1),
            1 => Val(CLK_EN_A::GPT0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPT3`"]
    #[inline(always)]
    pub fn is_gpt3(&self) -> bool {
        *self == CLK_EN_A::GPT3
    }
    #[doc = "Checks if the value of the field is `GPT2`"]
    #[inline(always)]
    pub fn is_gpt2(&self) -> bool {
        *self == CLK_EN_A::GPT2
    }
    #[doc = "Checks if the value of the field is `GPT1`"]
    #[inline(always)]
    pub fn is_gpt1(&self) -> bool {
        *self == CLK_EN_A::GPT1
    }
    #[doc = "Checks if the value of the field is `GPT0`"]
    #[inline(always)]
    pub fn is_gpt0(&self) -> bool {
        *self == CLK_EN_A::GPT0
    }
}
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable clock for GPT3"]
    #[inline(always)]
    pub fn gpt3(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT3)
    }
    #[doc = "Enable clock for GPT2"]
    #[inline(always)]
    pub fn gpt2(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT2)
    }
    #[doc = "Enable clock for GPT1"]
    #[inline(always)]
    pub fn gpt1(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT1)
    }
    #[doc = "Enable clock for GPT0"]
    #[inline(always)]
    pub fn gpt0(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AM_CLK_EN_R {
        AM_CLK_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Each bit below has the following meaning: 0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, GPTCLKGS.CLK_EN and GPTCLKGDS.CLK_EN when enabled. ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&mut self) -> AM_CLK_EN_W {
        AM_CLK_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
}
