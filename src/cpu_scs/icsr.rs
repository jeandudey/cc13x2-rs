#[doc = "Reader of register ICSR"]
pub type R = crate::R<u32, super::ICSR>;
#[doc = "Writer for register ICSR"]
pub type W = crate::W<u32, super::ICSR>;
#[doc = "Register ICSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NMIPENDSET`"]
pub type NMIPENDSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIPENDSET`"]
pub struct NMIPENDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIPENDSET_W<'a> {
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
#[doc = "Reader of field `RESERVED29`"]
pub type RESERVED29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED29`"]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `PENDSVSET`"]
pub type PENDSVSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSVSET`"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PENDSVCLR`"]
pub type PENDSVCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSVCLR`"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PENDSTSET`"]
pub type PENDSTSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSTSET`"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PENDSTCLR`"]
pub type PENDSTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSTCLR`"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
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
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
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
#[doc = "Reader of field `ISRPREEMPT`"]
pub type ISRPREEMPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRPREEMPT`"]
pub struct ISRPREEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRPREEMPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ISRPENDING`"]
pub type ISRPENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISRPENDING`"]
pub struct ISRPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> ISRPENDING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RESERVED18`"]
pub type RESERVED18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED18`"]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `VECTPENDING`"]
pub type VECTPENDING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VECTPENDING`"]
pub struct VECTPENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTPENDING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RETTOBASE`"]
pub type RETTOBASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETTOBASE`"]
pub struct RETTOBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RETTOBASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `VECTACTIVE`"]
pub type VECTACTIVE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTACTIVE`"]
pub struct VECTACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 0:8 - 8:0\\]
Active ISR number field. Reset clears this field."]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W {
        NMIPENDSET_W { w: self }
    }
    #[doc = "Bits 29:30 - 30:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
    #[inline(always)]
    pub fn isrpreempt(&mut self) -> ISRPREEMPT_W {
        ISRPREEMPT_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
    #[inline(always)]
    pub fn isrpending(&mut self) -> ISRPENDING_W {
        ISRPENDING_W { w: self }
    }
    #[doc = "Bits 18:21 - 21:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bits 12:17 - 17:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    pub fn vectpending(&mut self) -> VECTPENDING_W {
        VECTPENDING_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    pub fn rettobase(&mut self) -> RETTOBASE_W {
        RETTOBASE_W { w: self }
    }
    #[doc = "Bits 9:10 - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\]
Active ISR number field. Reset clears this field."]
    #[inline(always)]
    pub fn vectactive(&mut self) -> VECTACTIVE_W {
        VECTACTIVE_W { w: self }
    }
}
