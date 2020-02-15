#[doc = "Reader of register DHCSR"]
pub type R = crate::R<u32, super::DHCSR>;
#[doc = "Writer for register DHCSR"]
pub type W = crate::W<u32, super::DHCSR>;
#[doc = "Register DHCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED26`"]
pub type RESERVED26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `S_RESET_ST`"]
pub type S_RESET_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_RESET_ST`"]
pub struct S_RESET_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RESET_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `S_RETIRE_ST`"]
pub type S_RETIRE_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_RETIRE_ST`"]
pub struct S_RETIRE_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RETIRE_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `S_LOCKUP`"]
pub type S_LOCKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_LOCKUP`"]
pub struct S_LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> S_LOCKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `S_SLEEP`"]
pub type S_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_SLEEP`"]
pub struct S_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> S_SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `S_HALT`"]
pub type S_HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_HALT`"]
pub struct S_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> S_HALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `S_REGRDY`"]
pub type S_REGRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_REGRDY`"]
pub struct S_REGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> S_REGRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
#[doc = "Reader of field `C_SNAPSTALL`"]
pub type C_SNAPSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_SNAPSTALL`"]
pub struct C_SNAPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> C_SNAPSTALL_W<'a> {
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
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
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
#[doc = "Reader of field `C_MASKINTS`"]
pub type C_MASKINTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_MASKINTS`"]
pub struct C_MASKINTS_W<'a> {
    w: &'a mut W,
}
impl<'a> C_MASKINTS_W<'a> {
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
#[doc = "Reader of field `C_STEP`"]
pub type C_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_STEP`"]
pub struct C_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> C_STEP_W<'a> {
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
#[doc = "Reader of field `C_HALT`"]
pub type C_HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_HALT`"]
pub struct C_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> C_HALT_W<'a> {
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
#[doc = "Reader of field `C_DEBUGEN`"]
pub type C_DEBUGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_DEBUGEN`"]
pub struct C_DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DEBUGEN_W<'a> {
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
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 5 - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_reset_st(&mut self) -> S_RESET_ST_W {
        S_RESET_ST_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_retire_st(&mut self) -> S_RETIRE_ST_W {
        S_RETIRE_ST_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_lockup(&mut self) -> S_LOCKUP_W {
        S_LOCKUP_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_sleep(&mut self) -> S_SLEEP_W {
        S_SLEEP_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_halt(&mut self) -> S_HALT_W {
        S_HALT_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_regrdy(&mut self) -> S_REGRDY_W {
        S_REGRDY_W { w: self }
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W {
        C_SNAPSTALL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W {
        C_MASKINTS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_step(&mut self) -> C_STEP_W {
        C_STEP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    pub fn c_halt(&mut self) -> C_HALT_W {
        C_HALT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W {
        C_DEBUGEN_W { w: self }
    }
}
