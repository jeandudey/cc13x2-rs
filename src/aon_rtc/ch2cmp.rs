#[doc = "Reader of register CH2CMP"]
pub type R = crate::R<u32, super::CH2CMP>;
#[doc = "Writer for register CH2CMP"]
pub type W = crate::W<u32, super::CH2CMP>;
#[doc = "Register CH2CMP `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2CMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RTC Channel 2 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to one SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RTC Channel 2 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to one SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
