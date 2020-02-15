#[doc = "Reader of register ADC1"]
pub type R = crate::R<u8, super::ADC1>;
#[doc = "Writer for register ADC1"]
pub type W = crate::W<u8, super::ADC1>;
#[doc = "Register ADC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u8) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `SCALE_DIS`"]
pub type SCALE_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCALE_DIS`"]
pub struct SCALE_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scale_dis(&self) -> SCALE_DIS_R {
        SCALE_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scale_dis(&mut self) -> SCALE_DIS_W {
        SCALE_DIS_W { w: self }
    }
}
