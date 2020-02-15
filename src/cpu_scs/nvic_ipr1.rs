#[doc = "Reader of register NVIC_IPR1"]
pub type R = crate::R<u32, super::NVIC_IPR1>;
#[doc = "Writer for register NVIC_IPR1"]
pub type W = crate::W<u32, super::NVIC_IPR1>;
#[doc = "Register NVIC_IPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_7`"]
pub type PRI_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_7`"]
pub struct PRI_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRI_6`"]
pub type PRI_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_6`"]
pub struct PRI_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_5`"]
pub type PRI_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_5`"]
pub struct PRI_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_4`"]
pub type PRI_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_4`"]
pub struct PRI_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn pri_7(&self) -> PRI_7_R {
        PRI_7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn pri_6(&self) -> PRI_6_R {
        PRI_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn pri_5(&self) -> PRI_5_R {
        PRI_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn pri_4(&self) -> PRI_4_R {
        PRI_4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn pri_7(&mut self) -> PRI_7_W {
        PRI_7_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn pri_6(&mut self) -> PRI_6_W {
        PRI_6_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn pri_5(&mut self) -> PRI_5_W {
        PRI_5_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn pri_4(&mut self) -> PRI_4_W {
        PRI_4_W { w: self }
    }
}
