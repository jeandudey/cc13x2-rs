#[doc = "Reader of register RFCBITS"]
pub type R = crate::R<u32, super::RFCBITS>;
#[doc = "Writer for register RFCBITS"]
pub type W = crate::W<u32, super::RFCBITS>;
#[doc = "Register RFCBITS `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCBITS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `READ`"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
}
