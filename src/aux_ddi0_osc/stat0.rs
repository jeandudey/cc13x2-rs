#[doc = "Reader of register STAT0"]
pub type R = crate::R<u32, super::STAT0>;
#[doc = "Writer for register STAT0"]
pub type W = crate::W<u32, super::STAT0>;
#[doc = "Register STAT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE31`"]
pub type SPARE31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE31`"]
pub struct SPARE31_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE31_W<'a> {
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
#[doc = "30:29\\]
Indicates source for the sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLK_LF_SRC_A {
    #[doc = "3: Low frequency XOSC"]
    XOSCLF = 3,
    #[doc = "2: Low frequency RCOSC"]
    RCOSCLF = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF = 0,
}
impl From<SCLK_LF_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCLK_LF_SRC`"]
pub type SCLK_LF_SRC_R = crate::R<u8, SCLK_LF_SRC_A>;
impl SCLK_LF_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_SRC_A {
        match self.bits {
            3 => SCLK_LF_SRC_A::XOSCLF,
            2 => SCLK_LF_SRC_A::RCOSCLF,
            1 => SCLK_LF_SRC_A::XOSCHFDLF,
            0 => SCLK_LF_SRC_A::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        *self == SCLK_LF_SRC_A::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        *self == SCLK_LF_SRC_A::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_A::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_A::RCOSCHFDLF
    }
}
#[doc = "Write proxy for field `SCLK_LF_SRC`"]
pub struct SCLK_LF_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_LF_SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::XOSCLF)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::RCOSCLF)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::XOSCHFDLF)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_A::RCOSCHFDLF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "28:28\\]
Indicates source for the sclk_hf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_HF_SRC_A {
    #[doc = "1: High frequency XOSC"]
    XOSC = 1,
    #[doc = "0: High frequency RCOSC clock"]
    RCOSC = 0,
}
impl From<SCLK_HF_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_HF_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLK_HF_SRC`"]
pub type SCLK_HF_SRC_R = crate::R<bool, SCLK_HF_SRC_A>;
impl SCLK_HF_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_SRC_A {
        match self.bits {
            true => SCLK_HF_SRC_A::XOSC,
            false => SCLK_HF_SRC_A::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SCLK_HF_SRC_A::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        *self == SCLK_HF_SRC_A::RCOSC
    }
}
#[doc = "Write proxy for field `SCLK_HF_SRC`"]
pub struct SCLK_HF_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_HF_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High frequency XOSC"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_A::XOSC)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_A::RCOSC)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RESERVED23`"]
pub type RESERVED23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED23`"]
pub struct RESERVED23_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | (((value as u32) & 0x1f) << 23);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_HF_EN`"]
pub type RCOSC_HF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_HF_EN`"]
pub struct RCOSC_HF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_EN_W<'a> {
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
#[doc = "Reader of field `RCOSC_LF_EN`"]
pub type RCOSC_LF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_LF_EN`"]
pub struct RCOSC_LF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_LF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `XOSC_LF_EN`"]
pub type XOSC_LF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_LF_EN`"]
pub struct XOSC_LF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_LF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CLK_DCDC_RDY`"]
pub type CLK_DCDC_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_DCDC_RDY`"]
pub struct CLK_DCDC_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DCDC_RDY_W<'a> {
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
#[doc = "Reader of field `CLK_DCDC_RDY_ACK`"]
pub type CLK_DCDC_RDY_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_DCDC_RDY_ACK`"]
pub struct CLK_DCDC_RDY_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DCDC_RDY_ACK_W<'a> {
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
#[doc = "Reader of field `SCLK_HF_LOSS`"]
pub type SCLK_HF_LOSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_HF_LOSS`"]
pub struct SCLK_HF_LOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_LOSS_W<'a> {
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
#[doc = "Reader of field `SCLK_LF_LOSS`"]
pub type SCLK_LF_LOSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_LF_LOSS`"]
pub struct SCLK_LF_LOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_LOSS_W<'a> {
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
#[doc = "Reader of field `XOSC_HF_EN`"]
pub type XOSC_HF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_EN`"]
pub struct XOSC_HF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED14`"]
pub type RESERVED14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED14`"]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
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
#[doc = "Reader of field `XB_48M_CLK_EN`"]
pub type XB_48M_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XB_48M_CLK_EN`"]
pub struct XB_48M_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XB_48M_CLK_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
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
#[doc = "Reader of field `XOSC_HF_LP_BUF_EN`"]
pub type XOSC_HF_LP_BUF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_LP_BUF_EN`"]
pub struct XOSC_HF_LP_BUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_LP_BUF_EN_W<'a> {
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
#[doc = "Reader of field `XOSC_HF_HP_BUF_EN`"]
pub type XOSC_HF_HP_BUF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_HP_BUF_EN`"]
pub struct XOSC_HF_HP_BUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_HP_BUF_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
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
#[doc = "Reader of field `ADC_THMET`"]
pub type ADC_THMET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_THMET`"]
pub struct ADC_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_THMET_W<'a> {
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
#[doc = "Reader of field `ADC_DATA_READY`"]
pub type ADC_DATA_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_DATA_READY`"]
pub struct ADC_DATA_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DATA_READY_W<'a> {
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
#[doc = "Reader of field `ADC_DATA`"]
pub type ADC_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_DATA`"]
pub struct ADC_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
#[doc = "Reader of field `PENDINGSCLKHFSWITCHING`"]
pub type PENDINGSCLKHFSWITCHING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDINGSCLKHFSWITCHING`"]
pub struct PENDINGSCLKHFSWITCHING_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDINGSCLKHFSWITCHING_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&self) -> SPARE31_R {
        SPARE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Indicates source for the sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src(&self) -> SCLK_LF_SRC_R {
        SCLK_LF_SRC_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Indicates source for the sclk_hf"]
    #[inline(always)]
    pub fn sclk_hf_src(&self) -> SCLK_HF_SRC_R {
        SCLK_HF_SRC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 23:27 - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
RCOSC_HF_EN"]
    #[inline(always)]
    pub fn rcosc_hf_en(&self) -> RCOSC_HF_EN_R {
        RCOSC_HF_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
RCOSC_LF_EN"]
    #[inline(always)]
    pub fn rcosc_lf_en(&self) -> RCOSC_LF_EN_R {
        RCOSC_LF_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
XOSC_LF_EN"]
    #[inline(always)]
    pub fn xosc_lf_en(&self) -> XOSC_LF_EN_R {
        XOSC_LF_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
CLK_DCDC_RDY"]
    #[inline(always)]
    pub fn clk_dcdc_rdy(&self) -> CLK_DCDC_RDY_R {
        CLK_DCDC_RDY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    pub fn clk_dcdc_rdy_ack(&self) -> CLK_DCDC_RDY_ACK_R {
        CLK_DCDC_RDY_ACK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates sclk_hf is lost"]
    #[inline(always)]
    pub fn sclk_hf_loss(&self) -> SCLK_HF_LOSS_R {
        SCLK_HF_LOSS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates sclk_lf is lost"]
    #[inline(always)]
    pub fn sclk_lf_loss(&self) -> SCLK_LF_LOSS_R {
        SCLK_LF_LOSS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    pub fn xosc_hf_en(&self) -> XOSC_HF_EN_R {
        XOSC_HF_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    pub fn xb_48m_clk_en(&self) -> XB_48M_CLK_EN_R {
        XB_48M_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_lp_buf_en(&self) -> XOSC_HF_LP_BUF_EN_R {
        XOSC_HF_LP_BUF_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_hp_buf_en(&self) -> XOSC_HF_HP_BUF_EN_R {
        XOSC_HF_HP_BUF_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC_THMET"]
    #[inline(always)]
    pub fn adc_thmet(&self) -> ADC_THMET_R {
        ADC_THMET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
indicates when adc_data is ready."]
    #[inline(always)]
    pub fn adc_data_ready(&self) -> ADC_DATA_READY_R {
        ADC_DATA_READY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
adc_data"]
    #[inline(always)]
    pub fn adc_data(&self) -> ADC_DATA_R {
        ADC_DATA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates when SCLK_HF clock source is ready to be switched"]
    #[inline(always)]
    pub fn pendingsclkhfswitching(&self) -> PENDINGSCLKHFSWITCHING_R {
        PENDINGSCLKHFSWITCHING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&mut self) -> SPARE31_W {
        SPARE31_W { w: self }
    }
    #[doc = "Bits 29:30 - 30:29\\]
Indicates source for the sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src(&mut self) -> SCLK_LF_SRC_W {
        SCLK_LF_SRC_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Indicates source for the sclk_hf"]
    #[inline(always)]
    pub fn sclk_hf_src(&mut self) -> SCLK_HF_SRC_W {
        SCLK_HF_SRC_W { w: self }
    }
    #[doc = "Bits 23:27 - 27:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&mut self) -> RESERVED23_W {
        RESERVED23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
RCOSC_HF_EN"]
    #[inline(always)]
    pub fn rcosc_hf_en(&mut self) -> RCOSC_HF_EN_W {
        RCOSC_HF_EN_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
RCOSC_LF_EN"]
    #[inline(always)]
    pub fn rcosc_lf_en(&mut self) -> RCOSC_LF_EN_W {
        RCOSC_LF_EN_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
XOSC_LF_EN"]
    #[inline(always)]
    pub fn xosc_lf_en(&mut self) -> XOSC_LF_EN_W {
        XOSC_LF_EN_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
CLK_DCDC_RDY"]
    #[inline(always)]
    pub fn clk_dcdc_rdy(&mut self) -> CLK_DCDC_RDY_W {
        CLK_DCDC_RDY_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    pub fn clk_dcdc_rdy_ack(&mut self) -> CLK_DCDC_RDY_ACK_W {
        CLK_DCDC_RDY_ACK_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates sclk_hf is lost"]
    #[inline(always)]
    pub fn sclk_hf_loss(&mut self) -> SCLK_HF_LOSS_W {
        SCLK_HF_LOSS_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates sclk_lf is lost"]
    #[inline(always)]
    pub fn sclk_lf_loss(&mut self) -> SCLK_LF_LOSS_W {
        SCLK_LF_LOSS_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    pub fn xosc_hf_en(&mut self) -> XOSC_HF_EN_W {
        XOSC_HF_EN_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    pub fn xb_48m_clk_en(&mut self) -> XB_48M_CLK_EN_W {
        XB_48M_CLK_EN_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_lp_buf_en(&mut self) -> XOSC_HF_LP_BUF_EN_W {
        XOSC_HF_LP_BUF_EN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_hp_buf_en(&mut self) -> XOSC_HF_HP_BUF_EN_W {
        XOSC_HF_HP_BUF_EN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
ADC_THMET"]
    #[inline(always)]
    pub fn adc_thmet(&mut self) -> ADC_THMET_W {
        ADC_THMET_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
indicates when adc_data is ready."]
    #[inline(always)]
    pub fn adc_data_ready(&mut self) -> ADC_DATA_READY_W {
        ADC_DATA_READY_W { w: self }
    }
    #[doc = "Bits 1:6 - 6:1\\]
adc_data"]
    #[inline(always)]
    pub fn adc_data(&mut self) -> ADC_DATA_W {
        ADC_DATA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates when SCLK_HF clock source is ready to be switched"]
    #[inline(always)]
    pub fn pendingsclkhfswitching(&mut self) -> PENDINGSCLKHFSWITCHING_W {
        PENDINGSCLKHFSWITCHING_W { w: self }
    }
}
