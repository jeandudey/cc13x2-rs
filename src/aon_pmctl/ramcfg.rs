#[doc = "Reader of register RAMCFG"]
pub type R = crate::R<u32, super::RAMCFG>;
#[doc = "Writer for register RAMCFG"]
pub type W = crate::W<u32, super::RAMCFG>;
#[doc = "Register RAMCFG `reset()`'s with value 0x0001_000f"]
impl crate::ResetValue for super::RAMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_000f
    }
}
#[doc = "Reader of field `RESERVED18`"]
pub type RESERVED18_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED18`"]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | (((value as u32) & 0x3fff) << 18);
        self.w
    }
}
#[doc = "Reader of field `AUX_SRAM_PWR_OFF`"]
pub type AUX_SRAM_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_SRAM_PWR_OFF`"]
pub struct AUX_SRAM_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SRAM_PWR_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `AUX_SRAM_RET_EN`"]
pub type AUX_SRAM_RET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_SRAM_RET_EN`"]
pub struct AUX_SRAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SRAM_RET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUS_SRAM_RET_EN_A {
    #[doc = "15: Retention on for all banks SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2,  SRAM:BANK3  and SRAM:BANK4"]
    RET_FULL = 15,
    #[doc = "7: Retention on for SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3"]
    RET_LEVEL3 = 7,
    #[doc = "3: Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    RET_LEVEL2 = 3,
    #[doc = "1: Retention on for SRAM:BANK0 and  SRAM:BANK1"]
    RET_LEVEL1 = 1,
    #[doc = "0: Retention is disabled"]
    RET_NONE = 0,
}
impl From<BUS_SRAM_RET_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: BUS_SRAM_RET_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUS_SRAM_RET_EN`"]
pub type BUS_SRAM_RET_EN_R = crate::R<u8, BUS_SRAM_RET_EN_A>;
impl BUS_SRAM_RET_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BUS_SRAM_RET_EN_A> {
        use crate::Variant::*;
        match self.bits {
            15 => Val(BUS_SRAM_RET_EN_A::RET_FULL),
            7 => Val(BUS_SRAM_RET_EN_A::RET_LEVEL3),
            3 => Val(BUS_SRAM_RET_EN_A::RET_LEVEL2),
            1 => Val(BUS_SRAM_RET_EN_A::RET_LEVEL1),
            0 => Val(BUS_SRAM_RET_EN_A::RET_NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RET_FULL`"]
    #[inline(always)]
    pub fn is_ret_full(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_FULL
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL3`"]
    #[inline(always)]
    pub fn is_ret_level3(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_LEVEL3
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL2`"]
    #[inline(always)]
    pub fn is_ret_level2(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_LEVEL2
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL1`"]
    #[inline(always)]
    pub fn is_ret_level1(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_LEVEL1
    }
    #[doc = "Checks if the value of the field is `RET_NONE`"]
    #[inline(always)]
    pub fn is_ret_none(&self) -> bool {
        *self == BUS_SRAM_RET_EN_A::RET_NONE
    }
}
#[doc = "Write proxy for field `BUS_SRAM_RET_EN`"]
pub struct BUS_SRAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_SRAM_RET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUS_SRAM_RET_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Retention on for all banks SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2, SRAM:BANK3 and SRAM:BANK4"]
    #[inline(always)]
    pub fn ret_full(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_FULL)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3"]
    #[inline(always)]
    pub fn ret_level3(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_LEVEL3)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn ret_level2(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_LEVEL2)
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn ret_level1(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_LEVEL1)
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn ret_none(self) -> &'a mut W {
        self.variant(BUS_SRAM_RET_EN_A::RET_NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_pwr_off(&self) -> AUX_SRAM_PWR_OFF_R {
        AUX_SRAM_PWR_OFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_ret_en(&self) -> AUX_SRAM_RET_EN_R {
        AUX_SRAM_RET_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off"]
    #[inline(always)]
    pub fn bus_sram_ret_en(&self) -> BUS_SRAM_RET_EN_R {
        BUS_SRAM_RET_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_pwr_off(&mut self) -> AUX_SRAM_PWR_OFF_W {
        AUX_SRAM_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_sram_ret_en(&mut self) -> AUX_SRAM_RET_EN_W {
        AUX_SRAM_RET_EN_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 5 banks . This register controls which of the banks that has retention during MCU Bus domain power off"]
    #[inline(always)]
    pub fn bus_sram_ret_en(&mut self) -> BUS_SRAM_RET_EN_W {
        BUS_SRAM_RET_EN_W { w: self }
    }
}
