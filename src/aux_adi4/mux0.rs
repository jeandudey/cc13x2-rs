#[doc = "Reader of register MUX0"]
pub type R = crate::R<u8, super::MUX0>;
#[doc = "Writer for register MUX0"]
pub type W = crate::W<u8, super::MUX0>;
#[doc = "Register MUX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "6:6\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCOMPB_IN_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    VDDR_1P8V = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<ADCCOMPB_IN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCCOMPB_IN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCCOMPB_IN`"]
pub type ADCCOMPB_IN_R = crate::R<bool, ADCCOMPB_IN_A>;
impl ADCCOMPB_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCOMPB_IN_A {
        match self.bits {
            true => ADCCOMPB_IN_A::VDDR_1P8V,
            false => ADCCOMPB_IN_A::NC,
        }
    }
    #[doc = "Checks if the value of the field is `VDDR_1P8V`"]
    #[inline(always)]
    pub fn is_vddr_1p8v(&self) -> bool {
        *self == ADCCOMPB_IN_A::VDDR_1P8V
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_1p8v(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::VDDR_1P8V)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::NC)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPA_REF_A {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    ADCVREFP = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    VDDS = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    VSS = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DCOUPL = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<COMPA_REF_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPA_REF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMPA_REF`"]
pub type COMPA_REF_R = crate::R<u8, COMPA_REF_A>;
impl COMPA_REF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMPA_REF_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(COMPA_REF_A::ADCVREFP),
            4 => Val(COMPA_REF_A::VDDS),
            2 => Val(COMPA_REF_A::VSS),
            1 => Val(COMPA_REF_A::DCOUPL),
            0 => Val(COMPA_REF_A::NC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCVREFP`"]
    #[inline(always)]
    pub fn is_adcvrefp(&self) -> bool {
        *self == COMPA_REF_A::ADCVREFP
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == COMPA_REF_A::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == COMPA_REF_A::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        *self == COMPA_REF_A::DCOUPL
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == COMPA_REF_A::NC
    }
}
#[doc = "Write proxy for field `COMPA_REF`"]
pub struct COMPA_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPA_REF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adcvrefp(self) -> &'a mut W {
        self.variant(COMPA_REF_A::ADCVREFP)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut W {
        self.variant(COMPA_REF_A::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(COMPA_REF_A::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(COMPA_REF_A::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_REF_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&self) -> COMPA_REF_R {
        COMPA_REF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&mut self) -> COMPA_REF_W {
        COMPA_REF_W { w: self }
    }
}
