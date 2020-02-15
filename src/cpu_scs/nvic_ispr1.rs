#[doc = "Reader of register NVIC_ISPR1"]
pub type R = crate::R<u32, super::NVIC_ISPR1>;
#[doc = "Writer for register NVIC_ISPR1"]
pub type W = crate::W<u32, super::NVIC_ISPR1>;
#[doc = "Register NVIC_ISPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISPR1 {
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
#[doc = "Reader of field `SETPEND37`"]
pub type SETPEND37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND37`"]
pub struct SETPEND37_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND37_W<'a> {
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
#[doc = "Reader of field `SETPEND36`"]
pub type SETPEND36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND36`"]
pub struct SETPEND36_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND36_W<'a> {
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
#[doc = "Reader of field `SETPEND35`"]
pub type SETPEND35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND35`"]
pub struct SETPEND35_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND35_W<'a> {
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
#[doc = "Reader of field `SETPEND34`"]
pub type SETPEND34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND34`"]
pub struct SETPEND34_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND34_W<'a> {
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
#[doc = "Reader of field `SETPEND33`"]
pub type SETPEND33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND33`"]
pub struct SETPEND33_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND33_W<'a> {
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
#[doc = "Reader of field `SETPEND32`"]
pub type SETPEND32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND32`"]
pub struct SETPEND32_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND32_W<'a> {
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
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend37(&self) -> SETPEND37_R {
        SETPEND37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend36(&self) -> SETPEND36_R {
        SETPEND36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend35(&self) -> SETPEND35_R {
        SETPEND35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend34(&self) -> SETPEND34_R {
        SETPEND34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend33(&self) -> SETPEND33_R {
        SETPEND33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend32(&self) -> SETPEND32_R {
        SETPEND32_R::new((self.bits & 0x01) != 0)
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
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend37(&mut self) -> SETPEND37_W {
        SETPEND37_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend36(&mut self) -> SETPEND36_W {
        SETPEND36_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend35(&mut self) -> SETPEND35_W {
        SETPEND35_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend34(&mut self) -> SETPEND34_W {
        SETPEND34_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend33(&mut self) -> SETPEND33_W {
        SETPEND33_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend32(&mut self) -> SETPEND32_W {
        SETPEND32_W { w: self }
    }
}
