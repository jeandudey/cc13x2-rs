#[doc = "Reader of register MUX3"]
pub type R = crate::R<u8, super::MUX3>;
#[doc = "Writer for register MUX3"]
pub type W = crate::W<u8, super::MUX3>;
#[doc = "Register MUX3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCCOMPB_IN_A {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    AUXIO19 = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AUXIO20 = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AUXIO21 = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AUXIO22 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AUXIO23 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AUXIO24 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AUXIO25 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AUXIO26 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<ADCCOMPB_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCCOMPB_IN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCCOMPB_IN`"]
pub type ADCCOMPB_IN_R = crate::R<u8, ADCCOMPB_IN_A>;
impl ADCCOMPB_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCCOMPB_IN_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(ADCCOMPB_IN_A::AUXIO19),
            64 => Val(ADCCOMPB_IN_A::AUXIO20),
            32 => Val(ADCCOMPB_IN_A::AUXIO21),
            16 => Val(ADCCOMPB_IN_A::AUXIO22),
            8 => Val(ADCCOMPB_IN_A::AUXIO23),
            4 => Val(ADCCOMPB_IN_A::AUXIO24),
            2 => Val(ADCCOMPB_IN_A::AUXIO25),
            1 => Val(ADCCOMPB_IN_A::AUXIO26),
            0 => Val(ADCCOMPB_IN_A::NC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO19`"]
    #[inline(always)]
    pub fn is_auxio19(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO19
    }
    #[doc = "Checks if the value of the field is `AUXIO20`"]
    #[inline(always)]
    pub fn is_auxio20(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO20
    }
    #[doc = "Checks if the value of the field is `AUXIO21`"]
    #[inline(always)]
    pub fn is_auxio21(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO21
    }
    #[doc = "Checks if the value of the field is `AUXIO22`"]
    #[inline(always)]
    pub fn is_auxio22(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO22
    }
    #[doc = "Checks if the value of the field is `AUXIO23`"]
    #[inline(always)]
    pub fn is_auxio23(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO23
    }
    #[doc = "Checks if the value of the field is `AUXIO24`"]
    #[inline(always)]
    pub fn is_auxio24(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO24
    }
    #[doc = "Checks if the value of the field is `AUXIO25`"]
    #[inline(always)]
    pub fn is_auxio25(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO25
    }
    #[doc = "Checks if the value of the field is `AUXIO26`"]
    #[inline(always)]
    pub fn is_auxio26(&self) -> bool {
        *self == ADCCOMPB_IN_A::AUXIO26
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == ADCCOMPB_IN_A::NC
    }
}
#[doc = "Write proxy for field `ADCCOMPB_IN`"]
pub struct ADCCOMPB_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCOMPB_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCCOMPB_IN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio19(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO19)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio20(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio21(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO21)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio22(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO22)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio23(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO23)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio24(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO24)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio25(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO25)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio26(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO26)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
}
