#[doc = "Reader of register FEFUSECTL"]
pub type R = crate::R<u32, super::FEFUSECTL>;
#[doc = "Writer for register FEFUSECTL"]
pub type W = crate::W<u32, super::FEFUSECTL>;
#[doc = "Register FEFUSECTL `reset()`'s with value 0x0701_010a"]
impl crate::ResetValue for super::FEFUSECTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0701_010a
    }
}
#[doc = "Reader of field `RESERVED27`"]
pub type RESERVED27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED27`"]
pub struct RESERVED27_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `CHAIN_SEL`"]
pub type CHAIN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHAIN_SEL`"]
pub struct CHAIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED18`"]
pub type RESERVED18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED18`"]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `WRITE_EN`"]
pub type WRITE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE_EN`"]
pub struct WRITE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_EN_W<'a> {
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
#[doc = "Reader of field `BP_SEL`"]
pub type BP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BP_SEL`"]
pub struct BP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_SEL_W<'a> {
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
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `EF_CLRZ`"]
pub type EF_CLRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EF_CLRZ`"]
pub struct EF_CLRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CLRZ_W<'a> {
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
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `EF_TEST`"]
pub type EF_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EF_TEST`"]
pub struct EF_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_TEST_W<'a> {
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
#[doc = "Reader of field `EFUSE_EN`"]
pub type EFUSE_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_EN`"]
pub struct EFUSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn chain_sel(&self) -> CHAIN_SEL_R {
        CHAIN_SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn write_en(&self) -> WRITE_EN_R {
        WRITE_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_sel(&self) -> BP_SEL_R {
        BP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_clrz(&self) -> EF_CLRZ_R {
        EF_CLRZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_test(&self) -> EF_TEST_R {
        EF_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuse_en(&self) -> EFUSE_EN_R {
        EFUSE_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved27(&mut self) -> RESERVED27_W {
        RESERVED27_W { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn chain_sel(&mut self) -> CHAIN_SEL_W {
        CHAIN_SEL_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn write_en(&mut self) -> WRITE_EN_W {
        WRITE_EN_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_sel(&mut self) -> BP_SEL_W {
        BP_SEL_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_clrz(&mut self) -> EF_CLRZ_W {
        EF_CLRZ_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_test(&mut self) -> EF_TEST_W {
        EF_TEST_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuse_en(&mut self) -> EFUSE_EN_W {
        EFUSE_EN_W { w: self }
    }
}
