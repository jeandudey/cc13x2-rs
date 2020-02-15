#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
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
#[doc = "5:5\\]
I2C slave function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFE_A {
    #[doc = "1: Slave mode is enabled."]
    EN = 1,
    #[doc = "0: Slave mode is disabled."]
    DIS = 0,
}
impl From<SFE_A> for bool {
    #[inline(always)]
    fn from(variant: SFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SFE`"]
pub type SFE_R = crate::R<bool, SFE_A>;
impl SFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFE_A {
        match self.bits {
            true => SFE_A::EN,
            false => SFE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SFE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SFE_A::DIS
    }
}
#[doc = "Write proxy for field `SFE`"]
pub struct SFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SFE_A::EN)
    }
    #[doc = "Slave mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SFE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "4:4\\]
I2C master function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MFE_A {
    #[doc = "1: Master mode is enabled."]
    EN = 1,
    #[doc = "0: Master mode is disabled."]
    DIS = 0,
}
impl From<MFE_A> for bool {
    #[inline(always)]
    fn from(variant: MFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MFE`"]
pub type MFE_R = crate::R<bool, MFE_A>;
impl MFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFE_A {
        match self.bits {
            true => MFE_A::EN,
            false => MFE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MFE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MFE_A::DIS
    }
}
#[doc = "Write proxy for field `MFE`"]
pub struct MFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master mode is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MFE_A::EN)
    }
    #[doc = "Master mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MFE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBK_A {
    #[doc = "1: Enable Test Mode"]
    EN = 1,
    #[doc = "0: Disable Test Mode"]
    DIS = 0,
}
impl From<LPBK_A> for bool {
    #[inline(always)]
    fn from(variant: LPBK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPBK`"]
pub type LPBK_R = crate::R<bool, LPBK_A>;
impl LPBK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBK_A {
        match self.bits {
            true => LPBK_A::EN,
            false => LPBK_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LPBK_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LPBK_A::DIS
    }
}
#[doc = "Write proxy for field `LPBK`"]
pub struct LPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPBK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Test Mode"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LPBK_A::EN)
    }
    #[doc = "Disable Test Mode"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LPBK_A::DIS)
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
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C slave function enable"]
    #[inline(always)]
    pub fn sfe(&self) -> SFE_R {
        SFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
I2C master function enable"]
    #[inline(always)]
    pub fn mfe(&self) -> MFE_R {
        MFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new((self.bits & 0x01) != 0)
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
I2C slave function enable"]
    #[inline(always)]
    pub fn sfe(&mut self) -> SFE_W {
        SFE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
I2C master function enable"]
    #[inline(always)]
    pub fn mfe(&mut self) -> MFE_W {
        MFE_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
I2C loopback 0: Normal operation 1: Loopback operation (test mode)"]
    #[inline(always)]
    pub fn lpbk(&mut self) -> LPBK_W {
        LPBK_W { w: self }
    }
}
