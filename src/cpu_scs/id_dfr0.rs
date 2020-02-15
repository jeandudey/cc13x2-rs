#[doc = "Reader of register ID_DFR0"]
pub type R = crate::R<u32, super::ID_DFR0>;
#[doc = "Writer for register ID_DFR0"]
pub type W = crate::W<u32, super::ID_DFR0>;
#[doc = "Register ID_DFR0 `reset()`'s with value 0x0010_0000"]
impl crate::ResetValue for super::ID_DFR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0000
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `MICROCONTROLLER_DEBUG_MODEL`"]
pub type MICROCONTROLLER_DEBUG_MODEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MICROCONTROLLER_DEBUG_MODEL`"]
pub struct MICROCONTROLLER_DEBUG_MODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MICROCONTROLLER_DEBUG_MODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
    #[inline(always)]
    pub fn microcontroller_debug_model(&self) -> MICROCONTROLLER_DEBUG_MODEL_R {
        MICROCONTROLLER_DEBUG_MODEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 0:19 - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
    #[inline(always)]
    pub fn microcontroller_debug_model(&mut self) -> MICROCONTROLLER_DEBUG_MODEL_W {
        MICROCONTROLLER_DEBUG_MODEL_W { w: self }
    }
    #[doc = "Bits 0:19 - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
