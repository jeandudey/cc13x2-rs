#[doc = "Reader of register NVIC_IPR8"]
pub type R = crate::R<u32, super::NVIC_IPR8>;
#[doc = "Writer for register NVIC_IPR8"]
pub type W = crate::W<u32, super::NVIC_IPR8>;
#[doc = "Register NVIC_IPR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_35`"]
pub type PRI_35_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_35`"]
pub struct PRI_35_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRI_34`"]
pub type PRI_34_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_34`"]
pub struct PRI_34_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_33`"]
pub type PRI_33_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_33`"]
pub struct PRI_33_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_32`"]
pub type PRI_32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_32`"]
pub struct PRI_32_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn pri_35(&self) -> PRI_35_R {
        PRI_35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn pri_34(&self) -> PRI_34_R {
        PRI_34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn pri_33(&self) -> PRI_33_R {
        PRI_33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn pri_32(&self) -> PRI_32_R {
        PRI_32_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn pri_35(&mut self) -> PRI_35_W {
        PRI_35_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn pri_34(&mut self) -> PRI_34_W {
        PRI_34_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn pri_33(&mut self) -> PRI_33_W {
        PRI_33_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn pri_32(&mut self) -> PRI_32_W {
        PRI_32_W { w: self }
    }
}
