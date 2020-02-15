#[doc = "Reader of register CPUCLKDIV"]
pub type R = crate::R<u32, super::CPUCLKDIV>;
#[doc = "Writer for register CPUCLKDIV"]
pub type W = crate::W<u32, super::CPUCLKDIV>;
#[doc = "Register CPUCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIO_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DIV2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    DIV1 = 0,
}
impl From<RATIO_A> for bool {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<bool, RATIO_A>;
impl RATIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            true => RATIO_A::DIV2,
            false => RATIO_A::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RATIO_A::DIV1
    }
}
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
    }
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
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
