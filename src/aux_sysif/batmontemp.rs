#[doc = "Reader of register BATMONTEMP"]
pub type R = crate::R<u32, super::BATMONTEMP>;
#[doc = "Writer for register BATMONTEMP"]
pub type W = crate::W<u32, super::BATMONTEMP>;
#[doc = "Register BATMONTEMP `reset()`'s with value 0"]
impl crate::ResetValue for super::BATMONTEMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SIGN`"]
pub type SIGN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIGN`"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAC`"]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 2:10 - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:1 - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Sign extension of INT. Follow this procedure to get the correct value: - Do two dummy reads of SIGN. - Then read SIGN until two consecutive reads are equal."]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W {
        SIGN_W { w: self }
    }
    #[doc = "Bits 2:10 - 10:2\\]
See AON_BATMON:TEMP.INT. Follow this procedure to get the correct value: - Do two dummy reads of INT. - Then read INT until two consecutive reads are equal."]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
See AON_BATMON:TEMP.FRAC. Follow this procedure to get the correct value: - Do two dummy reads of FRAC. - Then read FRAC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W {
        FRAC_W { w: self }
    }
}
