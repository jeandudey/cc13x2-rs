#[doc = "Reader of register STMPCTL"]
pub type R = crate::R<u32, super::STMPCTL>;
#[doc = "Writer for register STMPCTL"]
pub type W = crate::W<u32, super::STMPCTL>;
#[doc = "Register STMPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPCTL {
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
#[doc = "Reader of field `OUT_RDY`"]
pub type OUT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_RDY`"]
pub struct OUT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RDY_W<'a> {
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
#[doc = "Reader of field `IN_RDY`"]
pub type IN_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_RDY`"]
pub struct IN_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RDY_W<'a> {
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
#[doc = "Reader of field `STMP_EN`"]
pub type STMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STMP_EN`"]
pub struct STMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STMP_EN_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn out_rdy(&self) -> OUT_RDY_R {
        OUT_RDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn in_rdy(&self) -> IN_RDY_R {
        IN_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    pub fn stmp_en(&self) -> STMP_EN_R {
        STMP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn out_rdy(&mut self) -> OUT_RDY_W {
        OUT_RDY_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn in_rdy(&mut self) -> IN_RDY_W {
        IN_RDY_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    pub fn stmp_en(&mut self) -> STMP_EN_W {
        STMP_EN_W { w: self }
    }
}
