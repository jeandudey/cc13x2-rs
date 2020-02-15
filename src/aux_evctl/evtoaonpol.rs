#[doc = "Reader of register EVTOAONPOL"]
pub type R = crate::R<u32, super::EVTOAONPOL>;
#[doc = "Writer for register EVTOAONPOL"]
pub type W = crate::W<u32, super::EVTOAONPOL>;
#[doc = "Register EVTOAONPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTOAONPOL {
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
#[doc = "8:8\\]
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER1_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER1_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER1_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER1_EV`"]
pub type AUX_TIMER1_EV_R = crate::R<bool, AUX_TIMER1_EV_A>;
impl AUX_TIMER1_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER1_EV_A {
        match self.bits {
            true => AUX_TIMER1_EV_A::LOW,
            false => AUX_TIMER1_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER1_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER1_EV_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER1_EV`"]
pub struct AUX_TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER1_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER1_EV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EV_A::HIGH)
    }
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
#[doc = "7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER0_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER0_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER0_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER0_EV`"]
pub type AUX_TIMER0_EV_R = crate::R<bool, AUX_TIMER0_EV_A>;
impl AUX_TIMER0_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER0_EV_A {
        match self.bits {
            true => AUX_TIMER0_EV_A::LOW,
            false => AUX_TIMER0_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER0_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER0_EV_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER0_EV`"]
pub struct AUX_TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER0_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER0_EV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EV_A::HIGH)
    }
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
#[doc = "6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TDC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TDC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TDC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TDC_DONE`"]
pub type AUX_TDC_DONE_R = crate::R<bool, AUX_TDC_DONE_A>;
impl AUX_TDC_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TDC_DONE_A {
        match self.bits {
            true => AUX_TDC_DONE_A::LOW,
            false => AUX_TDC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TDC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TDC_DONE_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TDC_DONE`"]
pub struct AUX_TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TDC_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TDC_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TDC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TDC_DONE_A::HIGH)
    }
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
#[doc = "5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_ADC_DONE`"]
pub type AUX_ADC_DONE_R = crate::R<bool, AUX_ADC_DONE_A>;
impl AUX_ADC_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_DONE_A {
        match self.bits {
            true => AUX_ADC_DONE_A::LOW,
            false => AUX_ADC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_DONE_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_ADC_DONE`"]
pub struct AUX_ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_ADC_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_DONE_A::HIGH)
    }
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
#[doc = "4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPB_A {
    #[doc = "1: Falling edge"]
    FALL = 1,
    #[doc = "0: Rising edge"]
    RISE = 0,
}
impl From<AUX_COMPB_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, AUX_COMPB_A>;
impl AUX_COMPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPB_A {
        match self.bits {
            true => AUX_COMPB_A::FALL,
            false => AUX_COMPB_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPB_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPB_A::RISE
    }
}
#[doc = "Write proxy for field `AUX_COMPB`"]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::RISE)
    }
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
#[doc = "3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPA_A {
    #[doc = "1: Falling edge"]
    FALL = 1,
    #[doc = "0: Rising edge"]
    RISE = 0,
}
impl From<AUX_COMPA_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, AUX_COMPA_A>;
impl AUX_COMPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPA_A {
        match self.bits {
            true => AUX_COMPA_A::FALL,
            false => AUX_COMPA_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPA_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPA_A::RISE
    }
}
#[doc = "Write proxy for field `AUX_COMPA`"]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::RISE)
    }
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
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits & 0x07) as u8)
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
Select the level of EVSTAT3.AUX_TIMER1_EV that sets EVTOAONFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W {
        AUX_TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT3.AUX_TIMER0_EV that sets EVTOAONFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W {
        AUX_TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT3.AUX_TDC_DONE that sets EVTOAONFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W {
        AUX_TDC_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT3.AUX_ADC_DONE that sets EVTOAONFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W {
        AUX_ADC_DONE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT2.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT2.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
}
