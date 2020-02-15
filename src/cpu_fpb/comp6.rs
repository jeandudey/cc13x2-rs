#[doc = "Reader of register COMP6"]
pub type R = crate::R<u32, super::COMP6>;
#[doc = "Writer for register COMP6"]
pub type W = crate::W<u32, super::COMP6>;
#[doc = "Register COMP6 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REPLACE`"]
pub type REPLACE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REPLACE`"]
pub struct REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPLACE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RESERVED29`"]
pub type RESERVED29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED29`"]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 2)) | (((value as u32) & 0x07ff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched. Comparator 6 is a literal comparator and the only supported setting is 0x0. Other settings will be ignored. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub fn replace(&self) -> REPLACE_R {
        REPLACE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 2) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Compare and remap enable comparator 6. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 6 disabled 0x1: Compare and remap for comparator 6 enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched. Comparator 6 is a literal comparator and the only supported setting is 0x0. Other settings will be ignored. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub fn replace(&mut self) -> REPLACE_W {
        REPLACE_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address."]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Compare and remap enable comparator 6. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 6 disabled 0x1: Compare and remap for comparator 6 enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
