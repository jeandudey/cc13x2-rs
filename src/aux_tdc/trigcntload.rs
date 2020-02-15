#[doc = "Reader of register TRIGCNTLOAD"]
pub type R = crate::R<u32, super::TRIGCNTLOAD>;
#[doc = "Writer for register TRIGCNTLOAD"]
pub type W = crate::W<u32, super::TRIGCNTLOAD>;
#[doc = "Register TRIGCNTLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIGCNTLOAD {
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
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT`"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
