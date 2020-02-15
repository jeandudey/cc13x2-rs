#[doc = "Reader of register LFOSCCTL"]
pub type R = crate::R<u32, super::LFOSCCTL>;
#[doc = "Writer for register LFOSCCTL"]
pub type W = crate::W<u32, super::LFOSCCTL>;
#[doc = "Register LFOSCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LFOSCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `XOSCLF_REGULATOR_TRIM`"]
pub type XOSCLF_REGULATOR_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSCLF_REGULATOR_TRIM`"]
pub struct XOSCLF_REGULATOR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLF_REGULATOR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `XOSCLF_CMIRRWR_RATIO`"]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSCLF_CMIRRWR_RATIO`"]
pub struct XOSCLF_CMIRRWR_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLF_CMIRRWR_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 10)) | (((value as u32) & 0xff) << 10);
        self.w
    }
}
#[doc = "9:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCOSCLF_RTUNE_TRIM_A {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _6P0MEG = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _6P5MEG = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _7P0MEG = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _7P5MEG = 0,
}
impl From<RCOSCLF_RTUNE_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: RCOSCLF_RTUNE_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCOSCLF_RTUNE_TRIM`"]
pub type RCOSCLF_RTUNE_TRIM_R = crate::R<u8, RCOSCLF_RTUNE_TRIM_A>;
impl RCOSCLF_RTUNE_TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCOSCLF_RTUNE_TRIM_A {
        match self.bits {
            3 => RCOSCLF_RTUNE_TRIM_A::_6P0MEG,
            2 => RCOSCLF_RTUNE_TRIM_A::_6P5MEG,
            1 => RCOSCLF_RTUNE_TRIM_A::_7P0MEG,
            0 => RCOSCLF_RTUNE_TRIM_A::_7P5MEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_6P0MEG`"]
    #[inline(always)]
    pub fn is_6p0meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_6P0MEG
    }
    #[doc = "Checks if the value of the field is `_6P5MEG`"]
    #[inline(always)]
    pub fn is_6p5meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_6P5MEG
    }
    #[doc = "Checks if the value of the field is `_7P0MEG`"]
    #[inline(always)]
    pub fn is_7p0meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_7P0MEG
    }
    #[doc = "Checks if the value of the field is `_7P5MEG`"]
    #[inline(always)]
    pub fn is_7p5meg(&self) -> bool {
        *self == RCOSCLF_RTUNE_TRIM_A::_7P5MEG
    }
}
#[doc = "Write proxy for field `RCOSCLF_RTUNE_TRIM`"]
pub struct RCOSCLF_RTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_RTUNE_TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCOSCLF_RTUNE_TRIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _6p0meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_6P0MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _6p5meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_6P5MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _7p0meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_7P0MEG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _7p5meg(self) -> &'a mut W {
        self.variant(RCOSCLF_RTUNE_TRIM_A::_7P5MEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RCOSCLF_CTUNE_TRIM`"]
pub type RCOSCLF_CTUNE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCLF_CTUNE_TRIM`"]
pub struct RCOSCLF_CTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_CTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 10:17 - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIM_R {
        RCOSCLF_RTUNE_TRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIM_R {
        RCOSCLF_CTUNE_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&mut self) -> XOSCLF_REGULATOR_TRIM_W {
        XOSCLF_REGULATOR_TRIM_W { w: self }
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> XOSCLF_CMIRRWR_RATIO_W {
        XOSCLF_CMIRRWR_RATIO_W { w: self }
    }
    #[doc = "Bits 10:17 - 17:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&mut self) -> RCOSCLF_RTUNE_TRIM_W {
        RCOSCLF_RTUNE_TRIM_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&mut self) -> RCOSCLF_CTUNE_TRIM_W {
        RCOSCLF_CTUNE_TRIM_W { w: self }
    }
}
