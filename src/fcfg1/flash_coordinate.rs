#[doc = "Reader of register FLASH_COORDINATE"]
pub type R = crate::R<u32, super::FLASH_COORDINATE>;
#[doc = "Writer for register FLASH_COORDINATE"]
pub type W = crate::W<u32, super::FLASH_COORDINATE>;
#[doc = "Register FLASH_COORDINATE `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_COORDINATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XCOORDINATE`"]
pub type XCOORDINATE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XCOORDINATE`"]
pub struct XCOORDINATE_W<'a> {
    w: &'a mut W,
}
impl<'a> XCOORDINATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `YCOORDINATE`"]
pub type YCOORDINATE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `YCOORDINATE`"]
pub struct YCOORDINATE_W<'a> {
    w: &'a mut W,
}
impl<'a> YCOORDINATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn xcoordinate(&self) -> XCOORDINATE_R {
        XCOORDINATE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn ycoordinate(&self) -> YCOORDINATE_R {
        YCOORDINATE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn xcoordinate(&mut self) -> XCOORDINATE_W {
        XCOORDINATE_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn ycoordinate(&mut self) -> YCOORDINATE_W {
        YCOORDINATE_W { w: self }
    }
}
