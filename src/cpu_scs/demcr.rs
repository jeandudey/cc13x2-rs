#[doc = "Reader of register DEMCR"]
pub type R = crate::R<u32, super::DEMCR>;
#[doc = "Writer for register DEMCR"]
pub type W = crate::W<u32, super::DEMCR>;
#[doc = "Register DEMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DEMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `TRCENA`"]
pub type TRCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCENA`"]
pub struct TRCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCENA_W<'a> {
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
#[doc = "Reader of field `MON_REQ`"]
pub type MON_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_REQ`"]
pub struct MON_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_REQ_W<'a> {
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
#[doc = "Reader of field `MON_STEP`"]
pub type MON_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_STEP`"]
pub struct MON_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_STEP_W<'a> {
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
#[doc = "Reader of field `MON_PEND`"]
pub type MON_PEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_PEND`"]
pub struct MON_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_PEND_W<'a> {
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
#[doc = "Reader of field `MON_EN`"]
pub type MON_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MON_EN`"]
pub struct MON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `VC_HARDERR`"]
pub type VC_HARDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_HARDERR`"]
pub struct VC_HARDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_HARDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `VC_INTERR`"]
pub type VC_INTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_INTERR`"]
pub struct VC_INTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_INTERR_W<'a> {
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
#[doc = "Reader of field `VC_BUSERR`"]
pub type VC_BUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_BUSERR`"]
pub struct VC_BUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_BUSERR_W<'a> {
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
#[doc = "Reader of field `VC_STATERR`"]
pub type VC_STATERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_STATERR`"]
pub struct VC_STATERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_STATERR_W<'a> {
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
#[doc = "Reader of field `VC_CHKERR`"]
pub type VC_CHKERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_CHKERR`"]
pub struct VC_CHKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_CHKERR_W<'a> {
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
#[doc = "Reader of field `VC_NOCPERR`"]
pub type VC_NOCPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_NOCPERR`"]
pub struct VC_NOCPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_NOCPERR_W<'a> {
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
#[doc = "Reader of field `VC_MMERR`"]
pub type VC_MMERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_MMERR`"]
pub struct VC_MMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_MMERR_W<'a> {
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
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `VC_CORERESET`"]
pub type VC_CORERESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC_CORERESET`"]
pub struct VC_CORERESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_CORERESET_W<'a> {
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
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
    #[inline(always)]
    pub fn trcena(&mut self) -> TRCENA_W {
        TRCENA_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND"]
    #[inline(always)]
    pub fn mon_req(&mut self) -> MON_REQ_W {
        MON_REQ_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    pub fn mon_step(&mut self) -> MON_STEP_W {
        MON_STEP_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    pub fn mon_pend(&mut self) -> MON_PEND_W {
        MON_PEND_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    pub fn mon_en(&mut self) -> MON_EN_W {
        MON_EN_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W {
        VC_HARDERR_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_interr(&mut self) -> VC_INTERR_W {
        VC_INTERR_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W {
        VC_BUSERR_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W {
        VC_STATERR_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W {
        VC_CHKERR_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W {
        VC_NOCPERR_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W {
        VC_MMERR_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W {
        VC_CORERESET_W { w: self }
    }
}
