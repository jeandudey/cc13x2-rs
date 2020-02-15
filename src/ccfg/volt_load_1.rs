#[doc = "Reader of register VOLT_LOAD_1"]
pub type R = crate::R<u32, super::VOLT_LOAD_1>;
#[doc = "Writer for register VOLT_LOAD_1"]
pub type W = crate::W<u32, super::VOLT_LOAD_1>;
#[doc = "Register VOLT_LOAD_1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::VOLT_LOAD_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `VDDR_EXT_TP125`"]
pub type VDDR_EXT_TP125_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP125`"]
pub struct VDDR_EXT_TP125_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP125_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_TP105`"]
pub type VDDR_EXT_TP105_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP105`"]
pub struct VDDR_EXT_TP105_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP105_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_TP85`"]
pub type VDDR_EXT_TP85_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP85`"]
pub struct VDDR_EXT_TP85_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP85_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_TP65`"]
pub type VDDR_EXT_TP65_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP65`"]
pub struct VDDR_EXT_TP65_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP65_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp125(&self) -> VDDR_EXT_TP125_R {
        VDDR_EXT_TP125_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp105(&self) -> VDDR_EXT_TP105_R {
        VDDR_EXT_TP105_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp85(&self) -> VDDR_EXT_TP85_R {
        VDDR_EXT_TP85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp65(&self) -> VDDR_EXT_TP65_R {
        VDDR_EXT_TP65_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp125(&mut self) -> VDDR_EXT_TP125_W {
        VDDR_EXT_TP125_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp105(&mut self) -> VDDR_EXT_TP105_W {
        VDDR_EXT_TP105_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp85(&mut self) -> VDDR_EXT_TP85_W {
        VDDR_EXT_TP85_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp65(&mut self) -> VDDR_EXT_TP65_W {
        VDDR_EXT_TP65_W { w: self }
    }
}
