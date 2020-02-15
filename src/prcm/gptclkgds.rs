#[doc = "Reader of register GPTCLKGDS"]
pub type R = crate::R<u32, super::GPTCLKGDS>;
#[doc = "Writer for register GPTCLKGDS"]
pub type W = crate::W<u32, super::GPTCLKGDS>;
#[doc = "Register GPTCLKGDS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTCLKGDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by GPTCLKGR.AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
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
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by GPTCLKGR.AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock Can be forced on by GPTCLKGR.AM_CLK_EN ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
}
