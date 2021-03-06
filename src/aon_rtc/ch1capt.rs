#[doc = "Reader of register CH1CAPT"]
pub type R = crate::R<u32, super::CH1CAPT>;
#[doc = "Writer for register CH1CAPT"]
pub type W = crate::W<u32, super::CH1CAPT>;
#[doc = "Register CH1CAPT `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CAPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SEC`"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SUBSEC`"]
pub type SUBSEC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUBSEC`"]
pub struct SUBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    pub fn subsec(&mut self) -> SUBSEC_W {
        SUBSEC_W { w: self }
    }
}
