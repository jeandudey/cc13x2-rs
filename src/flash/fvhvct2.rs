#[doc = "Reader of register FVHVCT2"]
pub type R = crate::R<u32, super::FVHVCT2>;
#[doc = "Writer for register FVHVCT2"]
pub type W = crate::W<u32, super::FVHVCT2>;
#[doc = "Register FVHVCT2 `reset()`'s with value 0x00a2_0000"]
impl crate::ResetValue for super::FVHVCT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00a2_0000
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `TRIM13_P`"]
pub type TRIM13_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM13_P`"]
pub struct TRIM13_P_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_P`"]
pub type VHVCT_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_P`"]
pub struct VHVCT_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_p(&self) -> TRIM13_P_R {
        TRIM13_P_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_p(&self) -> VHVCT_P_R {
        VHVCT_P_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_p(&mut self) -> TRIM13_P_W {
        TRIM13_P_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_p(&mut self) -> VHVCT_P_W {
        VHVCT_P_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
