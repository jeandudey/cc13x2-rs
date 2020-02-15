#[doc = "Reader of register PRECNTR"]
pub type R = crate::R<u32, super::PRECNTR>;
#[doc = "Writer for register PRECNTR"]
pub type W = crate::W<u32, super::PRECNTR>;
#[doc = "Register PRECNTR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRECNTR {
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
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. The read value gets 1 LSB uncertainty if the event source level rises when you capture the prescaler counter. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
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
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. The read value gets 1 LSB uncertainty if the event source level rises when you capture the prescaler counter. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
