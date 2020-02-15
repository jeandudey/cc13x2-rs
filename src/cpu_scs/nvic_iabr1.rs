#[doc = "Reader of register NVIC_IABR1"]
pub type R = crate::R<u32, super::NVIC_IABR1>;
#[doc = "Writer for register NVIC_IABR1"]
pub type W = crate::W<u32, super::NVIC_IABR1>;
#[doc = "Register NVIC_IABR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IABR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `ACTIVE37`"]
pub type ACTIVE37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE37`"]
pub struct ACTIVE37_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE37_W<'a> {
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
#[doc = "Reader of field `ACTIVE36`"]
pub type ACTIVE36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE36`"]
pub struct ACTIVE36_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE36_W<'a> {
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
#[doc = "Reader of field `ACTIVE35`"]
pub type ACTIVE35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE35`"]
pub struct ACTIVE35_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE35_W<'a> {
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
#[doc = "Reader of field `ACTIVE34`"]
pub type ACTIVE34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE34`"]
pub struct ACTIVE34_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE34_W<'a> {
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
#[doc = "Reader of field `ACTIVE33`"]
pub type ACTIVE33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE33`"]
pub struct ACTIVE33_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE33_W<'a> {
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
#[doc = "Reader of field `ACTIVE32`"]
pub type ACTIVE32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE32`"]
pub struct ACTIVE32_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE32_W<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 37 is not active. Reading 1 from this bit implies that the interrupt line 37 is active (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    pub fn active37(&self) -> ACTIVE37_R {
        ACTIVE37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 36 is not active. Reading 1 from this bit implies that the interrupt line 36 is active (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    pub fn active36(&self) -> ACTIVE36_R {
        ACTIVE36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 35 is not active. Reading 1 from this bit implies that the interrupt line 35 is active (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn active35(&self) -> ACTIVE35_R {
        ACTIVE35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 34 is not active. Reading 1 from this bit implies that the interrupt line 34 is active (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn active34(&self) -> ACTIVE34_R {
        ACTIVE34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn active33(&self) -> ACTIVE33_R {
        ACTIVE33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn active32(&self) -> ACTIVE32_R {
        ACTIVE32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 37 is not active. Reading 1 from this bit implies that the interrupt line 37 is active (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    pub fn active37(&mut self) -> ACTIVE37_W {
        ACTIVE37_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 36 is not active. Reading 1 from this bit implies that the interrupt line 36 is active (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    pub fn active36(&mut self) -> ACTIVE36_W {
        ACTIVE36_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 35 is not active. Reading 1 from this bit implies that the interrupt line 35 is active (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn active35(&mut self) -> ACTIVE35_W {
        ACTIVE35_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 34 is not active. Reading 1 from this bit implies that the interrupt line 34 is active (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn active34(&mut self) -> ACTIVE34_W {
        ACTIVE34_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn active33(&mut self) -> ACTIVE33_W {
        ACTIVE33_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn active32(&mut self) -> ACTIVE32_W {
        ACTIVE32_W { w: self }
    }
}
