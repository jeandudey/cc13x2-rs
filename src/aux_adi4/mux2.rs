#[doc = "Reader of register MUX2"]
pub type R = crate::R<u8, super::MUX2>;
#[doc = "Writer for register MUX2"]
pub type W = crate::W<u8, super::MUX2>;
#[doc = "Register MUX2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "7:3\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCCOMPB_IN_A {
    #[doc = "16: Internal. Only to be used through TI provided API."]
    VDDS = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    VSS = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    DCOUPL = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    ATEST1 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    ATEST0 = 1,
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
            16 => Val(ADCCOMPB_IN_A::VDDS),
            8 => Val(ADCCOMPB_IN_A::VSS),
            4 => Val(ADCCOMPB_IN_A::DCOUPL),
            2 => Val(ADCCOMPB_IN_A::ATEST1),
            1 => Val(ADCCOMPB_IN_A::ATEST0),
            0 => Val(ADCCOMPB_IN_A::NC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == ADCCOMPB_IN_A::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == ADCCOMPB_IN_A::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        *self == ADCCOMPB_IN_A::DCOUPL
    }
    #[doc = "Checks if the value of the field is `ATEST1`"]
    #[inline(always)]
    pub fn is_atest1(&self) -> bool {
        *self == ADCCOMPB_IN_A::ATEST1
    }
    #[doc = "Checks if the value of the field is `ATEST0`"]
    #[inline(always)]
    pub fn is_atest0(&self) -> bool {
        *self == ADCCOMPB_IN_A::ATEST0
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
    pub fn vdds(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest1(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::ATEST1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest0(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::ATEST0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u8) & 0x1f) << 3);
        self.w
    }
}
#[doc = "2:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAC_VREF_SEL_A {
    #[doc = "4: Internal. Only to be used through TI provided API."]
    VDDS = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    ADCREF = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DCOUPL = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<DAC_VREF_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_VREF_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DAC_VREF_SEL`"]
pub type DAC_VREF_SEL_R = crate::R<u8, DAC_VREF_SEL_A>;
impl DAC_VREF_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DAC_VREF_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(DAC_VREF_SEL_A::VDDS),
            2 => Val(DAC_VREF_SEL_A::ADCREF),
            1 => Val(DAC_VREF_SEL_A::DCOUPL),
            0 => Val(DAC_VREF_SEL_A::NC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        *self == DAC_VREF_SEL_A::VDDS
    }
    #[doc = "Checks if the value of the field is `ADCREF`"]
    #[inline(always)]
    pub fn is_adcref(&self) -> bool {
        *self == DAC_VREF_SEL_A::ADCREF
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        *self == DAC_VREF_SEL_A::DCOUPL
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DAC_VREF_SEL_A::NC
    }
}
#[doc = "Write proxy for field `DAC_VREF_SEL`"]
pub struct DAC_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_VREF_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC_VREF_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut W {
        self.variant(DAC_VREF_SEL_A::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adcref(self) -> &'a mut W {
        self.variant(DAC_VREF_SEL_A::ADCREF)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(DAC_VREF_SEL_A::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(DAC_VREF_SEL_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dac_vref_sel(&self) -> DAC_VREF_SEL_R {
        DAC_VREF_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dac_vref_sel(&mut self) -> DAC_VREF_SEL_W {
        DAC_VREF_SEL_W { w: self }
    }
}
