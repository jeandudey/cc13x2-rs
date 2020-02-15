#[doc = "Reader of register SSICLKGR"]
pub type R = crate::R<u32, super::SSICLKGR>;
#[doc = "Writer for register SSICLKGR"]
pub type W = crate::W<u32, super::SSICLKGR>;
#[doc = "Register SSICLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSICLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SSICLKGS.CLK_EN and SSICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AM_CLK_EN_A {
    #[doc = "2: Enable clock for SSI1"]
    SSI1 = 2,
    #[doc = "1: Enable clock for SSI0"]
    SSI0 = 1,
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
            2 => Val(AM_CLK_EN_A::SSI1),
            1 => Val(AM_CLK_EN_A::SSI0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI1`"]
    #[inline(always)]
    pub fn is_ssi1(&self) -> bool {
        *self == AM_CLK_EN_A::SSI1
    }
    #[doc = "Checks if the value of the field is `SSI0`"]
    #[inline(always)]
    pub fn is_ssi0(&self) -> bool {
        *self == AM_CLK_EN_A::SSI0
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
    #[doc = "Enable clock for SSI1"]
    #[inline(always)]
    pub fn ssi1(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::SSI1)
    }
    #[doc = "Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::SSI0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "2: Enable clock for SSI1"]
    SSI1 = 2,
    #[doc = "1: Enable clock for SSI0"]
    SSI0 = 1,
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
            2 => Val(CLK_EN_A::SSI1),
            1 => Val(CLK_EN_A::SSI0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI1`"]
    #[inline(always)]
    pub fn is_ssi1(&self) -> bool {
        *self == CLK_EN_A::SSI1
    }
    #[doc = "Checks if the value of the field is `SSI0`"]
    #[inline(always)]
    pub fn is_ssi0(&self) -> bool {
        *self == CLK_EN_A::SSI0
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
    #[doc = "Enable clock for SSI1"]
    #[inline(always)]
    pub fn ssi1(self) -> &'a mut W {
        self.variant(CLK_EN_A::SSI1)
    }
    #[doc = "Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0(self) -> &'a mut W {
        self.variant(CLK_EN_A::SSI0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SSICLKGS.CLK_EN and SSICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AM_CLK_EN_R {
        AM_CLK_EN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SSICLKGS.CLK_EN and SSICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&mut self) -> AM_CLK_EN_W {
        AM_CLK_EN_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
}
