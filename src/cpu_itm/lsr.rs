#[doc = "Reader of register LSR"]
pub type R = crate::R<u32, super::LSR>;
#[doc = "Writer for register LSR"]
pub type W = crate::W<u32, super::LSR>;
#[doc = "Register LSR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::LSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
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
#[doc = "Reader of field `BYTEACC`"]
pub type BYTEACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTEACC`"]
pub struct BYTEACC_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEACC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ACCESS`"]
pub type ACCESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCESS`"]
pub struct ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCESS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PRESENT`"]
pub type PRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESENT`"]
pub struct PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESENT_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
    #[inline(always)]
    pub fn byteacc(&self) -> BYTEACC_R {
        BYTEACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock mechanism exists for this component."]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Reads 0 which means 8-bit lock access is not be implemented."]
    #[inline(always)]
    pub fn byteacc(&mut self) -> BYTEACC_W {
        BYTEACC_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline(always)]
    pub fn access(&mut self) -> ACCESS_W {
        ACCESS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Indicates that a lock mechanism exists for this component."]
    #[inline(always)]
    pub fn present(&mut self) -> PRESENT_W {
        PRESENT_W { w: self }
    }
}
