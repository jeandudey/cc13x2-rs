#[doc = "Reader of register RTCSUBSEC"]
pub type R = crate::R<u32, super::RTCSUBSEC>;
#[doc = "Writer for register RTCSUBSEC"]
pub type W = crate::W<u32, super::RTCSUBSEC>;
#[doc = "Register RTCSUBSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSUBSEC {
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
#[doc = "Reader of field `SUBSEC`"]
pub type SUBSEC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUBSEC`"]
pub struct SUBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBSEC_W<'a> {
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
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0xffff) as u16)
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
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn subsec(&mut self) -> SUBSEC_W {
        SUBSEC_W { w: self }
    }
}
