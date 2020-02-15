#[doc = "Reader of register ADCDOUBLERNANOAMPCTL"]
pub type R = crate::R<u32, super::ADCDOUBLERNANOAMPCTL>;
#[doc = "Writer for register ADCDOUBLERNANOAMPCTL"]
pub type W = crate::W<u32, super::ADCDOUBLERNANOAMPCTL>;
#[doc = "Register ADCDOUBLERNANOAMPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCDOUBLERNANOAMPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `NANOAMP_BIAS_ENABLE`"]
pub type NANOAMP_BIAS_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NANOAMP_BIAS_ENABLE`"]
pub struct NANOAMP_BIAS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NANOAMP_BIAS_ENABLE_W<'a> {
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
#[doc = "Reader of field `SPARE23`"]
pub type SPARE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE23`"]
pub struct SPARE23_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 6)) | (((value as u32) & 0x0001_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC_SH_MODE_EN`"]
pub type ADC_SH_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SH_MODE_EN`"]
pub struct ADC_SH_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_MODE_EN_W<'a> {
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
#[doc = "Reader of field `ADC_SH_VBUF_EN`"]
pub type ADC_SH_VBUF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SH_VBUF_EN`"]
pub struct ADC_SH_VBUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_VBUF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADC_IREF_CTRL`"]
pub type ADC_IREF_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_IREF_CTRL`"]
pub struct ADC_IREF_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IREF_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&self) -> NANOAMP_BIAS_ENABLE_R {
        NANOAMP_BIAS_ENABLE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn spare23(&self) -> SPARE23_R {
        SPARE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_iref_ctrl(&self) -> ADC_IREF_CTRL_R {
        ADC_IREF_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&mut self) -> NANOAMP_BIAS_ENABLE_W {
        NANOAMP_BIAS_ENABLE_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn spare23(&mut self) -> SPARE23_W {
        SPARE23_W { w: self }
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&mut self) -> ADC_SH_MODE_EN_W {
        ADC_SH_MODE_EN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&mut self) -> ADC_SH_VBUF_EN_W {
        ADC_SH_VBUF_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_iref_ctrl(&mut self) -> ADC_IREF_CTRL_W {
        ADC_IREF_CTRL_W { w: self }
    }
}
