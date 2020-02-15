#[doc = "Reader of register XOSCHFCTL"]
pub type R = crate::R<u32, super::XOSCHFCTL>;
#[doc = "Writer for register XOSCHFCTL"]
pub type W = crate::W<u32, super::XOSCHFCTL>;
#[doc = "Register XOSCHFCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::XOSCHFCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE14`"]
pub type SPARE14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPARE14`"]
pub struct SPARE14_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | (((value as u32) & 0x0003_ffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `TCXO_MODE_XOSC_HF_EN`"]
pub type TCXO_MODE_XOSC_HF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCXO_MODE_XOSC_HF_EN`"]
pub struct TCXO_MODE_XOSC_HF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCXO_MODE_XOSC_HF_EN_W<'a> {
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
#[doc = "Reader of field `TCXO_MODE`"]
pub type TCXO_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCXO_MODE`"]
pub struct TCXO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCXO_MODE_W<'a> {
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
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PEAK_DET_ITRIM`"]
pub type PEAK_DET_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PEAK_DET_ITRIM`"]
pub struct PEAK_DET_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAK_DET_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
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
#[doc = "Reader of field `HP_BUF_ITRIM`"]
pub type HP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HP_BUF_ITRIM`"]
pub struct HP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `LP_BUF_ITRIM`"]
pub type LP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LP_BUF_ITRIM`"]
pub struct LP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare14(&self) -> SPARE14_R {
        SPARE14_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 13 - 13:13\\]
If this register is 1 when TCXO_MODE is 1, then the XOSC_HF is enabled, turning on the XOSC_HF bias current allowing a DC bias point to be provided to the clipped-sine wave clock signal on external input."]
    #[inline(always)]
    pub fn tcxo_mode_xosc_hf_en(&self) -> TCXO_MODE_XOSC_HF_EN_R {
        TCXO_MODE_XOSC_HF_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
If this register is 1 when BYPASS is 1, this will enable clock qualification on the TCXO clock on external input. This register has no effect when BYPASS is 0."]
    #[inline(always)]
    pub fn tcxo_mode(&self) -> TCXO_MODE_R {
        TCXO_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare14(&mut self) -> SPARE14_W {
        SPARE14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
If this register is 1 when TCXO_MODE is 1, then the XOSC_HF is enabled, turning on the XOSC_HF bias current allowing a DC bias point to be provided to the clipped-sine wave clock signal on external input."]
    #[inline(always)]
    pub fn tcxo_mode_xosc_hf_en(&mut self) -> TCXO_MODE_XOSC_HF_EN_W {
        TCXO_MODE_XOSC_HF_EN_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
If this register is 1 when BYPASS is 1, this will enable clock qualification on the TCXO clock on external input. This register has no effect when BYPASS is 0."]
    #[inline(always)]
    pub fn tcxo_mode(&mut self) -> TCXO_MODE_W {
        TCXO_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&mut self) -> PEAK_DET_ITRIM_W {
        PEAK_DET_ITRIM_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&mut self) -> HP_BUF_ITRIM_W {
        HP_BUF_ITRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&mut self) -> LP_BUF_ITRIM_W {
        LP_BUF_ITRIM_W { w: self }
    }
}
