#[doc = "Reader of register BLENGTH"]
pub type R = crate::R<u32, super::BLENGTH>;
#[doc = "Writer for register BLENGTH"]
pub type W = crate::W<u32, super::BLENGTH>;
#[doc = "Register BLENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::BLENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `BLENGTH`"]
pub type BLENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLENGTH`"]
pub struct BLENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BLENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bits 0:8 - 8:0\\]
This register specifies the length (in 32-bit words) of Vector B."]
    #[inline(always)]
    pub fn blength(&self) -> BLENGTH_R {
        BLENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\]
This register specifies the length (in 32-bit words) of Vector B."]
    #[inline(always)]
    pub fn blength(&mut self) -> BLENGTH_W {
        BLENGTH_W { w: self }
    }
}
