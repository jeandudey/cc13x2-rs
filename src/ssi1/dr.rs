#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Writer for register DR"]
pub type W = crate::W<u32, super::DR>;
#[doc = "Register DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit/receive data The values read from this field or written to this field must be right-justified when SSI is programmed for a data size that is less than 16 bits (CR0.DSS != 0b1111). Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit/receive data The values read from this field or written to this field must be right-justified when SSI is programmed for a data size that is less than 16 bits (CR0.DSS != 0b1111). Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
