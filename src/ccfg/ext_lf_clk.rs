#[doc = "Reader of register EXT_LF_CLK"]
pub type R = crate::R<u32, super::EXT_LF_CLK>;
#[doc = "Writer for register EXT_LF_CLK"]
pub type W = crate::W<u32, super::EXT_LF_CLK>;
#[doc = "Register EXT_LF_CLK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::EXT_LF_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `DIO`"]
pub type DIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIO`"]
pub struct DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTC_INCREMENT`"]
pub type RTC_INCREMENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC_INCREMENT`"]
pub struct RTC_INCREMENT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_INCREMENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline(always)]
    pub fn dio(&self) -> DIO_R {
        DIO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline(always)]
    pub fn rtc_increment(&self) -> RTC_INCREMENT_R {
        RTC_INCREMENT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline(always)]
    pub fn dio(&mut self) -> DIO_W {
        DIO_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline(always)]
    pub fn rtc_increment(&mut self) -> RTC_INCREMENT_W {
        RTC_INCREMENT_W { w: self }
    }
}
