#[doc = "Reader of register NVIC_IPR4"]
pub type R = crate::R<u32, super::NVIC_IPR4>;
#[doc = "Writer for register NVIC_IPR4"]
pub type W = crate::W<u32, super::NVIC_IPR4>;
#[doc = "Register NVIC_IPR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_19`"]
pub type PRI_19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_19`"]
pub struct PRI_19_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRI_18`"]
pub type PRI_18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_18`"]
pub struct PRI_18_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_17`"]
pub type PRI_17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_17`"]
pub struct PRI_17_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_16`"]
pub type PRI_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_16`"]
pub struct PRI_16_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn pri_19(&self) -> PRI_19_R {
        PRI_19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn pri_18(&self) -> PRI_18_R {
        PRI_18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn pri_17(&self) -> PRI_17_R {
        PRI_17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn pri_16(&self) -> PRI_16_R {
        PRI_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn pri_19(&mut self) -> PRI_19_W {
        PRI_19_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn pri_18(&mut self) -> PRI_18_W {
        PRI_18_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn pri_17(&mut self) -> PRI_17_W {
        PRI_17_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn pri_16(&mut self) -> PRI_16_W {
        PRI_16_W { w: self }
    }
}
