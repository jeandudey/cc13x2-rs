#[doc = "Reader of register NVIC_ISER1"]
pub type R = crate::R<u32, super::NVIC_ISER1>;
#[doc = "Writer for register NVIC_ISER1"]
pub type W = crate::W<u32, super::NVIC_ISER1>;
#[doc = "Register NVIC_ISER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISER1 {
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
#[doc = "Reader of field `SETENA37`"]
pub type SETENA37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA37`"]
pub struct SETENA37_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA37_W<'a> {
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
#[doc = "Reader of field `SETENA36`"]
pub type SETENA36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA36`"]
pub struct SETENA36_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA36_W<'a> {
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
#[doc = "Reader of field `SETENA35`"]
pub type SETENA35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA35`"]
pub struct SETENA35_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA35_W<'a> {
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
#[doc = "Reader of field `SETENA34`"]
pub type SETENA34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA34`"]
pub struct SETENA34_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA34_W<'a> {
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
#[doc = "Reader of field `SETENA33`"]
pub type SETENA33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA33`"]
pub struct SETENA33_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA33_W<'a> {
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
#[doc = "Reader of field `SETENA32`"]
pub type SETENA32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA32`"]
pub struct SETENA32_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA32_W<'a> {
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
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena37(&self) -> SETENA37_R {
        SETENA37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena36(&self) -> SETENA36_R {
        SETENA36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena35(&self) -> SETENA35_R {
        SETENA35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena34(&self) -> SETENA34_R {
        SETENA34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena33(&self) -> SETENA33_R {
        SETENA33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena32(&self) -> SETENA32_R {
        SETENA32_R::new((self.bits & 0x01) != 0)
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
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena37(&mut self) -> SETENA37_W {
        SETENA37_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena36(&mut self) -> SETENA36_W {
        SETENA36_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena35(&mut self) -> SETENA35_W {
        SETENA35_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena34(&mut self) -> SETENA34_W {
        SETENA34_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena33(&mut self) -> SETENA33_W {
        SETENA33_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena32(&mut self) -> SETENA32_W {
        SETENA32_W { w: self }
    }
}
