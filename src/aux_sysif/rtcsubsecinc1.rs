#[doc = "Reader of register RTCSUBSECINC1"]
pub type R = crate::R<u32, super::RTCSUBSECINC1>;
#[doc = "Writer for register RTCSUBSECINC1"]
pub type W = crate::W<u32, super::RTCSUBSECINC1>;
#[doc = "Register RTCSUBSECINC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSUBSECINC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INC23_16`"]
pub type INC23_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INC23_16`"]
pub struct INC23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> INC23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
New value for bits 23:16 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc23_16(&self) -> INC23_16_R {
        INC23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
New value for bits 23:16 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc23_16(&mut self) -> INC23_16_W {
        INC23_16_W { w: self }
    }
}
