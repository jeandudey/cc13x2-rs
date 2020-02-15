#[doc = "Reader of register VOLT_LOAD_0"]
pub type R = crate::R<u32, super::VOLT_LOAD_0>;
#[doc = "Writer for register VOLT_LOAD_0"]
pub type W = crate::W<u32, super::VOLT_LOAD_0>;
#[doc = "Register VOLT_LOAD_0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::VOLT_LOAD_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `VDDR_EXT_TP45`"]
pub type VDDR_EXT_TP45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP45`"]
pub struct VDDR_EXT_TP45_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_TP25`"]
pub type VDDR_EXT_TP25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP25`"]
pub struct VDDR_EXT_TP25_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_TP5`"]
pub type VDDR_EXT_TP5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TP5`"]
pub struct VDDR_EXT_TP5_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VDDR_EXT_TM15`"]
pub type VDDR_EXT_TM15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_EXT_TM15`"]
pub struct VDDR_EXT_TM15_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TM15_W<'a> {
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
    pub fn vddr_ext_tp45(&self) -> VDDR_EXT_TP45_R {
        VDDR_EXT_TP45_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp25(&self) -> VDDR_EXT_TP25_R {
        VDDR_EXT_TP25_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp5(&self) -> VDDR_EXT_TP5_R {
        VDDR_EXT_TP5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tm15(&self) -> VDDR_EXT_TM15_R {
        VDDR_EXT_TM15_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp45(&mut self) -> VDDR_EXT_TP45_W {
        VDDR_EXT_TP45_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp25(&mut self) -> VDDR_EXT_TP25_W {
        VDDR_EXT_TP25_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp5(&mut self) -> VDDR_EXT_TP5_W {
        VDDR_EXT_TP5_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tm15(&mut self) -> VDDR_EXT_TM15_W {
        VDDR_EXT_TM15_W { w: self }
    }
}
