#[doc = "Reader of register FTCTL"]
pub type R = crate::R<u32, super::FTCTL>;
#[doc = "Writer for register FTCTL"]
pub type W = crate::W<u32, super::FTCTL>;
#[doc = "Register FTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED17`"]
pub type RESERVED17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED17`"]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `WDATA_BLK_CLR`"]
pub type WDATA_BLK_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDATA_BLK_CLR`"]
pub struct WDATA_BLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_BLK_CLR_W<'a> {
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
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `TEST_EN`"]
pub type TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_EN`"]
pub struct TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_EN_W<'a> {
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
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wdata_blk_clr(&self) -> WDATA_BLK_CLR_R {
        WDATA_BLK_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wdata_blk_clr(&mut self) -> WDATA_BLK_CLR_W {
        WDATA_BLK_CLR_W { w: self }
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn test_en(&mut self) -> TEST_EN_W {
        TEST_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
