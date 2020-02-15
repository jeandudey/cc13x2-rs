#[doc = "Reader of register FPCCR"]
pub type R = crate::R<u32, super::FPCCR>;
#[doc = "Writer for register FPCCR"]
pub type W = crate::W<u32, super::FPCCR>;
#[doc = "Register FPCCR `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::FPCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `ASPEN`"]
pub type ASPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASPEN`"]
pub struct ASPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `LSPEN`"]
pub type LSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSPEN`"]
pub struct LSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
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
        self.w.bits = (self.w.bits & !(0x001f_ffff << 9)) | (((value as u32) & 0x001f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `MONRDY`"]
pub type MONRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONRDY`"]
pub struct MONRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MONRDY_W<'a> {
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
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BFRDY`"]
pub type BFRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFRDY`"]
pub struct BFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BFRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `MMRDY`"]
pub type MMRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMRDY`"]
pub struct MMRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HFRDY`"]
pub type HFRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFRDY`"]
pub struct HFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRDY_W<'a> {
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
#[doc = "Reader of field `THREAD`"]
pub type THREAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THREAD`"]
pub struct THREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> THREAD_W<'a> {
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
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
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
#[doc = "Reader of field `USER`"]
pub type USER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USER`"]
pub struct USER_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_W<'a> {
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
#[doc = "Reader of field `LSPACT`"]
pub type LSPACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSPACT`"]
pub struct LSPACT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPACT_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 9:29 - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Automatic State Preservation enable. When this bit is set is will cause bit \\[2\\]
of the Special CONTROL register to be set (FPCA) on execution of a floating point instruction which results in the floating point state automatically being preserved on exception entry."]
    #[inline(always)]
    pub fn aspen(&mut self) -> ASPEN_W {
        ASPEN_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Lazy State Preservation enable. Lazy state preservation is when the processor performs a context save, space on the stack is reserved for the floating point state but it is not stacked until the new context performs a floating point operation. 0: Disable automatic lazy state preservation for floating-point context. 1: Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&mut self) -> LSPEN_W {
        LSPEN_W { w: self }
    }
    #[doc = "Bits 9:29 - 29:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates whether the the software executing when the processor allocated the FP stack frame was able to set the DebugMonitor exception to pending. 0: DebugMonitor is disabled or priority did not permit setting DEMCR.MON_PEND when the floating-point stack frame was allocated. 1: DebugMonitor is enabled and priority permits setting DEMCR.MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&mut self) -> MONRDY_W {
        MONRDY_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the BusFault exception to pending. 0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated. 1: BusFault is enabled and priority permitted setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&mut self) -> BFRDY_W {
        BFRDY_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the MemManage exception to pending. 0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated. 1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&mut self) -> MMRDY_W {
        MMRDY_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates whether the software executing when the processor allocated the FP stack frame was able to set the HardFault exception to pending. 0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated. 1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&mut self) -> HFRDY_W {
        HFRDY_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates the processor mode was Thread when it allocated the FP stack frame. 0: Mode was not Thread Mode when the floating-point stack frame was allocated. 1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&mut self) -> THREAD_W {
        THREAD_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the privilege level of the software executing was User (Unpriviledged) when the processor allocated the FP stack frame: 0: Privilege level was not user when the floating-point stack frame was allocated. 1: Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&mut self) -> USER_W {
        USER_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates whether Lazy preservation of the FP state is active: 0: Lazy state preservation is not active. 1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn lspact(&mut self) -> LSPACT_W {
        LSPACT_W { w: self }
    }
}
