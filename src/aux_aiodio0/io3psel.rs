#[doc = "Reader of register IO3PSEL"]
pub type R = crate::R<u32, super::IO3PSEL>;
#[doc = "Writer for register IO3PSEL"]
pub type W = crate::W<u32, super::IO3PSEL>;
#[doc = "Register IO3PSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::IO3PSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "7: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    AUX_TIMER2_PULSE = 7,
    #[doc = "6: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    AUX_TIMER2_EV3 = 6,
    #[doc = "5: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    AUX_TIMER2_EV2 = 5,
    #[doc = "4: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    AUX_TIMER2_EV1 = 4,
    #[doc = "3: Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    AUX_TIMER2_EV0 = 3,
    #[doc = "2: Peripheral output mux selects AUX_SPIM MOSI."]
    AUX_SPIM_MOSI = 2,
    #[doc = "1: Peripheral output mux selects AUX_SPIM SCLK."]
    AUX_SPIM_SCLK = 1,
    #[doc = "0: Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    AUX_EV_OBS = 0,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            7 => SRC_A::AUX_TIMER2_PULSE,
            6 => SRC_A::AUX_TIMER2_EV3,
            5 => SRC_A::AUX_TIMER2_EV2,
            4 => SRC_A::AUX_TIMER2_EV1,
            3 => SRC_A::AUX_TIMER2_EV0,
            2 => SRC_A::AUX_SPIM_MOSI,
            1 => SRC_A::AUX_SPIM_SCLK,
            0 => SRC_A::AUX_EV_OBS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_PULSE`"]
    #[inline(always)]
    pub fn is_aux_timer2_pulse(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_PULSE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV3`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev3(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV3
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV2`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev2(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV2
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV1`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev1(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV1
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER2_EV0`"]
    #[inline(always)]
    pub fn is_aux_timer2_ev0(&self) -> bool {
        *self == SRC_A::AUX_TIMER2_EV0
    }
    #[doc = "Checks if the value of the field is `AUX_SPIM_MOSI`"]
    #[inline(always)]
    pub fn is_aux_spim_mosi(&self) -> bool {
        *self == SRC_A::AUX_SPIM_MOSI
    }
    #[doc = "Checks if the value of the field is `AUX_SPIM_SCLK`"]
    #[inline(always)]
    pub fn is_aux_spim_sclk(&self) -> bool {
        *self == SRC_A::AUX_SPIM_SCLK
    }
    #[doc = "Checks if the value of the field is `AUX_EV_OBS`"]
    #[inline(always)]
    pub fn is_aux_ev_obs(&self) -> bool {
        *self == SRC_A::AUX_EV_OBS
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn aux_timer2_pulse(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_PULSE)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn aux_timer2_ev3(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV3)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn aux_timer2_ev2(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV2)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn aux_timer2_ev1(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV1)
    }
    #[doc = "Peripheral output mux selects asynchronous version of AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(self) -> &'a mut W {
        self.variant(SRC_A::AUX_TIMER2_EV0)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM MOSI."]
    #[inline(always)]
    pub fn aux_spim_mosi(self) -> &'a mut W {
        self.variant(SRC_A::AUX_SPIM_MOSI)
    }
    #[doc = "Peripheral output mux selects AUX_SPIM SCLK."]
    #[inline(always)]
    pub fn aux_spim_sclk(self) -> &'a mut W {
        self.variant(SRC_A::AUX_SPIM_SCLK)
    }
    #[doc = "Peripheral output mux selects event selected by AUX_EVCTL:EVOBSCFG"]
    #[inline(always)]
    pub fn aux_ev_obs(self) -> &'a mut W {
        self.variant(SRC_A::AUX_EV_OBS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Select a peripheral signal that connects to AUXIO\\[8i+3\\]
when IOPOE bit 3 is set."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
