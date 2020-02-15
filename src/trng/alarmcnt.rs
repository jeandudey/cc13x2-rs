#[doc = "Reader of register ALARMCNT"]
pub type R = crate::R<u32, super::ALARMCNT>;
#[doc = "Writer for register ALARMCNT"]
pub type W = crate::W<u32, super::ALARMCNT>;
#[doc = "Register ALARMCNT `reset()`'s with value 0xff"]
impl crate::ResetValue for super::ALARMCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `RESERVED30`"]
pub type RESERVED30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED30`"]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SHUTDOWN_CNT`"]
pub type SHUTDOWN_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHUTDOWN_CNT`"]
pub struct SHUTDOWN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED21`"]
pub type RESERVED21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED21`"]
pub struct RESERVED21_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `SHUTDOWN_THR`"]
pub type SHUTDOWN_THR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHUTDOWN_THR`"]
pub struct SHUTDOWN_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALARM_THR`"]
pub type ALARM_THR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALARM_THR`"]
pub struct ALARM_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub fn shutdown_cnt(&self) -> SHUTDOWN_CNT_R {
        SHUTDOWN_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub fn shutdown_thr(&self) -> SHUTDOWN_THR_R {
        SHUTDOWN_THR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub fn alarm_thr(&self) -> ALARM_THR_R {
        ALARM_THR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub fn shutdown_cnt(&mut self) -> SHUTDOWN_CNT_W {
        SHUTDOWN_CNT_W { w: self }
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&mut self) -> RESERVED21_W {
        RESERVED21_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub fn shutdown_thr(&mut self) -> SHUTDOWN_THR_W {
        SHUTDOWN_THR_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub fn alarm_thr(&mut self) -> ALARM_THR_W {
        ALARM_THR_W { w: self }
    }
}
