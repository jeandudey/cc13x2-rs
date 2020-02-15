#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATS_CLR`"]
pub type STATS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATS_CLR`"]
pub struct STATS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STATS_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `STATS_EN`"]
pub type STATS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATS_EN`"]
pub struct STATS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DYN_CG_EN`"]
pub type DYN_CG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DYN_CG_EN`"]
pub struct DYN_CG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_CG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
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
        self.w.bits = (self.w.bits & !(0x007f_ffff << 6)) | (((value as u32) & 0x007f_ffff) << 6);
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
#[doc = "Reader of field `ARB_CFG`"]
pub type ARB_CFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARB_CFG`"]
pub struct ARB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_CFG_W<'a> {
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
#[doc = "Reader of field `PREF_EN`"]
pub type PREF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREF_EN`"]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
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
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1.\n\nValue on reset: 0"]
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
    #[doc = "Bit 31 - 31:31\\]
Set this bit to clear statistic counters."]
    #[inline(always)]
    pub fn stats_clr(&self) -> STATS_CLR_R {
        STATS_CLR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Set this bit to enable statistic counters."]
    #[inline(always)]
    pub fn stats_en(&self) -> STATS_EN_R {
        STATS_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    pub fn dyn_cg_en(&self) -> DYN_CG_EN_R {
        DYN_CG_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 6:28 - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IDCODE_LB_DIS_R {
        IDCODE_LB_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SYSBUS_LB_DIS_R {
        SYSBUS_LB_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ARB_CFG_R {
        ARB_CFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Set this bit to clear statistic counters."]
    #[inline(always)]
    pub fn stats_clr(&mut self) -> STATS_CLR_W {
        STATS_CLR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Set this bit to enable statistic counters."]
    #[inline(always)]
    pub fn stats_en(&mut self) -> STATS_EN_W {
        STATS_EN_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    pub fn dyn_cg_en(&mut self) -> DYN_CG_EN_W {
        DYN_CG_EN_W { w: self }
    }
    #[doc = "Bits 6:28 - 28:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn idcode_lb_dis(&mut self) -> IDCODE_LB_DIS_W {
        IDCODE_LB_DIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&mut self) -> SYSBUS_LB_DIS_W {
        SYSBUS_LB_DIS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline(always)]
    pub fn arb_cfg(&mut self) -> ARB_CFG_W {
        ARB_CFG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
