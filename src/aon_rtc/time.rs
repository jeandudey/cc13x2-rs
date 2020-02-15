#[doc = "Reader of register TIME"]
pub type R = crate::R<u32, super::TIME>;
#[doc = "Writer for register TIME"]
pub type W = crate::W<u32, super::TIME>;
#[doc = "Register TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC_L`"]
pub type SEC_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SEC_L`"]
pub struct SEC_L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SUBSEC_H`"]
pub type SUBSEC_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUBSEC_H`"]
pub struct SUBSEC_H_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBSEC_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Returns the lower halfword of SEC register."]
    #[inline(always)]
    pub fn sec_l(&self) -> SEC_L_R {
        SEC_L_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Returns the upper halfword of SUBSEC register."]
    #[inline(always)]
    pub fn subsec_h(&self) -> SUBSEC_H_R {
        SUBSEC_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Returns the lower halfword of SEC register."]
    #[inline(always)]
    pub fn sec_l(&mut self) -> SEC_L_W {
        SEC_L_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Returns the upper halfword of SUBSEC register."]
    #[inline(always)]
    pub fn subsec_h(&mut self) -> SUBSEC_H_W {
        SUBSEC_H_W { w: self }
    }
}
