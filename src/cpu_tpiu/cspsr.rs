#[doc = "Reader of register CSPSR"]
pub type R = crate::R<u32, super::CSPSR>;
#[doc = "Writer for register CSPSR"]
pub type W = crate::W<u32, super::CSPSR>;
#[doc = "Register CSPSR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CSPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `FOUR`"]
pub type FOUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOUR`"]
pub struct FOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOUR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `THREE`"]
pub type THREE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THREE`"]
pub struct THREE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TWO`"]
pub type TWO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWO`"]
pub struct TWO_W<'a> {
    w: &'a mut W,
}
impl<'a> TWO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ONE`"]
pub type ONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONE`"]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn four(&self) -> FOUR_R {
        FOUR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn three(&self) -> THREE_R {
        THREE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn two(&self) -> TWO_R {
        TWO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn four(&mut self) -> FOUR_W {
        FOUR_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn three(&mut self) -> THREE_W {
        THREE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn two(&mut self) -> TWO_W {
        TWO_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
}
