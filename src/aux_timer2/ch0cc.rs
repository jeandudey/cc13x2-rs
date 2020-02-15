#[doc = "Reader of register CH0CC"]
pub type R = crate::R<u32, super::CH0CC>;
#[doc = "Writer for register CH0CC"]
pub type W = crate::W<u32, super::CH0CC>;
#[doc = "Register CH0CC `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0CC {
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
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH0EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH0EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
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
Capture Compare value. 16-bit user defined compare value or channel-updated capture value. Compare mode: VALUE is compared against CNTR.VALUE and an event is generated as specified by CH0EVCFG.CCACT when these are equal. Capture mode: The current counter value is stored in VALUE when a capture event occurs. CH0EVCFG.CCACT determines if VALUE is a signal period or a regular capture value."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
