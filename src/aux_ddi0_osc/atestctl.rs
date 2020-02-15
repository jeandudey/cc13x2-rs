#[doc = "Reader of register ATESTCTL"]
pub type R = crate::R<u32, super::ATESTCTL>;
#[doc = "Writer for register ATESTCTL"]
pub type W = crate::W<u32, super::ATESTCTL>;
#[doc = "Register ATESTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ATESTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCLK_LF_AUX_EN`"]
pub type SCLK_LF_AUX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_LF_AUX_EN`"]
pub struct SCLK_LF_AUX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_AUX_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TEST_RCOSCMF`"]
pub type TEST_RCOSCMF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_RCOSCMF`"]
pub struct TEST_RCOSCMF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_RCOSCMF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `ATEST_RCOSCMF`"]
pub type ATEST_RCOSCMF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATEST_RCOSCMF`"]
pub struct ATEST_RCOSCMF_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_RCOSCMF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SCLK_LF_AUX_EN_R {
        SCLK_LF_AUX_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
    #[inline(always)]
    pub fn test_rcoscmf(&self) -> TEST_RCOSCMF_R {
        TEST_RCOSCMF_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
    #[inline(always)]
    pub fn atest_rcoscmf(&self) -> ATEST_RCOSCMF_R {
        ATEST_RCOSCMF_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&mut self) -> SCLK_LF_AUX_EN_W {
        SCLK_LF_AUX_EN_W { w: self }
    }
    #[doc = "Bits 16:30 - 30:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Test mode control for RCOSC_MF 0x0: test modes disabled 0x1: boosted bias current into self biased inverter 0x2: clock qualification disabled 0x3: boosted bias current into self biased inverter + clock qualification disabled"]
    #[inline(always)]
    pub fn test_rcoscmf(&mut self) -> TEST_RCOSCMF_W {
        TEST_RCOSCMF_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
ATEST control for RCOSC_MF 0x0: ATEST disabled 0x1: ATEST enabled, VDD_LOCAL connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock. 0x2: ATEST disabled 0x3: ATEST enabled, bias current connected, ATEST internal to **RCOSC_MF* enabled to send out 2MHz clock."]
    #[inline(always)]
    pub fn atest_rcoscmf(&mut self) -> ATEST_RCOSCMF_W {
        ATEST_RCOSCMF_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
