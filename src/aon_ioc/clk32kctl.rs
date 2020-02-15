#[doc = "Reader of register CLK32KCTL"]
pub type R = crate::R<u32, super::CLK32KCTL>;
#[doc = "Writer for register CLK32KCTL"]
pub type W = crate::W<u32, super::CLK32KCTL>;
#[doc = "Register CLK32KCTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CLK32KCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `OE_N`"]
pub type OE_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OE_N`"]
pub struct OE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_N_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (for example IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
    #[inline(always)]
    pub fn oe_n(&self) -> OE_N_R {
        OE_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (for example IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active"]
    #[inline(always)]
    pub fn oe_n(&mut self) -> OE_N_W {
        OE_N_W { w: self }
    }
}
