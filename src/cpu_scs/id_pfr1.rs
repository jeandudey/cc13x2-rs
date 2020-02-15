#[doc = "Reader of register ID_PFR1"]
pub type R = crate::R<u32, super::ID_PFR1>;
#[doc = "Writer for register ID_PFR1"]
pub type W = crate::W<u32, super::ID_PFR1>;
#[doc = "Register ID_PFR1 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::ID_PFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
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
#[doc = "Reader of field `MICROCONTROLLER_PROGRAMMERS_MODEL`"]
pub type MICROCONTROLLER_PROGRAMMERS_MODEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MICROCONTROLLER_PROGRAMMERS_MODEL`"]
pub struct MICROCONTROLLER_PROGRAMMERS_MODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MICROCONTROLLER_PROGRAMMERS_MODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_R {
        MICROCONTROLLER_PROGRAMMERS_MODEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&mut self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_W {
        MICROCONTROLLER_PROGRAMMERS_MODEL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
