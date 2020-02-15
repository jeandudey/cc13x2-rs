#[doc = "Reader of register SUBSECINC"]
pub type R = crate::R<u32, super::SUBSECINC>;
#[doc = "Writer for register SUBSECINC"]
pub type W = crate::W<u32, super::SUBSECINC>;
#[doc = "Register SUBSECINC `reset()`'s with value 0x0080_0000"]
impl crate::ResetValue for super::SUBSECINC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0000
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `VALUEINC`"]
pub type VALUEINC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUEINC`"]
pub struct VALUEINC_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUEINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_SYSIF:RTCSUBSECINC0 , AUX_SYSIF:RTCSUBSECINC1 and AUX_SYSIF:RTCSUBSECINCCTL"]
    #[inline(always)]
    pub fn valueinc(&self) -> VALUEINC_R {
        VALUEINC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_SYSIF:RTCSUBSECINC0 , AUX_SYSIF:RTCSUBSECINC1 and AUX_SYSIF:RTCSUBSECINCCTL"]
    #[inline(always)]
    pub fn valueinc(&mut self) -> VALUEINC_W {
        VALUEINC_W { w: self }
    }
}
