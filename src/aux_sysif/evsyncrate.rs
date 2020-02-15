#[doc = "Reader of register EVSYNCRATE"]
pub type R = crate::R<u32, super::EVSYNCRATE>;
#[doc = "Writer for register EVSYNCRATE"]
pub type W = crate::W<u32, super::EVSYNCRATE>;
#[doc = "Register EVSYNCRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSYNCRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPA_SYNC_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<AUX_COMPA_SYNC_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_SYNC_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_COMPA_SYNC_RATE`"]
pub type AUX_COMPA_SYNC_RATE_R = crate::R<bool, AUX_COMPA_SYNC_RATE_A>;
impl AUX_COMPA_SYNC_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPA_SYNC_RATE_A {
        match self.bits {
            true => AUX_COMPA_SYNC_RATE_A::BUS_RATE,
            false => AUX_COMPA_SYNC_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_COMPA_SYNC_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_COMPA_SYNC_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `AUX_COMPA_SYNC_RATE`"]
pub struct AUX_COMPA_SYNC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_SYNC_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPA_SYNC_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_COMPA_SYNC_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_COMPA_SYNC_RATE_A::SCE_RATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPB_SYNC_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<AUX_COMPB_SYNC_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_SYNC_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_COMPB_SYNC_RATE`"]
pub type AUX_COMPB_SYNC_RATE_R = crate::R<bool, AUX_COMPB_SYNC_RATE_A>;
impl AUX_COMPB_SYNC_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPB_SYNC_RATE_A {
        match self.bits {
            true => AUX_COMPB_SYNC_RATE_A::BUS_RATE,
            false => AUX_COMPB_SYNC_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_COMPB_SYNC_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_COMPB_SYNC_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `AUX_COMPB_SYNC_RATE`"]
pub struct AUX_COMPB_SYNC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_SYNC_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPB_SYNC_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_COMPB_SYNC_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_COMPB_SYNC_RATE_A::SCE_RATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_SYNC_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<AUX_TIMER2_SYNC_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_SYNC_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER2_SYNC_RATE`"]
pub type AUX_TIMER2_SYNC_RATE_R = crate::R<bool, AUX_TIMER2_SYNC_RATE_A>;
impl AUX_TIMER2_SYNC_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_SYNC_RATE_A {
        match self.bits {
            true => AUX_TIMER2_SYNC_RATE_A::BUS_RATE,
            false => AUX_TIMER2_SYNC_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AUX_TIMER2_SYNC_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AUX_TIMER2_SYNC_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `AUX_TIMER2_SYNC_RATE`"]
pub struct AUX_TIMER2_SYNC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_SYNC_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER2_SYNC_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(AUX_TIMER2_SYNC_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(AUX_TIMER2_SYNC_RATE_A::SCE_RATE)
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline(always)]
    pub fn aux_compa_sync_rate(&self) -> AUX_COMPA_SYNC_RATE_R {
        AUX_COMPA_SYNC_RATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline(always)]
    pub fn aux_compb_sync_rate(&self) -> AUX_COMPB_SYNC_RATE_R {
        AUX_COMPB_SYNC_RATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_sync_rate(&self) -> AUX_TIMER2_SYNC_RATE_R {
        AUX_TIMER2_SYNC_RATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline(always)]
    pub fn aux_compa_sync_rate(&mut self) -> AUX_COMPA_SYNC_RATE_W {
        AUX_COMPA_SYNC_RATE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline(always)]
    pub fn aux_compb_sync_rate(&mut self) -> AUX_COMPB_SYNC_RATE_W {
        AUX_COMPB_SYNC_RATE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_sync_rate(&mut self) -> AUX_TIMER2_SYNC_RATE_W {
        AUX_TIMER2_SYNC_RATE_W { w: self }
    }
}
