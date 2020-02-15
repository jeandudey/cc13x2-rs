#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED19`"]
pub type RESERVED19_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED19`"]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 19)) | (((value as u32) & 0x1fff) << 19);
        self.w
    }
}
#[doc = "18:16\\]
Eventmask selecting which delayed events that form the combined event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMB_EV_MASK_A {
    #[doc = "4: Use Channel 2 delayed event in combined event"]
    CH2 = 4,
    #[doc = "2: Use Channel 1 delayed event in combined event"]
    CH1 = 2,
    #[doc = "1: Use Channel 0 delayed event in combined event"]
    CH0 = 1,
    #[doc = "0: No event is selected for combined event."]
    NONE = 0,
}
impl From<COMB_EV_MASK_A> for u8 {
    #[inline(always)]
    fn from(variant: COMB_EV_MASK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMB_EV_MASK`"]
pub type COMB_EV_MASK_R = crate::R<u8, COMB_EV_MASK_A>;
impl COMB_EV_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMB_EV_MASK_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(COMB_EV_MASK_A::CH2),
            2 => Val(COMB_EV_MASK_A::CH1),
            1 => Val(COMB_EV_MASK_A::CH0),
            0 => Val(COMB_EV_MASK_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == COMB_EV_MASK_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == COMB_EV_MASK_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == COMB_EV_MASK_A::CH0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == COMB_EV_MASK_A::NONE
    }
}
#[doc = "Write proxy for field `COMB_EV_MASK`"]
pub struct COMB_EV_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_EV_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMB_EV_MASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use Channel 2 delayed event in combined event"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::CH2)
    }
    #[doc = "Use Channel 1 delayed event in combined event"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::CH1)
    }
    #[doc = "Use Channel 0 delayed event in combined event"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::CH0)
    }
    #[doc = "No event is selected for combined event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_DELAY_A {
    #[doc = "13: Delay by 144 clock cycles"]
    D144 = 13,
    #[doc = "12: Delay by 128 clock cycles"]
    D128 = 12,
    #[doc = "11: Delay by 112 clock cycles"]
    D112 = 11,
    #[doc = "10: Delay by 96 clock cycles"]
    D96 = 10,
    #[doc = "9: Delay by 80 clock cycles"]
    D80 = 9,
    #[doc = "8: Delay by 64 clock cycles"]
    D64 = 8,
    #[doc = "7: Delay by 48 clock cycles"]
    D48 = 7,
    #[doc = "6: Delay by 32 clock cycles"]
    D32 = 6,
    #[doc = "5: Delay by 16 clock cycles"]
    D16 = 5,
    #[doc = "4: Delay by 8 clock cycles"]
    D8 = 4,
    #[doc = "3: Delay by 4 clock cycles"]
    D4 = 3,
    #[doc = "2: Delay by 2 clock cycles"]
    D2 = 2,
    #[doc = "1: Delay by 1 clock cycles"]
    D1 = 1,
    #[doc = "0: No delay on delayed event"]
    D0 = 0,
}
impl From<EV_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EV_DELAY`"]
pub type EV_DELAY_R = crate::R<u8, EV_DELAY_A>;
impl EV_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EV_DELAY_A> {
        use crate::Variant::*;
        match self.bits {
            13 => Val(EV_DELAY_A::D144),
            12 => Val(EV_DELAY_A::D128),
            11 => Val(EV_DELAY_A::D112),
            10 => Val(EV_DELAY_A::D96),
            9 => Val(EV_DELAY_A::D80),
            8 => Val(EV_DELAY_A::D64),
            7 => Val(EV_DELAY_A::D48),
            6 => Val(EV_DELAY_A::D32),
            5 => Val(EV_DELAY_A::D16),
            4 => Val(EV_DELAY_A::D8),
            3 => Val(EV_DELAY_A::D4),
            2 => Val(EV_DELAY_A::D2),
            1 => Val(EV_DELAY_A::D1),
            0 => Val(EV_DELAY_A::D0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `D144`"]
    #[inline(always)]
    pub fn is_d144(&self) -> bool {
        *self == EV_DELAY_A::D144
    }
    #[doc = "Checks if the value of the field is `D128`"]
    #[inline(always)]
    pub fn is_d128(&self) -> bool {
        *self == EV_DELAY_A::D128
    }
    #[doc = "Checks if the value of the field is `D112`"]
    #[inline(always)]
    pub fn is_d112(&self) -> bool {
        *self == EV_DELAY_A::D112
    }
    #[doc = "Checks if the value of the field is `D96`"]
    #[inline(always)]
    pub fn is_d96(&self) -> bool {
        *self == EV_DELAY_A::D96
    }
    #[doc = "Checks if the value of the field is `D80`"]
    #[inline(always)]
    pub fn is_d80(&self) -> bool {
        *self == EV_DELAY_A::D80
    }
    #[doc = "Checks if the value of the field is `D64`"]
    #[inline(always)]
    pub fn is_d64(&self) -> bool {
        *self == EV_DELAY_A::D64
    }
    #[doc = "Checks if the value of the field is `D48`"]
    #[inline(always)]
    pub fn is_d48(&self) -> bool {
        *self == EV_DELAY_A::D48
    }
    #[doc = "Checks if the value of the field is `D32`"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == EV_DELAY_A::D32
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == EV_DELAY_A::D16
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == EV_DELAY_A::D8
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == EV_DELAY_A::D4
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == EV_DELAY_A::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == EV_DELAY_A::D1
    }
    #[doc = "Checks if the value of the field is `D0`"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == EV_DELAY_A::D0
    }
}
#[doc = "Write proxy for field `EV_DELAY`"]
pub struct EV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_DELAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Delay by 144 clock cycles"]
    #[inline(always)]
    pub fn d144(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D144)
    }
    #[doc = "Delay by 128 clock cycles"]
    #[inline(always)]
    pub fn d128(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D128)
    }
    #[doc = "Delay by 112 clock cycles"]
    #[inline(always)]
    pub fn d112(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D112)
    }
    #[doc = "Delay by 96 clock cycles"]
    #[inline(always)]
    pub fn d96(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D96)
    }
    #[doc = "Delay by 80 clock cycles"]
    #[inline(always)]
    pub fn d80(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D80)
    }
    #[doc = "Delay by 64 clock cycles"]
    #[inline(always)]
    pub fn d64(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D64)
    }
    #[doc = "Delay by 48 clock cycles"]
    #[inline(always)]
    pub fn d48(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D48)
    }
    #[doc = "Delay by 32 clock cycles"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D32)
    }
    #[doc = "Delay by 16 clock cycles"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D16)
    }
    #[doc = "Delay by 8 clock cycles"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D8)
    }
    #[doc = "Delay by 4 clock cycles"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D4)
    }
    #[doc = "Delay by 2 clock cycles"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D2)
    }
    #[doc = "Delay by 1 clock cycles"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D1)
    }
    #[doc = "No delay on delayed event"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTC_4KHZ_EN`"]
pub type RTC_4KHZ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_4KHZ_EN`"]
pub struct RTC_4KHZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_4KHZ_EN_W<'a> {
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
#[doc = "Reader of field `RTC_UPD_EN`"]
pub type RTC_UPD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_UPD_EN`"]
pub struct RTC_UPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_UPD_EN_W<'a> {
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    pub fn comb_ev_mask(&self) -> COMB_EV_MASK_R {
        COMB_EV_MASK_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline(always)]
    pub fn ev_delay(&self) -> EV_DELAY_R {
        EV_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline(always)]
    pub fn rtc_4khz_en(&self) -> RTC_4KHZ_EN_R {
        RTC_4KHZ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline(always)]
    pub fn rtc_upd_en(&self) -> RTC_UPD_EN_R {
        RTC_UPD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    pub fn comb_ev_mask(&mut self) -> COMB_EV_MASK_W {
        COMB_EV_MASK_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline(always)]
    pub fn ev_delay(&mut self) -> EV_DELAY_W {
        EV_DELAY_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bits 3:6 - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline(always)]
    pub fn rtc_4khz_en(&mut self) -> RTC_4KHZ_EN_W {
        RTC_4KHZ_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline(always)]
    pub fn rtc_upd_en(&mut self) -> RTC_UPD_EN_W {
        RTC_UPD_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
