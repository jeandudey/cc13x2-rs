#[doc = "Reader of register NVIC_IPR6"]
pub type R = crate::R<u32, super::NVIC_IPR6>;
#[doc = "Writer for register NVIC_IPR6"]
pub type W = crate::W<u32, super::NVIC_IPR6>;
#[doc = "Register NVIC_IPR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_27`"]
pub type PRI_27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_27`"]
pub struct PRI_27_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRI_26`"]
pub type PRI_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_26`"]
pub struct PRI_26_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_25`"]
pub type PRI_25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_25`"]
pub struct PRI_25_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_24`"]
pub type PRI_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_24`"]
pub struct PRI_24_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn pri_27(&self) -> PRI_27_R {
        PRI_27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn pri_26(&self) -> PRI_26_R {
        PRI_26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn pri_25(&self) -> PRI_25_R {
        PRI_25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn pri_24(&self) -> PRI_24_R {
        PRI_24_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn pri_27(&mut self) -> PRI_27_W {
        PRI_27_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn pri_26(&mut self) -> PRI_26_W {
        PRI_26_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn pri_25(&mut self) -> PRI_25_W {
        PRI_25_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn pri_24(&mut self) -> PRI_24_W {
        PRI_24_W { w: self }
    }
}
