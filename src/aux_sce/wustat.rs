#[doc = "Reader of register WUSTAT"]
pub type R = crate::R<u32, super::WUSTAT>;
#[doc = "Writer for register WUSTAT"]
pub type W = crate::W<u32, super::WUSTAT>;
#[doc = "Register WUSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::WUSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 19)) | (((value as u32) & 0x1fff) << 19);
        self.w
    }
}
#[doc = "Reader of field `EXC_VECTOR`"]
pub type EXC_VECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXC_VECTOR`"]
pub struct EXC_VECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXC_VECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
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
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `WU_SIGNAL`"]
pub type WU_SIGNAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WU_SIGNAL`"]
pub struct WU_SIGNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_SIGNAL_W<'a> {
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
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_SIGNALS_A {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    SCEWEV_PROG = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AUX_ADC_FIFO_NOT_EMPTY = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AUX_TIMER1_EV_OR_IDLE = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AUX_TIMER0_EV_OR_IDLE = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AUX_TDC_DONE = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AUX_COMPB = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AUX_COMPA = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AUX_PROG_DLY_IDLE = 1,
}
impl From<EV_SIGNALS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_SIGNALS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV_SIGNALS`"]
pub type EV_SIGNALS_R = crate::R<u8, EV_SIGNALS_A>;
impl EV_SIGNALS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EV_SIGNALS_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(EV_SIGNALS_A::SCEWEV_PROG),
            64 => Val(EV_SIGNALS_A::AUX_ADC_FIFO_NOT_EMPTY),
            32 => Val(EV_SIGNALS_A::AUX_TIMER1_EV_OR_IDLE),
            16 => Val(EV_SIGNALS_A::AUX_TIMER0_EV_OR_IDLE),
            8 => Val(EV_SIGNALS_A::AUX_TDC_DONE),
            4 => Val(EV_SIGNALS_A::AUX_COMPB),
            2 => Val(EV_SIGNALS_A::AUX_COMPA),
            1 => Val(EV_SIGNALS_A::AUX_PROG_DLY_IDLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCEWEV_PROG`"]
    #[inline(always)]
    pub fn is_scewev_prog(&self) -> bool {
        *self == EV_SIGNALS_A::SCEWEV_PROG
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV_OR_IDLE`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev_or_idle(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_TIMER1_EV_OR_IDLE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV_OR_IDLE`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev_or_idle(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_TIMER0_EV_OR_IDLE
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_PROG_DLY_IDLE`"]
    #[inline(always)]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_PROG_DLY_IDLE
    }
}
#[doc = "Write proxy for field `EV_SIGNALS`"]
pub struct EV_SIGNALS_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_SIGNALS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_SIGNALS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scewev_prog(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::SCEWEV_PROG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_timer1_ev_or_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_TIMER1_EV_OR_IDLE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_timer0_ev_or_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_TIMER0_EV_OR_IDLE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_TDC_DONE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_COMPB)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_COMPA)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_prog_dly_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_PROG_DLY_IDLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:31 - 31:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&self) -> EXC_VECTOR_R {
        EXC_VECTOR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&self) -> WU_SIGNAL_R {
        WU_SIGNAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&self) -> EV_SIGNALS_R {
        EV_SIGNALS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 19:31 - 31:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&mut self) -> EXC_VECTOR_W {
        EXC_VECTOR_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&mut self) -> WU_SIGNAL_W {
        WU_SIGNAL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&mut self) -> EV_SIGNALS_W {
        EV_SIGNALS_W { w: self }
    }
}
