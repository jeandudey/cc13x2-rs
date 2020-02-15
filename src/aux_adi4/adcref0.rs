#[doc = "Reader of register ADCREF0"]
pub type R = crate::R<u8, super::ADCREF0>;
#[doc = "Writer for register ADCREF0"]
pub type W = crate::W<u8, super::ADCREF0>;
#[doc = "Register ADCREF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCREF0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE7`"]
pub type SPARE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE7`"]
pub struct SPARE7_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE7_W<'a> {
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
#[doc = "Reader of field `REF_ON_IDLE`"]
pub type REF_ON_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REF_ON_IDLE`"]
pub struct REF_ON_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_ON_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IOMUX`"]
pub type IOMUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOMUX`"]
pub struct IOMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT`"]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare7(&self) -> SPARE7_R {
        SPARE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable ADCREF in IDLE state. 0: Disabled in IDLE state 1: Enabled in IDLE state Keep ADCREF enabled when ADC0.SMPL_MODE = 0. Recommendation: Enable ADCREF always when ADC0.SMPL_CYCLE_EXP is less than 0x6 (21.3us sampling time)."]
    #[inline(always)]
    pub fn ref_on_idle(&self) -> REF_ON_IDLE_R {
        REF_ON_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare7(&mut self) -> SPARE7_W {
        SPARE7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Enable ADCREF in IDLE state. 0: Disabled in IDLE state 1: Enabled in IDLE state Keep ADCREF enabled when ADC0.SMPL_MODE = 0. Recommendation: Enable ADCREF always when ADC0.SMPL_CYCLE_EXP is less than 0x6 (21.3us sampling time)."]
    #[inline(always)]
    pub fn ref_on_idle(&mut self) -> REF_ON_IDLE_W {
        REF_ON_IDLE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iomux(&mut self) -> IOMUX_W {
        IOMUX_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
