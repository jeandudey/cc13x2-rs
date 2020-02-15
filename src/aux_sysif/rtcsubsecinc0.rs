#[doc = "Reader of register RTCSUBSECINC0"]
pub type R = crate::R<u32, super::RTCSUBSECINC0>;
#[doc = "Writer for register RTCSUBSECINC0"]
pub type W = crate::W<u32, super::RTCSUBSECINC0>;
#[doc = "Register RTCSUBSECINC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSUBSECINC0 {
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
#[doc = "Reader of field `INC15_0`"]
pub type INC15_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INC15_0`"]
pub struct INC15_0_W<'a> {
    w: &'a mut W,
}
impl<'a> INC15_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
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
    #[doc = "Bits 0:15 - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc15_0(&self) -> INC15_0_R {
        INC15_0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
New value for bits 15:0 in AON_RTC:SUBSECINC."]
    #[inline(always)]
    pub fn inc15_0(&mut self) -> INC15_0_W {
        INC15_0_W { w: self }
    }
}
