#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
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
#[doc = "Reader of field `IDCODE_LB_DIS`"]
pub type IDCODE_LB_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDCODE_LB_DIS`"]
pub struct IDCODE_LB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCODE_LB_DIS_W<'a> {
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
#[doc = "Reader of field `SYSBUS_LB_DIS`"]
pub type SYSBUS_LB_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBUS_LB_DIS`"]
pub struct SYSBUS_LB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBUS_LB_DIS_W<'a> {
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
#[doc = "Reader of field `MODE_CHANGING`"]
pub type MODE_CHANGING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE_CHANGING`"]
pub struct MODE_CHANGING_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_CHANGING_W<'a> {
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
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV`"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
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
#[doc = "1:0\\]
Current VIMS mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: VIMS Off mode"]
    OFF = 3,
    #[doc = "1: VIMS Cache mode"]
    CACHE = 1,
    #[doc = "0: VIMS GPRAM mode"]
    GPRAM = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(MODE_A::OFF),
            1 => Val(MODE_A::CACHE),
            0 => Val(MODE_A::GPRAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `CACHE`"]
    #[inline(always)]
    pub fn is_cache(&self) -> bool {
        *self == MODE_A::CACHE
    }
    #[doc = "Checks if the value of the field is `GPRAM`"]
    #[inline(always)]
    pub fn is_gpram(&self) -> bool {
        *self == MODE_A::GPRAM
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VIMS Off mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = "VIMS Cache mode"]
    #[inline(always)]
    pub fn cache(self) -> &'a mut W {
        self.variant(MODE_A::CACHE)
    }
    #[doc = "VIMS GPRAM mode"]
    #[inline(always)]
    pub fn gpram(self) -> &'a mut W {
        self.variant(MODE_A::GPRAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IDCODE_LB_DIS_R {
        IDCODE_LB_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SYSBUS_LB_DIS_R {
        SYSBUS_LB_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
    #[inline(always)]
    pub fn mode_changing(&self) -> MODE_CHANGING_R {
        MODE_CHANGING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Current VIMS mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
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
Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn idcode_lb_dis(&mut self) -> IDCODE_LB_DIS_W {
        IDCODE_LB_DIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&mut self) -> SYSBUS_LB_DIS_W {
        SYSBUS_LB_DIS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE"]
    #[inline(always)]
    pub fn mode_changing(&mut self) -> MODE_CHANGING_W {
        MODE_CHANGING_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is set when invalidation of the cache memory is active / ongoing"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Current VIMS mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
