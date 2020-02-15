#[doc = "Reader of register NVIC_IPR9"]
pub type R = crate::R<u32, super::NVIC_IPR9>;
#[doc = "Writer for register NVIC_IPR9"]
pub type W = crate::W<u32, super::NVIC_IPR9>;
#[doc = "Register NVIC_IPR9 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_37`"]
pub type PRI_37_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_37`"]
pub struct PRI_37_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_36`"]
pub type PRI_36_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_36`"]
pub struct PRI_36_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 37 (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    pub fn pri_37(&self) -> PRI_37_R {
        PRI_37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 36 (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    pub fn pri_36(&self) -> PRI_36_R {
        PRI_36_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 37 (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    pub fn pri_37(&mut self) -> PRI_37_W {
        PRI_37_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 36 (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    pub fn pri_36(&mut self) -> PRI_36_W {
        PRI_36_W { w: self }
    }
}
