#[doc = "Reader of register FCFG_BNK_TYPE"]
pub type R = crate::R<u32, super::FCFG_BNK_TYPE>;
#[doc = "Writer for register FCFG_BNK_TYPE"]
pub type W = crate::W<u32, super::FCFG_BNK_TYPE>;
#[doc = "Register FCFG_BNK_TYPE `reset()`'s with value 0x04"]
impl crate::ResetValue for super::FCFG_BNK_TYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `B7_TYPE`"]
pub type B7_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B7_TYPE`"]
pub struct B7_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `B6_TYPE`"]
pub type B6_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B6_TYPE`"]
pub struct B6_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B6_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `B5_TYPE`"]
pub type B5_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B5_TYPE`"]
pub struct B5_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B5_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `B4_TYPE`"]
pub type B4_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B4_TYPE`"]
pub struct B4_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B4_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `B3_TYPE`"]
pub type B3_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B3_TYPE`"]
pub struct B3_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `B2_TYPE`"]
pub type B2_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B2_TYPE`"]
pub struct B2_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `B1_TYPE`"]
pub type B1_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B1_TYPE`"]
pub struct B1_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `B0_TYPE`"]
pub type B0_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B0_TYPE`"]
pub struct B0_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_type(&self) -> B7_TYPE_R {
        B7_TYPE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b6_type(&self) -> B6_TYPE_R {
        B6_TYPE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b5_type(&self) -> B5_TYPE_R {
        B5_TYPE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_type(&self) -> B4_TYPE_R {
        B4_TYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_type(&self) -> B3_TYPE_R {
        B3_TYPE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_type(&self) -> B2_TYPE_R {
        B2_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_type(&self) -> B1_TYPE_R {
        B1_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_type(&self) -> B0_TYPE_R {
        B0_TYPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_type(&mut self) -> B7_TYPE_W {
        B7_TYPE_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b6_type(&mut self) -> B6_TYPE_W {
        B6_TYPE_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b5_type(&mut self) -> B5_TYPE_W {
        B5_TYPE_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_type(&mut self) -> B4_TYPE_W {
        B4_TYPE_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_type(&mut self) -> B3_TYPE_W {
        B3_TYPE_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_type(&mut self) -> B2_TYPE_W {
        B2_TYPE_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_type(&mut self) -> B1_TYPE_W {
        B1_TYPE_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_type(&mut self) -> B0_TYPE_W {
        B0_TYPE_W { w: self }
    }
}
