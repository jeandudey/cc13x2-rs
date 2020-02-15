#[doc = "Reader of register SHPR3"]
pub type R = crate::R<u32, super::SHPR3>;
#[doc = "Writer for register SHPR3"]
pub type W = crate::W<u32, super::SHPR3>;
#[doc = "Register SHPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_15`"]
pub type PRI_15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_15`"]
pub struct PRI_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRI_14`"]
pub type PRI_14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_14`"]
pub struct PRI_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_12`"]
pub type PRI_12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_12`"]
pub struct PRI_12_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 15. SysTick exception"]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 14. Pend SV"]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 12. Debug Monitor"]
    #[inline(always)]
    pub fn pri_12(&self) -> PRI_12_R {
        PRI_12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of system handler 15. SysTick exception"]
    #[inline(always)]
    pub fn pri_15(&mut self) -> PRI_15_W {
        PRI_15_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of system handler 14. Pend SV"]
    #[inline(always)]
    pub fn pri_14(&mut self) -> PRI_14_W {
        PRI_14_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of system handler 12. Debug Monitor"]
    #[inline(always)]
    pub fn pri_12(&mut self) -> PRI_12_W {
        PRI_12_W { w: self }
    }
}
