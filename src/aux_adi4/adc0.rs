#[doc = "Reader of register ADC0"]
pub type R = crate::R<u8, super::ADC0>;
#[doc = "Writer for register ADC0"]
pub type W = crate::W<u8, super::ADC0>;
#[doc = "Register ADC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPL_MODE`"]
pub type SMPL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPL_MODE`"]
pub struct SMPL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMPL_CYCLE_EXP_A {
    #[doc = "15: 65536x 6 MHz clock periods = 10.9ms"]
    _10P9_MS = 15,
    #[doc = "14: 32768x 6 MHz clock periods = 5.46ms"]
    _5P46_MS = 14,
    #[doc = "13: 16384x 6 MHz clock periods = 2.73ms"]
    _2P73_MS = 13,
    #[doc = "12: 8192x 6 MHz clock periods = 1.37ms"]
    _1P37_MS = 12,
    #[doc = "11: 4096x 6 MHz clock periods = 682us"]
    _682_US = 11,
    #[doc = "10: 2048x 6 MHz clock periods = 341us"]
    _341_US = 10,
    #[doc = "9: 1024x 6 MHz clock periods = 170us"]
    _170_US = 9,
    #[doc = "8: 512x 6 MHz clock periods = 85.3us"]
    _85P3_US = 8,
    #[doc = "7: 256x 6 MHz clock periods = 42.6us"]
    _42P6_US = 7,
    #[doc = "6: 128x 6 MHz clock periods = 21.3us"]
    _21P3_US = 6,
    #[doc = "5: 64x 6 MHz clock periods = 10.6us"]
    _10P6_US = 5,
    #[doc = "4: 32x 6 MHz clock periods = 5.3us"]
    _5P3_US = 4,
    #[doc = "3: 16x 6 MHz clock periods = 2.7us"]
    _2P7_US = 3,
}
impl From<SMPL_CYCLE_EXP_A> for u8 {
    #[inline(always)]
    fn from(variant: SMPL_CYCLE_EXP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMPL_CYCLE_EXP`"]
pub type SMPL_CYCLE_EXP_R = crate::R<u8, SMPL_CYCLE_EXP_A>;
impl SMPL_CYCLE_EXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMPL_CYCLE_EXP_A> {
        use crate::Variant::*;
        match self.bits {
            15 => Val(SMPL_CYCLE_EXP_A::_10P9_MS),
            14 => Val(SMPL_CYCLE_EXP_A::_5P46_MS),
            13 => Val(SMPL_CYCLE_EXP_A::_2P73_MS),
            12 => Val(SMPL_CYCLE_EXP_A::_1P37_MS),
            11 => Val(SMPL_CYCLE_EXP_A::_682_US),
            10 => Val(SMPL_CYCLE_EXP_A::_341_US),
            9 => Val(SMPL_CYCLE_EXP_A::_170_US),
            8 => Val(SMPL_CYCLE_EXP_A::_85P3_US),
            7 => Val(SMPL_CYCLE_EXP_A::_42P6_US),
            6 => Val(SMPL_CYCLE_EXP_A::_21P3_US),
            5 => Val(SMPL_CYCLE_EXP_A::_10P6_US),
            4 => Val(SMPL_CYCLE_EXP_A::_5P3_US),
            3 => Val(SMPL_CYCLE_EXP_A::_2P7_US),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10P9_MS`"]
    #[inline(always)]
    pub fn is_10p9_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_10P9_MS
    }
    #[doc = "Checks if the value of the field is `_5P46_MS`"]
    #[inline(always)]
    pub fn is_5p46_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_5P46_MS
    }
    #[doc = "Checks if the value of the field is `_2P73_MS`"]
    #[inline(always)]
    pub fn is_2p73_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_2P73_MS
    }
    #[doc = "Checks if the value of the field is `_1P37_MS`"]
    #[inline(always)]
    pub fn is_1p37_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_1P37_MS
    }
    #[doc = "Checks if the value of the field is `_682_US`"]
    #[inline(always)]
    pub fn is_682_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_682_US
    }
    #[doc = "Checks if the value of the field is `_341_US`"]
    #[inline(always)]
    pub fn is_341_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_341_US
    }
    #[doc = "Checks if the value of the field is `_170_US`"]
    #[inline(always)]
    pub fn is_170_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_170_US
    }
    #[doc = "Checks if the value of the field is `_85P3_US`"]
    #[inline(always)]
    pub fn is_85p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_85P3_US
    }
    #[doc = "Checks if the value of the field is `_42P6_US`"]
    #[inline(always)]
    pub fn is_42p6_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_42P6_US
    }
    #[doc = "Checks if the value of the field is `_21P3_US`"]
    #[inline(always)]
    pub fn is_21p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_21P3_US
    }
    #[doc = "Checks if the value of the field is `_10P6_US`"]
    #[inline(always)]
    pub fn is_10p6_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_10P6_US
    }
    #[doc = "Checks if the value of the field is `_5P3_US`"]
    #[inline(always)]
    pub fn is_5p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_5P3_US
    }
    #[doc = "Checks if the value of the field is `_2P7_US`"]
    #[inline(always)]
    pub fn is_2p7_us(&self) -> bool {
        *self == SMPL_CYCLE_EXP_A::_2P7_US
    }
}
#[doc = "Write proxy for field `SMPL_CYCLE_EXP`"]
pub struct SMPL_CYCLE_EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_CYCLE_EXP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPL_CYCLE_EXP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    #[inline(always)]
    pub fn _10p9_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_10P9_MS)
    }
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    #[inline(always)]
    pub fn _5p46_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_5P46_MS)
    }
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    #[inline(always)]
    pub fn _2p73_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_2P73_MS)
    }
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    #[inline(always)]
    pub fn _1p37_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_1P37_MS)
    }
    #[doc = "4096x 6 MHz clock periods = 682us"]
    #[inline(always)]
    pub fn _682_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_682_US)
    }
    #[doc = "2048x 6 MHz clock periods = 341us"]
    #[inline(always)]
    pub fn _341_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_341_US)
    }
    #[doc = "1024x 6 MHz clock periods = 170us"]
    #[inline(always)]
    pub fn _170_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_170_US)
    }
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    #[inline(always)]
    pub fn _85p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_85P3_US)
    }
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    #[inline(always)]
    pub fn _42p6_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_42P6_US)
    }
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    #[inline(always)]
    pub fn _21p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_21P3_US)
    }
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    #[inline(always)]
    pub fn _10p6_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_10P6_US)
    }
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    #[inline(always)]
    pub fn _5p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_5P3_US)
    }
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    #[inline(always)]
    pub fn _2p7_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXP_A::_2P7_US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u8) & 0x0f) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESET_N`"]
pub type RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_N`"]
pub struct RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline(always)]
    pub fn smpl_mode(&self) -> SMPL_MODE_R {
        SMPL_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    pub fn smpl_cycle_exp(&self) -> SMPL_CYCLE_EXP_R {
        SMPL_CYCLE_EXP_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\]
ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline(always)]
    pub fn smpl_mode(&mut self) -> SMPL_MODE_W {
        SMPL_MODE_W { w: self }
    }
    #[doc = "Bits 3:6 - 6:3\\]
Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    pub fn smpl_cycle_exp(&mut self) -> SMPL_CYCLE_EXP_W {
        SMPL_CYCLE_EXP_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline(always)]
    pub fn reset_n(&mut self) -> RESET_N_W {
        RESET_N_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
ADC Enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
