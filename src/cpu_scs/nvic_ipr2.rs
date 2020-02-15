#[doc = "Reader of register NVIC_IPR2"]
pub type R = crate::R<u32, super::NVIC_IPR2>;
#[doc = "Writer for register NVIC_IPR2"]
pub type W = crate::W<u32, super::NVIC_IPR2>;
#[doc = "Register NVIC_IPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_11`"]
pub type PRI_11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_11`"]
pub struct PRI_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRI_10`"]
pub type PRI_10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_10`"]
pub struct PRI_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_9`"]
pub type PRI_9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_9`"]
pub struct PRI_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_8`"]
pub type PRI_8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_8`"]
pub struct PRI_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn pri_11(&self) -> PRI_11_R {
        PRI_11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn pri_10(&self) -> PRI_10_R {
        PRI_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn pri_9(&self) -> PRI_9_R {
        PRI_9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn pri_8(&self) -> PRI_8_R {
        PRI_8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn pri_11(&mut self) -> PRI_11_W {
        PRI_11_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn pri_10(&mut self) -> PRI_10_W {
        PRI_10_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn pri_9(&mut self) -> PRI_9_W {
        PRI_9_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn pri_8(&mut self) -> PRI_8_W {
        PRI_8_W { w: self }
    }
}
