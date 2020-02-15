#[doc = "Reader of register MISC_CONF_1"]
pub type R = crate::R<u32, super::MISC_CONF_1>;
#[doc = "Writer for register MISC_CONF_1"]
pub type W = crate::W<u32, super::MISC_CONF_1>;
#[doc = "Register MISC_CONF_1 `reset()`'s with value 0xffff_ff00"]
impl crate::ResetValue for super::MISC_CONF_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ff00
    }
}
#[doc = "Reader of field `DEVICE_MINOR_REV`"]
pub type DEVICE_MINOR_REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVICE_MINOR_REV`"]
pub struct DEVICE_MINOR_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_MINOR_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline(always)]
    pub fn device_minor_rev(&self) -> DEVICE_MINOR_REV_R {
        DEVICE_MINOR_REV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline(always)]
    pub fn device_minor_rev(&mut self) -> DEVICE_MINOR_REV_W {
        DEVICE_MINOR_REV_W { w: self }
    }
}
