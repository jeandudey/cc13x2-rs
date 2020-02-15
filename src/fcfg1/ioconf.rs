#[doc = "Reader of register IOCONF"]
pub type R = crate::R<u32, super::IOCONF>;
#[doc = "Writer for register IOCONF"]
pub type W = crate::W<u32, super::IOCONF>;
#[doc = "Register IOCONF `reset()`'s with value 0xffff_ff00"]
impl crate::ResetValue for super::IOCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ff00
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `GPIO_CNT`"]
pub type GPIO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_CNT`"]
pub struct GPIO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\]
Number of available DIOs."]
    #[inline(always)]
    pub fn gpio_cnt(&self) -> GPIO_CNT_R {
        GPIO_CNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
Number of available DIOs."]
    #[inline(always)]
    pub fn gpio_cnt(&mut self) -> GPIO_CNT_W {
        GPIO_CNT_W { w: self }
    }
}
