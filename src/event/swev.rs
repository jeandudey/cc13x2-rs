#[doc = "Reader of register SWEV"]
pub type R = crate::R<u32, super::SWEV>;
#[doc = "Writer for register SWEV"]
pub type W = crate::W<u32, super::SWEV>;
#[doc = "Register SWEV `reset()`'s with value 0"]
impl crate::ResetValue for super::SWEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `SWEV3`"]
pub type SWEV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV3`"]
pub struct SWEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 17)) | (((value as u32) & 0x7f) << 17);
        self.w
    }
}
#[doc = "Reader of field `SWEV2`"]
pub type SWEV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV2`"]
pub struct SWEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV2_W<'a> {
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
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `SWEV1`"]
pub type SWEV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV1`"]
pub struct SWEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `SWEV0`"]
pub type SWEV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV0`"]
pub struct SWEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV0_W<'a> {
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
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    pub fn swev3(&self) -> SWEV3_R {
        SWEV3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    pub fn swev3(&mut self) -> SWEV3_W {
        SWEV3_W { w: self }
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    pub fn swev2(&mut self) -> SWEV2_W {
        SWEV2_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    pub fn swev1(&mut self) -> SWEV1_W {
        SWEV1_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    pub fn swev0(&mut self) -> SWEV0_W {
        SWEV0_W { w: self }
    }
}
