#[doc = "Reader of register SWWUTRIG"]
pub type R = crate::R<u32, super::SWWUTRIG>;
#[doc = "Writer for register SWWUTRIG"]
pub type W = crate::W<u32, super::SWWUTRIG>;
#[doc = "Register SWWUTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::SWWUTRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SW_WU3`"]
pub type SW_WU3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU3`"]
pub struct SW_WU3_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU3_W<'a> {
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
#[doc = "Reader of field `SW_WU2`"]
pub type SW_WU2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU2`"]
pub struct SW_WU2_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU2_W<'a> {
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
#[doc = "Reader of field `SW_WU1`"]
pub type SW_WU1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU1`"]
pub struct SW_WU1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU1_W<'a> {
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
#[doc = "Reader of field `SW_WU0`"]
pub type SW_WU0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU0`"]
pub struct SW_WU0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU0_W<'a> {
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
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu3(&self) -> SW_WU3_R {
        SW_WU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SW_WU2_R {
        SW_WU2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SW_WU1_R {
        SW_WU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SW_WU0_R {
        SW_WU0_R::new((self.bits & 0x01) != 0)
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
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu3(&mut self) -> SW_WU3_W {
        SW_WU3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu2(&mut self) -> SW_WU2_W {
        SW_WU2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu1(&mut self) -> SW_WU1_W {
        SW_WU1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu0(&mut self) -> SW_WU0_W {
        SW_WU0_W { w: self }
    }
}
