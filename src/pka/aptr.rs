#[doc = "Reader of register APTR"]
pub type R = crate::R<u32, super::APTR>;
#[doc = "Writer for register APTR"]
pub type W = crate::W<u32, super::APTR>;
#[doc = "Register APTR `reset()`'s with value 0"]
impl crate::ResetValue for super::APTR {
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
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
#[doc = "Reader of field `APTR`"]
pub type APTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APTR`"]
pub struct APTR_W<'a> {
    w: &'a mut W,
}
impl<'a> APTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:31 - 31:11\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 0:10 - 10:0\\]
This register specifies the location of vector A within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn aptr(&self) -> APTR_R {
        APTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:31 - 31:11\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bits 0:10 - 10:0\\]
This register specifies the location of vector A within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn aptr(&mut self) -> APTR_W {
        APTR_W { w: self }
    }
}
