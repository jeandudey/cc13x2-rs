#[doc = "Reader of register MIMR"]
pub type R = crate::R<u32, super::MIMR>;
#[doc = "Writer for register MIMR"]
pub type W = crate::W<u32, super::MIMR>;
#[doc = "Register MIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MIMR {
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
Interrupt mask 0: The MRIS.RIS interrupt is suppressed and not sent to the interrupt controller. 1: The master interrupt is sent to the interrupt controller when the MRIS.RIS is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<IM_A> for bool {
    #[inline(always)]
    fn from(variant: IM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IM`"]
pub type IM_R = crate::R<bool, IM_A>;
impl IM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM_A {
        match self.bits {
            true => IM_A::EN,
            false => IM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IM_A::DIS
    }
}
#[doc = "Write proxy for field `IM`"]
pub struct IM_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IM_A::DIS)
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask 0: The MRIS.RIS interrupt is suppressed and not sent to the interrupt controller. 1: The master interrupt is sent to the interrupt controller when the MRIS.RIS is set."]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask 0: The MRIS.RIS interrupt is suppressed and not sent to the interrupt controller. 1: The master interrupt is sent to the interrupt controller when the MRIS.RIS is set."]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
}
