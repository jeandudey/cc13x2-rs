#[doc = "Reader of register PWRCTL"]
pub type R = crate::R<u32, super::PWRCTL>;
#[doc = "Writer for register PWRCTL"]
pub type W = crate::W<u32, super::PWRCTL>;
#[doc = "Register PWRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCTL {
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
#[doc = "Reader of field `DCDC_ACTIVE`"]
pub type DCDC_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_ACTIVE`"]
pub struct DCDC_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_ACTIVE_W<'a> {
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
#[doc = "Reader of field `EXT_REG_MODE`"]
pub type EXT_REG_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_REG_MODE`"]
pub struct EXT_REG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_REG_MODE_W<'a> {
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
#[doc = "Reader of field `DCDC_EN`"]
pub type DCDC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_EN`"]
pub struct DCDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_EN_W<'a> {
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
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDR in active mode. 1: Use DCDC for regulation of VDDR in active mode. DCDC_EN must also be set for DCDC to be used as regulator for VDDR in active mode"]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC or GLDO are generating VDDR 1: DCDC and GLDO are bypassed and an external regulator supplies VDDR"]
    #[inline(always)]
    pub fn ext_reg_mode(&self) -> EXT_REG_MODE_R {
        EXT_REG_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_en(&self) -> DCDC_EN_R {
        DCDC_EN_R::new((self.bits & 0x01) != 0)
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
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDR in active mode. 1: Use DCDC for regulation of VDDR in active mode. DCDC_EN must also be set for DCDC to be used as regulator for VDDR in active mode"]
    #[inline(always)]
    pub fn dcdc_active(&mut self) -> DCDC_ACTIVE_W {
        DCDC_ACTIVE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC or GLDO are generating VDDR 1: DCDC and GLDO are bypassed and an external regulator supplies VDDR"]
    #[inline(always)]
    pub fn ext_reg_mode(&mut self) -> EXT_REG_MODE_W {
        EXT_REG_MODE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_en(&mut self) -> DCDC_EN_W {
        DCDC_EN_W { w: self }
    }
}
