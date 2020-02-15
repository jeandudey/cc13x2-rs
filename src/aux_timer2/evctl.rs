#[doc = "Reader of register EVCTL"]
pub type R = crate::R<u32, super::EVCTL>;
#[doc = "Writer for register EVCTL"]
pub type W = crate::W<u32, super::EVCTL>;
#[doc = "Register EVCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EV3_SET`"]
pub type EV3_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV3_SET`"]
pub struct EV3_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV3_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EV3_CLR`"]
pub type EV3_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV3_CLR`"]
pub struct EV3_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV3_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EV2_SET`"]
pub type EV2_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV2_SET`"]
pub struct EV2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EV2_CLR`"]
pub type EV2_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV2_CLR`"]
pub struct EV2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EV1_SET`"]
pub type EV1_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV1_SET`"]
pub struct EV1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_SET_W<'a> {
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
#[doc = "Reader of field `EV1_CLR`"]
pub type EV1_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV1_CLR`"]
pub struct EV1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_CLR_W<'a> {
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
#[doc = "Reader of field `EV0_SET`"]
pub type EV0_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV0_SET`"]
pub struct EV0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_SET_W<'a> {
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
#[doc = "Reader of field `EV0_CLR`"]
pub type EV0_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV0_CLR`"]
pub struct EV0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_CLR_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
Set event 3. Write 1 to set event 3."]
    #[inline(always)]
    pub fn ev3_set(&self) -> EV3_SET_R {
        EV3_SET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
    #[inline(always)]
    pub fn ev3_clr(&self) -> EV3_CLR_R {
        EV3_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set event 2. Write 1 to set event 2."]
    #[inline(always)]
    pub fn ev2_set(&self) -> EV2_SET_R {
        EV2_SET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
    #[inline(always)]
    pub fn ev2_clr(&self) -> EV2_CLR_R {
        EV2_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Set event 1. Write 1 to set event 1."]
    #[inline(always)]
    pub fn ev1_set(&self) -> EV1_SET_R {
        EV1_SET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
    #[inline(always)]
    pub fn ev1_clr(&self) -> EV1_CLR_R {
        EV1_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set event 0. Write 1 to set event 0."]
    #[inline(always)]
    pub fn ev0_set(&self) -> EV0_SET_R {
        EV0_SET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
    #[inline(always)]
    pub fn ev0_clr(&self) -> EV0_CLR_R {
        EV0_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Set event 3. Write 1 to set event 3."]
    #[inline(always)]
    pub fn ev3_set(&mut self) -> EV3_SET_W {
        EV3_SET_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
    #[inline(always)]
    pub fn ev3_clr(&mut self) -> EV3_CLR_W {
        EV3_CLR_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Set event 2. Write 1 to set event 2."]
    #[inline(always)]
    pub fn ev2_set(&mut self) -> EV2_SET_W {
        EV2_SET_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
    #[inline(always)]
    pub fn ev2_clr(&mut self) -> EV2_CLR_W {
        EV2_CLR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Set event 1. Write 1 to set event 1."]
    #[inline(always)]
    pub fn ev1_set(&mut self) -> EV1_SET_W {
        EV1_SET_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
    #[inline(always)]
    pub fn ev1_clr(&mut self) -> EV1_CLR_W {
        EV1_CLR_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Set event 0. Write 1 to set event 0."]
    #[inline(always)]
    pub fn ev0_set(&mut self) -> EV0_SET_W {
        EV0_SET_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
    #[inline(always)]
    pub fn ev0_clr(&mut self) -> EV0_CLR_W {
        EV0_CLR_W { w: self }
    }
}
