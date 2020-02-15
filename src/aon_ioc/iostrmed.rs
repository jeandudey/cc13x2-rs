#[doc = "Reader of register IOSTRMED"]
pub type R = crate::R<u32, super::IOSTRMED>;
#[doc = "Writer for register IOSTRMED"]
pub type W = crate::W<u32, super::IOSTRMED>;
#[doc = "Register IOSTRMED `reset()`'s with value 0x06"]
impl crate::ResetValue for super::IOSTRMED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `GRAY_CODE`"]
pub type GRAY_CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GRAY_CODE`"]
pub struct GRAY_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAY_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gray_code(&self) -> GRAY_CODE_R {
        GRAY_CODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gray_code(&mut self) -> GRAY_CODE_W {
        GRAY_CODE_W { w: self }
    }
}
