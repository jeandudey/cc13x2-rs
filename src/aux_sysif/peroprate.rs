#[doc = "Reader of register PEROPRATE"]
pub type R = crate::R<u32, super::PEROPRATE>;
#[doc = "Writer for register PEROPRATE"]
pub type W = crate::W<u32, super::PEROPRATE>;
#[doc = "Register PEROPRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::PEROPRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAIF_DAC_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<ANAIF_DAC_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: ANAIF_DAC_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANAIF_DAC_OP_RATE`"]
pub type ANAIF_DAC_OP_RATE_R = crate::R<bool, ANAIF_DAC_OP_RATE_A>;
impl ANAIF_DAC_OP_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANAIF_DAC_OP_RATE_A {
        match self.bits {
            true => ANAIF_DAC_OP_RATE_A::BUS_RATE,
            false => ANAIF_DAC_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == ANAIF_DAC_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == ANAIF_DAC_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `ANAIF_DAC_OP_RATE`"]
pub struct ANAIF_DAC_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAIF_DAC_OP_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANAIF_DAC_OP_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(ANAIF_DAC_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(ANAIF_DAC_OP_RATE_A::SCE_RATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "2:2\\]
Select operational rate for AUX_TIMER01.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER01_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<TIMER01_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER01_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER01_OP_RATE`"]
pub type TIMER01_OP_RATE_R = crate::R<bool, TIMER01_OP_RATE_A>;
impl TIMER01_OP_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER01_OP_RATE_A {
        match self.bits {
            true => TIMER01_OP_RATE_A::BUS_RATE,
            false => TIMER01_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == TIMER01_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == TIMER01_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `TIMER01_OP_RATE`"]
pub struct TIMER01_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER01_OP_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER01_OP_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(TIMER01_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(TIMER01_OP_RATE_A::SCE_RATE)
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
Select operational rate for AUX_SPIM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIM_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<SPIM_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIM_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIM_OP_RATE`"]
pub type SPIM_OP_RATE_R = crate::R<bool, SPIM_OP_RATE_A>;
impl SPIM_OP_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIM_OP_RATE_A {
        match self.bits {
            true => SPIM_OP_RATE_A::BUS_RATE,
            false => SPIM_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == SPIM_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == SPIM_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `SPIM_OP_RATE`"]
pub struct SPIM_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIM_OP_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIM_OP_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(SPIM_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(SPIM_OP_RATE_A::SCE_RATE)
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
Select operational rate for AUX_MAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_OP_RATE_A {
    #[doc = "1: AUX bus rate"]
    BUS_RATE = 1,
    #[doc = "0: SCE rate"]
    SCE_RATE = 0,
}
impl From<MAC_OP_RATE_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_OP_RATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAC_OP_RATE`"]
pub type MAC_OP_RATE_R = crate::R<bool, MAC_OP_RATE_A>;
impl MAC_OP_RATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAC_OP_RATE_A {
        match self.bits {
            true => MAC_OP_RATE_A::BUS_RATE,
            false => MAC_OP_RATE_A::SCE_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_RATE`"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == MAC_OP_RATE_A::BUS_RATE
    }
    #[doc = "Checks if the value of the field is `SCE_RATE`"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == MAC_OP_RATE_A::SCE_RATE
    }
}
#[doc = "Write proxy for field `MAC_OP_RATE`"]
pub struct MAC_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_OP_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_OP_RATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut W {
        self.variant(MAC_OP_RATE_A::BUS_RATE)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut W {
        self.variant(MAC_OP_RATE_A::SCE_RATE)
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
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline(always)]
    pub fn anaif_dac_op_rate(&self) -> ANAIF_DAC_OP_RATE_R {
        ANAIF_DAC_OP_RATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select operational rate for AUX_TIMER01."]
    #[inline(always)]
    pub fn timer01_op_rate(&self) -> TIMER01_OP_RATE_R {
        TIMER01_OP_RATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select operational rate for AUX_SPIM."]
    #[inline(always)]
    pub fn spim_op_rate(&self) -> SPIM_OP_RATE_R {
        SPIM_OP_RATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select operational rate for AUX_MAC."]
    #[inline(always)]
    pub fn mac_op_rate(&self) -> MAC_OP_RATE_R {
        MAC_OP_RATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline(always)]
    pub fn anaif_dac_op_rate(&mut self) -> ANAIF_DAC_OP_RATE_W {
        ANAIF_DAC_OP_RATE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select operational rate for AUX_TIMER01."]
    #[inline(always)]
    pub fn timer01_op_rate(&mut self) -> TIMER01_OP_RATE_W {
        TIMER01_OP_RATE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Select operational rate for AUX_SPIM."]
    #[inline(always)]
    pub fn spim_op_rate(&mut self) -> SPIM_OP_RATE_W {
        SPIM_OP_RATE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select operational rate for AUX_MAC."]
    #[inline(always)]
    pub fn mac_op_rate(&mut self) -> MAC_OP_RATE_W {
        MAC_OP_RATE_W { w: self }
    }
}
