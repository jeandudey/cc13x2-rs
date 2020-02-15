#[doc = "Reader of register NVIC_IPR3"]
pub type R = crate::R<u32, super::NVIC_IPR3>;
#[doc = "Writer for register NVIC_IPR3"]
pub type W = crate::W<u32, super::NVIC_IPR3>;
#[doc = "Register NVIC_IPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR3 {
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
#[doc = "Reader of field `PRI_13`"]
pub type PRI_13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_13`"]
pub struct PRI_13_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_13_W<'a> {
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
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn pri_13(&self) -> PRI_13_R {
        PRI_13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn pri_12(&self) -> PRI_12_R {
        PRI_12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn pri_15(&mut self) -> PRI_15_W {
        PRI_15_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn pri_14(&mut self) -> PRI_14_W {
        PRI_14_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn pri_13(&mut self) -> PRI_13_W {
        PRI_13_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn pri_12(&mut self) -> PRI_12_W {
        PRI_12_W { w: self }
    }
}
