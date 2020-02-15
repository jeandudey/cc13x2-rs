#[doc = "Reader of register STMPOUTTRIG"]
pub type R = crate::R<u32, super::STMPOUTTRIG>;
#[doc = "Writer for register STMPOUTTRIG"]
pub type W = crate::W<u32, super::STMPOUTTRIG>;
#[doc = "Register STMPOUTTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPOUTTRIG {
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
#[doc = "Reader of field `OUT_START_WCNT`"]
pub type OUT_START_WCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OUT_START_WCNT`"]
pub struct OUT_START_WCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_START_WCNT_W<'a> {
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
Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub fn out_start_wcnt(&self) -> OUT_START_WCNT_R {
        OUT_START_WCNT_R::new((self.bits & 0xffff) as u16)
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
Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub fn out_start_wcnt(&mut self) -> OUT_START_WCNT_W {
        OUT_START_WCNT_W { w: self }
    }
}
