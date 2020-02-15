#[doc = "Reader of register STAT1"]
pub type R = crate::R<u32, super::STAT1>;
#[doc = "Writer for register STAT1"]
pub type W = crate::W<u32, super::STAT1>;
#[doc = "Register STAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "31:28\\]
AMPCOMP FSM State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMPSTATE_A {
    #[doc = "14: FAST_START_SETTLE"]
    FAST_START_SETTLE = 14,
    #[doc = "13: FAST_START"]
    FAST_START = 13,
    #[doc = "12: DUMMY_TO_INIT_1"]
    DUMMY_TO_INIT_1 = 12,
    #[doc = "11: IDAC_DECREMENT_WITH_MEASURE"]
    IDAC_DEC_W_MEASURE = 11,
    #[doc = "10: IBIAS_INCREMENT"]
    IBIAS_INC = 10,
    #[doc = "9: LPM_UPDATE"]
    LPM_UPDATE = 9,
    #[doc = "8: IBIAS_DECREMENT_WITH_MEASURE"]
    IBIAS_DEC_W_MEASURE = 8,
    #[doc = "7: IBIAS_CAP_UPDATE"]
    IBIAS_CAP_UPDATE = 7,
    #[doc = "6: IDAC_INCREMENT"]
    IDAC_INCREMENT = 6,
    #[doc = "5: HPM_UPDATE"]
    HPM_UPDATE = 5,
    #[doc = "4: HPM_RAMP3"]
    HPM_RAMP3 = 4,
    #[doc = "3: HPM_RAMP2"]
    HPM_RAMP2 = 3,
    #[doc = "2: HPM_RAMP1"]
    HPM_RAMP1 = 2,
    #[doc = "1: INITIALIZATION"]
    INITIALIZATION = 1,
    #[doc = "0: RESET"]
    RESET = 0,
}
impl From<RAMPSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMPSTATE`"]
pub type RAMPSTATE_R = crate::R<u8, RAMPSTATE_A>;
impl RAMPSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMPSTATE_A> {
        use crate::Variant::*;
        match self.bits {
            14 => Val(RAMPSTATE_A::FAST_START_SETTLE),
            13 => Val(RAMPSTATE_A::FAST_START),
            12 => Val(RAMPSTATE_A::DUMMY_TO_INIT_1),
            11 => Val(RAMPSTATE_A::IDAC_DEC_W_MEASURE),
            10 => Val(RAMPSTATE_A::IBIAS_INC),
            9 => Val(RAMPSTATE_A::LPM_UPDATE),
            8 => Val(RAMPSTATE_A::IBIAS_DEC_W_MEASURE),
            7 => Val(RAMPSTATE_A::IBIAS_CAP_UPDATE),
            6 => Val(RAMPSTATE_A::IDAC_INCREMENT),
            5 => Val(RAMPSTATE_A::HPM_UPDATE),
            4 => Val(RAMPSTATE_A::HPM_RAMP3),
            3 => Val(RAMPSTATE_A::HPM_RAMP2),
            2 => Val(RAMPSTATE_A::HPM_RAMP1),
            1 => Val(RAMPSTATE_A::INITIALIZATION),
            0 => Val(RAMPSTATE_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FAST_START_SETTLE`"]
    #[inline(always)]
    pub fn is_fast_start_settle(&self) -> bool {
        *self == RAMPSTATE_A::FAST_START_SETTLE
    }
    #[doc = "Checks if the value of the field is `FAST_START`"]
    #[inline(always)]
    pub fn is_fast_start(&self) -> bool {
        *self == RAMPSTATE_A::FAST_START
    }
    #[doc = "Checks if the value of the field is `DUMMY_TO_INIT_1`"]
    #[inline(always)]
    pub fn is_dummy_to_init_1(&self) -> bool {
        *self == RAMPSTATE_A::DUMMY_TO_INIT_1
    }
    #[doc = "Checks if the value of the field is `IDAC_DEC_W_MEASURE`"]
    #[inline(always)]
    pub fn is_idac_dec_w_measure(&self) -> bool {
        *self == RAMPSTATE_A::IDAC_DEC_W_MEASURE
    }
    #[doc = "Checks if the value of the field is `IBIAS_INC`"]
    #[inline(always)]
    pub fn is_ibias_inc(&self) -> bool {
        *self == RAMPSTATE_A::IBIAS_INC
    }
    #[doc = "Checks if the value of the field is `LPM_UPDATE`"]
    #[inline(always)]
    pub fn is_lpm_update(&self) -> bool {
        *self == RAMPSTATE_A::LPM_UPDATE
    }
    #[doc = "Checks if the value of the field is `IBIAS_DEC_W_MEASURE`"]
    #[inline(always)]
    pub fn is_ibias_dec_w_measure(&self) -> bool {
        *self == RAMPSTATE_A::IBIAS_DEC_W_MEASURE
    }
    #[doc = "Checks if the value of the field is `IBIAS_CAP_UPDATE`"]
    #[inline(always)]
    pub fn is_ibias_cap_update(&self) -> bool {
        *self == RAMPSTATE_A::IBIAS_CAP_UPDATE
    }
    #[doc = "Checks if the value of the field is `IDAC_INCREMENT`"]
    #[inline(always)]
    pub fn is_idac_increment(&self) -> bool {
        *self == RAMPSTATE_A::IDAC_INCREMENT
    }
    #[doc = "Checks if the value of the field is `HPM_UPDATE`"]
    #[inline(always)]
    pub fn is_hpm_update(&self) -> bool {
        *self == RAMPSTATE_A::HPM_UPDATE
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP3`"]
    #[inline(always)]
    pub fn is_hpm_ramp3(&self) -> bool {
        *self == RAMPSTATE_A::HPM_RAMP3
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP2`"]
    #[inline(always)]
    pub fn is_hpm_ramp2(&self) -> bool {
        *self == RAMPSTATE_A::HPM_RAMP2
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP1`"]
    #[inline(always)]
    pub fn is_hpm_ramp1(&self) -> bool {
        *self == RAMPSTATE_A::HPM_RAMP1
    }
    #[doc = "Checks if the value of the field is `INITIALIZATION`"]
    #[inline(always)]
    pub fn is_initialization(&self) -> bool {
        *self == RAMPSTATE_A::INITIALIZATION
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RAMPSTATE_A::RESET
    }
}
#[doc = "Write proxy for field `RAMPSTATE`"]
pub struct RAMPSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMPSTATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FAST_START_SETTLE"]
    #[inline(always)]
    pub fn fast_start_settle(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::FAST_START_SETTLE)
    }
    #[doc = "FAST_START"]
    #[inline(always)]
    pub fn fast_start(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::FAST_START)
    }
    #[doc = "DUMMY_TO_INIT_1"]
    #[inline(always)]
    pub fn dummy_to_init_1(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::DUMMY_TO_INIT_1)
    }
    #[doc = "IDAC_DECREMENT_WITH_MEASURE"]
    #[inline(always)]
    pub fn idac_dec_w_measure(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::IDAC_DEC_W_MEASURE)
    }
    #[doc = "IBIAS_INCREMENT"]
    #[inline(always)]
    pub fn ibias_inc(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::IBIAS_INC)
    }
    #[doc = "LPM_UPDATE"]
    #[inline(always)]
    pub fn lpm_update(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::LPM_UPDATE)
    }
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE"]
    #[inline(always)]
    pub fn ibias_dec_w_measure(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::IBIAS_DEC_W_MEASURE)
    }
    #[doc = "IBIAS_CAP_UPDATE"]
    #[inline(always)]
    pub fn ibias_cap_update(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::IBIAS_CAP_UPDATE)
    }
    #[doc = "IDAC_INCREMENT"]
    #[inline(always)]
    pub fn idac_increment(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::IDAC_INCREMENT)
    }
    #[doc = "HPM_UPDATE"]
    #[inline(always)]
    pub fn hpm_update(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::HPM_UPDATE)
    }
    #[doc = "HPM_RAMP3"]
    #[inline(always)]
    pub fn hpm_ramp3(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::HPM_RAMP3)
    }
    #[doc = "HPM_RAMP2"]
    #[inline(always)]
    pub fn hpm_ramp2(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::HPM_RAMP2)
    }
    #[doc = "HPM_RAMP1"]
    #[inline(always)]
    pub fn hpm_ramp1(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::HPM_RAMP1)
    }
    #[doc = "INITIALIZATION"]
    #[inline(always)]
    pub fn initialization(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::INITIALIZATION)
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RAMPSTATE_A::RESET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `HPM_UPDATE_AMP`"]
pub type HPM_UPDATE_AMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPM_UPDATE_AMP`"]
pub struct HPM_UPDATE_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_UPDATE_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
        self.w
    }
}
#[doc = "Reader of field `LPM_UPDATE_AMP`"]
pub type LPM_UPDATE_AMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_UPDATE_AMP`"]
pub struct LPM_UPDATE_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_UPDATE_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `FORCE_RCOSC_HF`"]
pub type FORCE_RCOSC_HF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_RCOSC_HF`"]
pub struct FORCE_RCOSC_HF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_RCOSC_HF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SCLK_HF_EN`"]
pub type SCLK_HF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_HF_EN`"]
pub struct SCLK_HF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SCLK_MF_EN`"]
pub type SCLK_MF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_MF_EN`"]
pub struct SCLK_MF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_MF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ACLK_ADC_EN`"]
pub type ACLK_ADC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_ADC_EN`"]
pub struct ACLK_ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_ADC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ACLK_TDC_EN`"]
pub type ACLK_TDC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_TDC_EN`"]
pub struct ACLK_TDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_TDC_EN_W<'a> {
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
#[doc = "Reader of field `ACLK_REF_EN`"]
pub type ACLK_REF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_REF_EN`"]
pub struct ACLK_REF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_EN_W<'a> {
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
#[doc = "Reader of field `CLK_CHP_EN`"]
pub type CLK_CHP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_CHP_EN`"]
pub struct CLK_CHP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CHP_EN_W<'a> {
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
#[doc = "Reader of field `CLK_DCDC_EN`"]
pub type CLK_DCDC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_DCDC_EN`"]
pub struct CLK_DCDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DCDC_EN_W<'a> {
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
#[doc = "Reader of field `SCLK_HF_GOOD`"]
pub type SCLK_HF_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_HF_GOOD`"]
pub struct SCLK_HF_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_GOOD_W<'a> {
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
#[doc = "Reader of field `SCLK_MF_GOOD`"]
pub type SCLK_MF_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_MF_GOOD`"]
pub struct SCLK_MF_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_MF_GOOD_W<'a> {
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
#[doc = "Reader of field `SCLK_LF_GOOD`"]
pub type SCLK_LF_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_LF_GOOD`"]
pub struct SCLK_LF_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_GOOD_W<'a> {
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
#[doc = "Reader of field `ACLK_ADC_GOOD`"]
pub type ACLK_ADC_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_ADC_GOOD`"]
pub struct ACLK_ADC_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_ADC_GOOD_W<'a> {
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
#[doc = "Reader of field `ACLK_TDC_GOOD`"]
pub type ACLK_TDC_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_TDC_GOOD`"]
pub struct ACLK_TDC_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_TDC_GOOD_W<'a> {
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
#[doc = "Reader of field `ACLK_REF_GOOD`"]
pub type ACLK_REF_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACLK_REF_GOOD`"]
pub struct ACLK_REF_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_GOOD_W<'a> {
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
#[doc = "Reader of field `CLK_CHP_GOOD`"]
pub type CLK_CHP_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_CHP_GOOD`"]
pub struct CLK_CHP_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CHP_GOOD_W<'a> {
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
#[doc = "Reader of field `CLK_DCDC_GOOD`"]
pub type CLK_DCDC_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_DCDC_GOOD`"]
pub struct CLK_DCDC_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DCDC_GOOD_W<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\]
AMPCOMP FSM State"]
    #[inline(always)]
    pub fn rampstate(&self) -> RAMPSTATE_R {
        RAMPSTATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27 - 27:22\\]
XOSC_HF amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub fn hpm_update_amp(&self) -> HPM_UPDATE_AMP_R {
        HPM_UPDATE_AMP_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
XOSC_HF amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub fn lpm_update_amp(&self) -> LPM_UPDATE_AMP_R {
        LPM_UPDATE_AMP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
force_rcosc_hf"]
    #[inline(always)]
    pub fn force_rcosc_hf(&self) -> FORCE_RCOSC_HF_R {
        FORCE_RCOSC_HF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
SCLK_HF_EN"]
    #[inline(always)]
    pub fn sclk_hf_en(&self) -> SCLK_HF_EN_R {
        SCLK_HF_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
SCLK_MF_EN"]
    #[inline(always)]
    pub fn sclk_mf_en(&self) -> SCLK_MF_EN_R {
        SCLK_MF_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
ACLK_ADC_EN"]
    #[inline(always)]
    pub fn aclk_adc_en(&self) -> ACLK_ADC_EN_R {
        ACLK_ADC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
ACLK_TDC_EN"]
    #[inline(always)]
    pub fn aclk_tdc_en(&self) -> ACLK_TDC_EN_R {
        ACLK_TDC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
ACLK_REF_EN"]
    #[inline(always)]
    pub fn aclk_ref_en(&self) -> ACLK_REF_EN_R {
        ACLK_REF_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
CLK_CHP_EN"]
    #[inline(always)]
    pub fn clk_chp_en(&self) -> CLK_CHP_EN_R {
        CLK_CHP_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
CLK_DCDC_EN"]
    #[inline(always)]
    pub fn clk_dcdc_en(&self) -> CLK_DCDC_EN_R {
        CLK_DCDC_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF_GOOD"]
    #[inline(always)]
    pub fn sclk_hf_good(&self) -> SCLK_HF_GOOD_R {
        SCLK_HF_GOOD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_MF_GOOD"]
    #[inline(always)]
    pub fn sclk_mf_good(&self) -> SCLK_MF_GOOD_R {
        SCLK_MF_GOOD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SCLK_LF_GOOD"]
    #[inline(always)]
    pub fn sclk_lf_good(&self) -> SCLK_LF_GOOD_R {
        SCLK_LF_GOOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ACLK_ADC_GOOD"]
    #[inline(always)]
    pub fn aclk_adc_good(&self) -> ACLK_ADC_GOOD_R {
        ACLK_ADC_GOOD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ACLK_TDC_GOOD"]
    #[inline(always)]
    pub fn aclk_tdc_good(&self) -> ACLK_TDC_GOOD_R {
        ACLK_TDC_GOOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ACLK_REF_GOOD."]
    #[inline(always)]
    pub fn aclk_ref_good(&self) -> ACLK_REF_GOOD_R {
        ACLK_REF_GOOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLK_CHP_GOOD"]
    #[inline(always)]
    pub fn clk_chp_good(&self) -> CLK_CHP_GOOD_R {
        CLK_CHP_GOOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
CLK_DCDC_GOOD"]
    #[inline(always)]
    pub fn clk_dcdc_good(&self) -> CLK_DCDC_GOOD_R {
        CLK_DCDC_GOOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
AMPCOMP FSM State"]
    #[inline(always)]
    pub fn rampstate(&mut self) -> RAMPSTATE_W {
        RAMPSTATE_W { w: self }
    }
    #[doc = "Bits 22:27 - 27:22\\]
XOSC_HF amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub fn hpm_update_amp(&mut self) -> HPM_UPDATE_AMP_W {
        HPM_UPDATE_AMP_W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\]
XOSC_HF amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub fn lpm_update_amp(&mut self) -> LPM_UPDATE_AMP_W {
        LPM_UPDATE_AMP_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
force_rcosc_hf"]
    #[inline(always)]
    pub fn force_rcosc_hf(&mut self) -> FORCE_RCOSC_HF_W {
        FORCE_RCOSC_HF_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
SCLK_HF_EN"]
    #[inline(always)]
    pub fn sclk_hf_en(&mut self) -> SCLK_HF_EN_W {
        SCLK_HF_EN_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
SCLK_MF_EN"]
    #[inline(always)]
    pub fn sclk_mf_en(&mut self) -> SCLK_MF_EN_W {
        SCLK_MF_EN_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
ACLK_ADC_EN"]
    #[inline(always)]
    pub fn aclk_adc_en(&mut self) -> ACLK_ADC_EN_W {
        ACLK_ADC_EN_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
ACLK_TDC_EN"]
    #[inline(always)]
    pub fn aclk_tdc_en(&mut self) -> ACLK_TDC_EN_W {
        ACLK_TDC_EN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
ACLK_REF_EN"]
    #[inline(always)]
    pub fn aclk_ref_en(&mut self) -> ACLK_REF_EN_W {
        ACLK_REF_EN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
CLK_CHP_EN"]
    #[inline(always)]
    pub fn clk_chp_en(&mut self) -> CLK_CHP_EN_W {
        CLK_CHP_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
CLK_DCDC_EN"]
    #[inline(always)]
    pub fn clk_dcdc_en(&mut self) -> CLK_DCDC_EN_W {
        CLK_DCDC_EN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
SCLK_HF_GOOD"]
    #[inline(always)]
    pub fn sclk_hf_good(&mut self) -> SCLK_HF_GOOD_W {
        SCLK_HF_GOOD_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
SCLK_MF_GOOD"]
    #[inline(always)]
    pub fn sclk_mf_good(&mut self) -> SCLK_MF_GOOD_W {
        SCLK_MF_GOOD_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
SCLK_LF_GOOD"]
    #[inline(always)]
    pub fn sclk_lf_good(&mut self) -> SCLK_LF_GOOD_W {
        SCLK_LF_GOOD_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
ACLK_ADC_GOOD"]
    #[inline(always)]
    pub fn aclk_adc_good(&mut self) -> ACLK_ADC_GOOD_W {
        ACLK_ADC_GOOD_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ACLK_TDC_GOOD"]
    #[inline(always)]
    pub fn aclk_tdc_good(&mut self) -> ACLK_TDC_GOOD_W {
        ACLK_TDC_GOOD_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
ACLK_REF_GOOD."]
    #[inline(always)]
    pub fn aclk_ref_good(&mut self) -> ACLK_REF_GOOD_W {
        ACLK_REF_GOOD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
CLK_CHP_GOOD"]
    #[inline(always)]
    pub fn clk_chp_good(&mut self) -> CLK_CHP_GOOD_W {
        CLK_CHP_GOOD_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
CLK_DCDC_GOOD"]
    #[inline(always)]
    pub fn clk_dcdc_good(&mut self) -> CLK_DCDC_GOOD_W {
        CLK_DCDC_GOOD_W { w: self }
    }
}
