#[doc = "Reader of register SOAR"]
pub type R = crate::R<u32, super::SOAR>;
#[doc = "Writer for register SOAR"]
pub type W = crate::W<u32, super::SOAR>;
#[doc = "Register SOAR `reset()`'s with value 0"]
impl crate::ResetValue for super::SOAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `OAR`"]
pub type OAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OAR`"]
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
}
