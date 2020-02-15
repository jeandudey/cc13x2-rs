#[doc = "Reader of register SICR"]
pub type R = crate::R<u32, super::SICR>;
#[doc = "Writer for register SICR"]
pub type W = crate::W<u32, super::SICR>;
#[doc = "Register SICR `reset()`'s with value 0"]
impl crate::ResetValue for super::SICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `STOPIC`"]
pub type STOPIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPIC`"]
pub struct STOPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIC_W<'a> {
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
#[doc = "Reader of field `STARTIC`"]
pub type STARTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTIC`"]
pub struct STARTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIC_W<'a> {
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
#[doc = "Reader of field `DATAIC`"]
pub type DATAIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAIC`"]
pub struct DATAIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIC_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub fn stopic(&self) -> STOPIC_R {
        STOPIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    pub fn startic(&self) -> STARTIC_R {
        STARTIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    pub fn dataic(&self) -> DATAIC_R {
        DATAIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub fn stopic(&mut self) -> STOPIC_W {
        STOPIC_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    pub fn startic(&mut self) -> STARTIC_W {
        STARTIC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    pub fn dataic(&mut self) -> DATAIC_W {
        DATAIC_W { w: self }
    }
}
