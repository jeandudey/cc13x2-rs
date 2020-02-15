#[doc = "Reader of register ACTLR"]
pub type R = crate::R<u32, super::ACTLR>;
#[doc = "Writer for register ACTLR"]
pub type W = crate::W<u32, super::ACTLR>;
#[doc = "Register ACTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACTLR {
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
#[doc = "Reader of field `DISOOFP`"]
pub type DISOOFP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISOOFP`"]
pub struct DISOOFP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISOOFP_W<'a> {
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
#[doc = "Reader of field `DISFPCA`"]
pub type DISFPCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISFPCA`"]
pub struct DISFPCA_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFPCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `DISFOLD`"]
pub type DISFOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISFOLD`"]
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
#[doc = "Reader of field `DISDEFWBUF`"]
pub type DISDEFWBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISDEFWBUF`"]
pub struct DISDEFWBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDEFWBUF_W<'a> {
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
#[doc = "Reader of field `DISMCYCINT`"]
pub type DISMCYCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISMCYCINT`"]
pub struct DISMCYCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMCYCINT_W<'a> {
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
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 9 - 9:9\\]
Disables floating point instructions completing out of order with respect to integer instructions."]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Disables folding of IT instruction."]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Disables floating point instructions completing out of order with respect to integer instructions."]
    #[inline(always)]
    pub fn disoofp(&mut self) -> DISOOFP_W {
        DISOOFP_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&mut self) -> DISFPCA_W {
        DISFPCA_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Disables folding of IT instruction."]
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W {
        DISDEFWBUF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W {
        DISMCYCINT_W { w: self }
    }
}
