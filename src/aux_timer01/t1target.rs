#[doc = "Reader of register T1TARGET"]
pub type R = crate::R<u32, super::T1TARGET>;
#[doc = "Writer for register T1TARGET"]
pub type W = crate::W<u32, super::T1TARGET>;
#[doc = "Register T1TARGET `reset()`'s with value 0"]
impl crate::ResetValue for super::T1TARGET {
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
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
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
Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
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
Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 peripheral clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 peripheral clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 peripheral clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
