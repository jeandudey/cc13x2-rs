#[doc = "Reader of register EVTOAONFLAGS"]
pub type R = crate::R<u32, super::EVTOAONFLAGS>;
#[doc = "Writer for register EVTOAONFLAGS"]
pub type W = crate::W<u32, super::EVTOAONFLAGS>;
#[doc = "Register EVTOAONFLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTOAONFLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `AUX_TIMER1_EV`"]
pub type AUX_TIMER1_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER1_EV`"]
pub struct AUX_TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER1_EV_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER0_EV`"]
pub type AUX_TIMER0_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER0_EV`"]
pub struct AUX_TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER0_EV_W<'a> {
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
#[doc = "Reader of field `AUX_TDC_DONE`"]
pub type AUX_TDC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TDC_DONE`"]
pub struct AUX_TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TDC_DONE_W<'a> {
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
#[doc = "Reader of field `AUX_ADC_DONE`"]
pub type AUX_ADC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ADC_DONE`"]
pub struct AUX_ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_DONE_W<'a> {
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
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPB`"]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
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
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPA`"]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
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
#[doc = "Reader of field `SWEV2`"]
pub type SWEV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV2`"]
pub struct SWEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV2_W<'a> {
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
#[doc = "Reader of field `SWEV1`"]
pub type SWEV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV1`"]
pub struct SWEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV1_W<'a> {
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
#[doc = "Reader of field `SWEV0`"]
pub type SWEV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV0`"]
pub struct SWEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV0_W<'a> {
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
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
This event flag is set when level selected by EVTOAONPOL.AUX_TIMER1_EV occurs on EVSTAT3.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This event flag is set when level selected by EVTOAONPOL.AUX_TIMER0_EV occurs on EVSTAT3.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This event flag is set when level selected by EVTOAONPOL.AUX_TDC_DONE occurs on EVSTAT3.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This event flag is set when level selected by EVTOAONPOL.AUX_ADC_DONE occurs on EVSTAT3.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT2.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT2.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
This event flag is set when level selected by EVTOAONPOL.AUX_TIMER1_EV occurs on EVSTAT3.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W {
        AUX_TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
This event flag is set when level selected by EVTOAONPOL.AUX_TIMER0_EV occurs on EVSTAT3.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W {
        AUX_TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
This event flag is set when level selected by EVTOAONPOL.AUX_TDC_DONE occurs on EVSTAT3.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W {
        AUX_TDC_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
This event flag is set when level selected by EVTOAONPOL.AUX_ADC_DONE occurs on EVSTAT3.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W {
        AUX_ADC_DONE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT2.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT2.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
    #[inline(always)]
    pub fn swev2(&mut self) -> SWEV2_W {
        SWEV2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
    #[inline(always)]
    pub fn swev1(&mut self) -> SWEV1_W {
        SWEV1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
    #[inline(always)]
    pub fn swev0(&mut self) -> SWEV0_W {
        SWEV0_W { w: self }
    }
}
