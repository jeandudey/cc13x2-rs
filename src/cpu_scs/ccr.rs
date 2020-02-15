#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
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
#[doc = "Reader of field `STKALIGN`"]
pub type STKALIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STKALIGN`"]
pub struct STKALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> STKALIGN_W<'a> {
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
#[doc = "Reader of field `BFHFNMIGN`"]
pub type BFHFNMIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFHFNMIGN`"]
pub struct BFHFNMIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFHFNMIGN_W<'a> {
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
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `DIV_0_TRP`"]
pub type DIV_0_TRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV_0_TRP`"]
pub struct DIV_0_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_0_TRP_W<'a> {
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
#[doc = "Reader of field `UNALIGN_TRP`"]
pub type UNALIGN_TRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNALIGN_TRP`"]
pub struct UNALIGN_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGN_TRP_W<'a> {
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
#[doc = "Reader of field `USERSETMPEND`"]
pub type USERSETMPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USERSETMPEND`"]
pub struct USERSETMPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USERSETMPEND_W<'a> {
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
#[doc = "Reader of field `NONBASETHREDENA`"]
pub type NONBASETHREDENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NONBASETHREDENA`"]
pub struct NONBASETHREDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NONBASETHREDENA_W<'a> {
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
Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline(always)]
    pub fn nonbasethredena(&self) -> NONBASETHREDENA_R {
        NONBASETHREDENA_R::new((self.bits & 0x01) != 0)
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
Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline(always)]
    pub fn stkalign(&mut self) -> STKALIGN_W {
        STKALIGN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline(always)]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W {
        BFHFNMIGN_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline(always)]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W {
        DIV_0_TRP_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline(always)]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W {
        UNALIGN_TRP_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline(always)]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W {
        USERSETMPEND_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline(always)]
    pub fn nonbasethredena(&mut self) -> NONBASETHREDENA_W {
        NONBASETHREDENA_W { w: self }
    }
}
