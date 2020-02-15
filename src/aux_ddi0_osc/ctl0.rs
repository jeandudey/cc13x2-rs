#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "31:31\\]
Set based on the accurate high frequency XTAL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_IS_24M_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _24M = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _48M = 0,
}
impl From<XTAL_IS_24M_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_IS_24M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL_IS_24M`"]
pub type XTAL_IS_24M_R = crate::R<bool, XTAL_IS_24M_A>;
impl XTAL_IS_24M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_IS_24M_A {
        match self.bits {
            true => XTAL_IS_24M_A::_24M,
            false => XTAL_IS_24M_A::_48M,
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == XTAL_IS_24M_A::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        *self == XTAL_IS_24M_A::_48M
    }
}
#[doc = "Write proxy for field `XTAL_IS_24M`"]
pub struct XTAL_IS_24M_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_IS_24M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_IS_24M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XTAL_IS_24M_A::_24M)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XTAL_IS_24M_A::_48M)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RESERVED30`"]
pub type RESERVED30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED30`"]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `BYPASS_XOSC_LF_CLK_QUAL`"]
pub type BYPASS_XOSC_LF_CLK_QUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_XOSC_LF_CLK_QUAL`"]
pub struct BYPASS_XOSC_LF_CLK_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_XOSC_LF_CLK_QUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `BYPASS_RCOSC_LF_CLK_QUAL`"]
pub type BYPASS_RCOSC_LF_CLK_QUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_RCOSC_LF_CLK_QUAL`"]
pub struct BYPASS_RCOSC_LF_CLK_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_RCOSC_LF_CLK_QUAL_W<'a> {
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
#[doc = "Reader of field `DOUBLER_START_DURATION`"]
pub type DOUBLER_START_DURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOUBLER_START_DURATION`"]
pub struct DOUBLER_START_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLER_START_DURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DOUBLER_RESET_DURATION`"]
pub type DOUBLER_RESET_DURATION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUBLER_RESET_DURATION`"]
pub struct DOUBLER_RESET_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLER_RESET_DURATION_W<'a> {
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
#[doc = "Reader of field `CLK_DCDC_SRC_SEL`"]
pub type CLK_DCDC_SRC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_DCDC_SRC_SEL`"]
pub struct CLK_DCDC_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DCDC_SRC_SEL_W<'a> {
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
#[doc = "Reader of field `RESERVED15`"]
pub type RESERVED15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED15`"]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 15)) | (((value as u32) & 0x01ff) << 15);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_MODE_EN`"]
pub type HPOSC_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPOSC_MODE_EN`"]
pub struct HPOSC_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_MODE_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED13`"]
pub type RESERVED13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED13`"]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
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
#[doc = "Reader of field `RCOSC_LF_TRIMMED`"]
pub type RCOSC_LF_TRIMMED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_LF_TRIMMED`"]
pub struct RCOSC_LF_TRIMMED_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_LF_TRIMMED_W<'a> {
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
#[doc = "Reader of field `XOSC_HF_POWER_MODE`"]
pub type XOSC_HF_POWER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_POWER_MODE`"]
pub struct XOSC_HF_POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_POWER_MODE_W<'a> {
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
#[doc = "Reader of field `XOSC_LF_DIG_BYPASS`"]
pub type XOSC_LF_DIG_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_LF_DIG_BYPASS`"]
pub struct XOSC_LF_DIG_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_LF_DIG_BYPASS_W<'a> {
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
#[doc = "Reader of field `CLK_LOSS_EN`"]
pub type CLK_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_LOSS_EN`"]
pub struct CLK_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LOSS_EN_W<'a> {
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
#[doc = "Reader of field `ACLK_TDC_SRC_SEL`"]
pub type ACLK_TDC_SRC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACLK_TDC_SRC_SEL`"]
pub struct ACLK_TDC_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_TDC_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `ACLK_REF_SRC_SEL`"]
pub type ACLK_REF_SRC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACLK_REF_SRC_SEL`"]
pub struct ACLK_REF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "3:2\\]
Source select for sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLK_LF_SRC_SEL_A {
    #[doc = "3: Low frequency XOSC"]
    XOSCLF = 3,
    #[doc = "2: Low frequency RCOSC"]
    RCOSCLF = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF = 0,
}
impl From<SCLK_LF_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCLK_LF_SRC_SEL`"]
pub type SCLK_LF_SRC_SEL_R = crate::R<u8, SCLK_LF_SRC_SEL_A>;
impl SCLK_LF_SRC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_SRC_SEL_A {
        match self.bits {
            3 => SCLK_LF_SRC_SEL_A::XOSCLF,
            2 => SCLK_LF_SRC_SEL_A::RCOSCLF,
            1 => SCLK_LF_SRC_SEL_A::XOSCHFDLF,
            0 => SCLK_LF_SRC_SEL_A::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_SEL_A::RCOSCHFDLF
    }
}
#[doc = "Write proxy for field `SCLK_LF_SRC_SEL`"]
pub struct SCLK_LF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_LF_SRC_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::XOSCLF)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::RCOSCLF)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::XOSCHFDLF)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::RCOSCHFDLF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
#[doc = "0:0\\]
Source select for sclk_hf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_HF_SRC_SEL_A {
    #[doc = "1: High frequency XOSC clock"]
    XOSC = 1,
    #[doc = "0: High frequency RCOSC clock"]
    RCOSC = 0,
}
impl From<SCLK_HF_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_HF_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLK_HF_SRC_SEL`"]
pub type SCLK_HF_SRC_SEL_R = crate::R<bool, SCLK_HF_SRC_SEL_A>;
impl SCLK_HF_SRC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_SRC_SEL_A {
        match self.bits {
            true => SCLK_HF_SRC_SEL_A::XOSC,
            false => SCLK_HF_SRC_SEL_A::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SCLK_HF_SRC_SEL_A::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        *self == SCLK_HF_SRC_SEL_A::RCOSC
    }
}
#[doc = "Write proxy for field `SCLK_HF_SRC_SEL`"]
pub struct SCLK_HF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_HF_SRC_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High frequency XOSC clock"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SEL_A::XOSC)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SEL_A::RCOSC)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub fn xtal_is_24m(&self) -> XTAL_IS_24M_R {
        XTAL_IS_24M_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&self) -> BYPASS_XOSC_LF_CLK_QUAL_R {
        BYPASS_XOSC_LF_CLK_QUAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&self) -> BYPASS_RCOSC_LF_CLK_QUAL_R {
        BYPASS_RCOSC_LF_CLK_QUAL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_start_duration(&self) -> DOUBLER_START_DURATION_R {
        DOUBLER_START_DURATION_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_reset_duration(&self) -> DOUBLER_RESET_DURATION_R {
        DOUBLER_RESET_DURATION_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Select DCDC clock source. 0: CLK_DCDC is 48 MHz clock from RCOSC or XOSC / HPOSC 1: CLK_DCDC is always 48 MHz clock from RCOSC"]
    #[inline(always)]
    pub fn clk_dcdc_src_sel(&self) -> CLK_DCDC_SRC_SEL_R {
        CLK_DCDC_SRC_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 15:23 - 23:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_mode_en(&self) -> HPOSC_MODE_EN_R {
        HPOSC_MODE_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&self) -> RCOSC_LF_TRIMMED_R {
        RCOSC_LF_TRIMMED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&self) -> XOSC_HF_POWER_MODE_R {
        XOSC_HF_POWER_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&self) -> XOSC_LF_DIG_BYPASS_R {
        XOSC_LF_DIG_BYPASS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to the system controller. Checks both SCLK_HF, SCLK_MF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&self) -> ACLK_TDC_SRC_SEL_R {
        ACLK_TDC_SRC_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Source select for aclk_ref 000: RCOSC_HF derived (31.25kHz) 001: XOSC_HF derived (31.25kHz) 010: RCOSC_LF (32kHz) 011: XOSC_LF (32.768kHz) 100: RCOSC_MF (2MHz) 101-111: Not used"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&self) -> ACLK_REF_SRC_SEL_R {
        ACLK_REF_SRC_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&self) -> SCLK_LF_SRC_SEL_R {
        SCLK_LF_SRC_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf."]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&self) -> SCLK_HF_SRC_SEL_R {
        SCLK_HF_SRC_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub fn xtal_is_24m(&mut self) -> XTAL_IS_24M_W {
        XTAL_IS_24M_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&mut self) -> BYPASS_XOSC_LF_CLK_QUAL_W {
        BYPASS_XOSC_LF_CLK_QUAL_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&mut self) -> BYPASS_RCOSC_LF_CLK_QUAL_W {
        BYPASS_RCOSC_LF_CLK_QUAL_W { w: self }
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_start_duration(&mut self) -> DOUBLER_START_DURATION_W {
        DOUBLER_START_DURATION_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_reset_duration(&mut self) -> DOUBLER_RESET_DURATION_W {
        DOUBLER_RESET_DURATION_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Select DCDC clock source. 0: CLK_DCDC is 48 MHz clock from RCOSC or XOSC / HPOSC 1: CLK_DCDC is always 48 MHz clock from RCOSC"]
    #[inline(always)]
    pub fn clk_dcdc_src_sel(&mut self) -> CLK_DCDC_SRC_SEL_W {
        CLK_DCDC_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 15:23 - 23:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_mode_en(&mut self) -> HPOSC_MODE_EN_W {
        HPOSC_MODE_EN_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&mut self) -> RCOSC_LF_TRIMMED_W {
        RCOSC_LF_TRIMMED_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&mut self) -> XOSC_HF_POWER_MODE_W {
        XOSC_HF_POWER_MODE_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&mut self) -> XOSC_LF_DIG_BYPASS_W {
        XOSC_LF_DIG_BYPASS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to the system controller. Checks both SCLK_HF, SCLK_MF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W {
        CLK_LOSS_EN_W { w: self }
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&mut self) -> ACLK_TDC_SRC_SEL_W {
        ACLK_TDC_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\]
Source select for aclk_ref 000: RCOSC_HF derived (31.25kHz) 001: XOSC_HF derived (31.25kHz) 010: RCOSC_LF (32kHz) 011: XOSC_LF (32.768kHz) 100: RCOSC_MF (2MHz) 101-111: Not used"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&mut self) -> ACLK_REF_SRC_SEL_W {
        ACLK_REF_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&mut self) -> SCLK_LF_SRC_SEL_W {
        SCLK_LF_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf."]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&mut self) -> SCLK_HF_SRC_SEL_W {
        SCLK_HF_SRC_SEL_W { w: self }
    }
}
