#[doc = "Reader of register TRACECLKMUX"]
pub type R = crate::R<u32, super::TRACECLKMUX>;
#[doc = "Writer for register TRACECLKMUX"]
pub type W = crate::W<u32, super::TRACECLKMUX>;
#[doc = "Register TRACECLKMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACECLKMUX {
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
pub enum TRACECLK_N_SWV_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    TRACECLK = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    SWV = 0,
}
impl From<TRACECLK_N_SWV_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLK_N_SWV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRACECLK_N_SWV`"]
pub type TRACECLK_N_SWV_R = crate::R<bool, TRACECLK_N_SWV_A>;
impl TRACECLK_N_SWV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLK_N_SWV_A {
        match self.bits {
            true => TRACECLK_N_SWV_A::TRACECLK,
            false => TRACECLK_N_SWV_A::SWV,
        }
    }
    #[doc = "Checks if the value of the field is `TRACECLK`"]
    #[inline(always)]
    pub fn is_traceclk(&self) -> bool {
        *self == TRACECLK_N_SWV_A::TRACECLK
    }
    #[doc = "Checks if the value of the field is `SWV`"]
    #[inline(always)]
    pub fn is_swv(&self) -> bool {
        *self == TRACECLK_N_SWV_A::SWV
    }
}
#[doc = "Write proxy for field `TRACECLK_N_SWV`"]
pub struct TRACECLK_N_SWV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLK_N_SWV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECLK_N_SWV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn traceclk(self) -> &'a mut W {
        self.variant(TRACECLK_N_SWV_A::TRACECLK)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn swv(self) -> &'a mut W {
        self.variant(TRACECLK_N_SWV_A::SWV)
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
    pub fn traceclk_n_swv(&self) -> TRACECLK_N_SWV_R {
        TRACECLK_N_SWV_R::new((self.bits & 0x01) != 0)
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
    pub fn traceclk_n_swv(&mut self) -> TRACECLK_N_SWV_W {
        TRACECLK_N_SWV_W { w: self }
    }
}
