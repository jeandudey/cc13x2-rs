#[doc = "Reader of register DFSR"]
pub type R = crate::R<u32, super::DFSR>;
#[doc = "Writer for register DFSR"]
pub type W = crate::W<u32, super::DFSR>;
#[doc = "Register DFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `EXTERNAL`"]
pub type EXTERNAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTERNAL`"]
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `VCATCH`"]
pub type VCATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCATCH`"]
pub struct VCATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VCATCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DWTTRAP`"]
pub type DWTTRAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DWTTRAP`"]
pub struct DWTTRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTTRAP_W<'a> {
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
#[doc = "Reader of field `BKPT`"]
pub type BKPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPT`"]
pub struct BKPT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPT_W<'a> {
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
#[doc = "Reader of field `HALTED`"]
pub type HALTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALTED`"]
pub struct HALTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTED_W<'a> {
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
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 4 - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted"]
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred"]
    #[inline(always)]
    pub fn vcatch(&mut self) -> VCATCH_W {
        VCATCH_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match"]
    #[inline(always)]
    pub fn dwttrap(&mut self) -> DWTTRAP_W {
        DWTTRAP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution"]
    #[inline(always)]
    pub fn bkpt(&mut self) -> BKPT_W {
        BKPT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step"]
    #[inline(always)]
    pub fn halted(&mut self) -> HALTED_W {
        HALTED_W { w: self }
    }
}
