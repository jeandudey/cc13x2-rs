#[doc = "Reader of register RESULT"]
pub type R = crate::R<u32, super::RESULT>;
#[doc = "Writer for register RESULT"]
pub type W = crate::W<u32, super::RESULT>;
#[doc = "Register RESULT `reset()`'s with value 0x02"]
impl crate::ResetValue for super::RESULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 0:24 - 24:0\\]
TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bits 0:24 - 24:0\\]
TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
