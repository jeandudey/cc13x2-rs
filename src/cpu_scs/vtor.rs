#[doc = "Reader of register VTOR"]
pub type R = crate::R<u32, super::VTOR>;
#[doc = "Writer for register VTOR"]
pub type W = crate::W<u32, super::VTOR>;
#[doc = "Register VTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::VTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED30`"]
pub type RESERVED30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED30`"]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `TBLOFF`"]
pub type TBLOFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TBLOFF`"]
pub struct TBLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 7)) | (((value as u32) & 0x007f_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 7:29 - 29:7\\]
Bits 29 down to 7 of the vector table base offset."]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new(((self.bits >> 7) & 0x007f_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bits 7:29 - 29:7\\]
Bits 29 down to 7 of the vector table base offset."]
    #[inline(always)]
    pub fn tbloff(&mut self) -> TBLOFF_W {
        TBLOFF_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
