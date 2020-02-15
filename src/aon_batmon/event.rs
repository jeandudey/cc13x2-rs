#[doc = "Reader of register EVENT"]
pub type R = crate::R<u32, super::EVENT>;
#[doc = "Writer for register EVENT"]
pub type W = crate::W<u32, super::EVENT>;
#[doc = "Register EVENT `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `TEMP_UPDATE`"]
pub type TEMP_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP_UPDATE`"]
pub struct TEMP_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BATT_UPDATE`"]
pub type BATT_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATT_UPDATE`"]
pub struct BATT_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> BATT_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TEMP_BELOW_LL`"]
pub type TEMP_BELOW_LL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP_BELOW_LL`"]
pub struct TEMP_BELOW_LL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_BELOW_LL_W<'a> {
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
#[doc = "Reader of field `TEMP_OVER_UL`"]
pub type TEMP_OVER_UL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP_OVER_UL`"]
pub struct TEMP_OVER_UL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_OVER_UL_W<'a> {
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
#[doc = "Reader of field `BATT_BELOW_LL`"]
pub type BATT_BELOW_LL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATT_BELOW_LL`"]
pub struct BATT_BELOW_LL_W<'a> {
    w: &'a mut W,
}
impl<'a> BATT_BELOW_LL_W<'a> {
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
#[doc = "Reader of field `BATT_OVER_UL`"]
pub type BATT_OVER_UL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATT_OVER_UL`"]
pub struct BATT_OVER_UL_W<'a> {
    w: &'a mut W,
}
impl<'a> BATT_OVER_UL_W<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
Alias to TEMPUPD.STAT"]
    #[inline(always)]
    pub fn temp_update(&self) -> TEMP_UPDATE_R {
        TEMP_UPDATE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Alias to BATUPD.STAT"]
    #[inline(always)]
    pub fn batt_update(&self) -> BATT_UPDATE_R {
        BATT_UPDATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_below_ll(&self) -> TEMP_BELOW_LL_R {
        TEMP_BELOW_LL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_over_ul(&self) -> TEMP_OVER_UL_R {
        TEMP_OVER_UL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_below_ll(&self) -> BATT_BELOW_LL_R {
        BATT_BELOW_LL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_over_ul(&self) -> BATT_OVER_UL_R {
        BATT_OVER_UL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Alias to TEMPUPD.STAT"]
    #[inline(always)]
    pub fn temp_update(&mut self) -> TEMP_UPDATE_W {
        TEMP_UPDATE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Alias to BATUPD.STAT"]
    #[inline(always)]
    pub fn batt_update(&mut self) -> BATT_UPDATE_W {
        BATT_UPDATE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Read: 1: Temperature level is below the lower limit set by TEMPLL. 0: Temperature level is not below the lower limit set by TEMPLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_below_ll(&mut self) -> TEMP_BELOW_LL_W {
        TEMP_BELOW_LL_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Read: 1: Temperature level is above the upper limit set by TEMPUL. 0: Temperature level is not above the upper limit set by TEMPUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn temp_over_ul(&mut self) -> TEMP_OVER_UL_W {
        TEMP_OVER_UL_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Read: 1: Battery level is below the lower limit set by BATTLL. 0: Battery level is not below the lower limit set by BATTLL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_below_ll(&mut self) -> BATT_BELOW_LL_W {
        BATT_BELOW_LL_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Read: 1: Battery level is above the upper limit set by BATTUL. 0: Battery level is not above the upper limit set by BATTUL. Write: 1: Clears the flag 0: No change in the flag"]
    #[inline(always)]
    pub fn batt_over_ul(&mut self) -> BATT_OVER_UL_W {
        BATT_OVER_UL_W { w: self }
    }
}
