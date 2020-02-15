#[doc = "Reader of register HWOPT"]
pub type R = crate::R<u32, super::HWOPT>;
#[doc = "Writer for register HWOPT"]
pub type W = crate::W<u32, super::HWOPT>;
#[doc = "Register HWOPT `reset()`'s with value 0x0600"]
impl crate::ResetValue for super::HWOPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `NR_OF_FROS`"]
pub type NR_OF_FROS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NR_OF_FROS`"]
pub struct NR_OF_FROS_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_OF_FROS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
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
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub fn nr_of_fros(&self) -> NR_OF_FROS_R {
        NR_OF_FROS_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub fn nr_of_fros(&mut self) -> NR_OF_FROS_W {
        NR_OF_FROS_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
