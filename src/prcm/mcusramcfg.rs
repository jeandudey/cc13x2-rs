#[doc = "Reader of register MCUSRAMCFG"]
pub type R = crate::R<u32, super::MCUSRAMCFG>;
#[doc = "Writer for register MCUSRAMCFG"]
pub type W = crate::W<u32, super::MCUSRAMCFG>;
#[doc = "Register MCUSRAMCFG `reset()`'s with value 0x20"]
impl crate::ResetValue for super::MCUSRAMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
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
#[doc = "Reader of field `BM_OFF`"]
pub type BM_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BM_OFF`"]
pub struct BM_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BM_OFF_W<'a> {
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
#[doc = "Reader of field `PAGE`"]
pub type PAGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAGE`"]
pub struct PAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE_W<'a> {
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
#[doc = "Reader of field `PGS`"]
pub type PGS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGS`"]
pub struct PGS_W<'a> {
    w: &'a mut W,
}
impl<'a> PGS_W<'a> {
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
#[doc = "Reader of field `BM`"]
pub type BM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BM`"]
pub struct BM_W<'a> {
    w: &'a mut W,
}
impl<'a> BM_W<'a> {
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
#[doc = "Reader of field `PCH_F`"]
pub type PCH_F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCH_F`"]
pub struct PCH_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PCH_F_W<'a> {
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
#[doc = "Reader of field `PCH_L`"]
pub type PCH_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCH_L`"]
pub struct PCH_L_W<'a> {
    w: &'a mut W,
}
impl<'a> PCH_L_W<'a> {
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
Burst Mode disable 0: Burst Mode enabled. 1: Burst Mode off."]
    #[inline(always)]
    pub fn bm_off(&self) -> BM_OFF_R {
        BM_OFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Page Mode select 0: Page Mode disabled. Memory works in standard mode 1: Page Mode enabled. Only one half of butterfly array selected. Page Mode will select either LSB half or MSB half of the word based on PGS setting. This mode can be used for additional power saving"]
    #[inline(always)]
    pub fn page(&self) -> PAGE_R {
        PAGE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Select LSB half of word during Page Mode, PAGE = 1 1: Select MSB half of word during Page Mode, PAGE = 1"]
    #[inline(always)]
    pub fn pgs(&self) -> PGS_R {
        PGS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Burst Mode Enable 0: Burst Mode Disable. Memory works in standard mode. 1: Burst Mode Enable When in Burst Mode bitline precharge and wordline firing depends on PCH_F and PCH_L. Burst Mode results in reduction in active power."]
    #[inline(always)]
    pub fn bm(&self) -> BM_R {
        BM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No bitline precharge in second half of cycle 1: Bitline precharge in second half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    pub fn pch_f(&self) -> PCH_F_R {
        PCH_F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: No bitline precharge in first half of cycle 1: Bitline precharge in first half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    pub fn pch_l(&self) -> PCH_L_R {
        PCH_L_R::new((self.bits & 0x01) != 0)
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
Burst Mode disable 0: Burst Mode enabled. 1: Burst Mode off."]
    #[inline(always)]
    pub fn bm_off(&mut self) -> BM_OFF_W {
        BM_OFF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Page Mode select 0: Page Mode disabled. Memory works in standard mode 1: Page Mode enabled. Only one half of butterfly array selected. Page Mode will select either LSB half or MSB half of the word based on PGS setting. This mode can be used for additional power saving"]
    #[inline(always)]
    pub fn page(&mut self) -> PAGE_W {
        PAGE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Select LSB half of word during Page Mode, PAGE = 1 1: Select MSB half of word during Page Mode, PAGE = 1"]
    #[inline(always)]
    pub fn pgs(&mut self) -> PGS_W {
        PGS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Burst Mode Enable 0: Burst Mode Disable. Memory works in standard mode. 1: Burst Mode Enable When in Burst Mode bitline precharge and wordline firing depends on PCH_F and PCH_L. Burst Mode results in reduction in active power."]
    #[inline(always)]
    pub fn bm(&mut self) -> BM_W {
        BM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: No bitline precharge in second half of cycle 1: Bitline precharge in second half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    pub fn pch_f(&mut self) -> PCH_F_W {
        PCH_F_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: No bitline precharge in first half of cycle 1: Bitline precharge in first half of cycle when in Burst Mode, BM = 1"]
    #[inline(always)]
    pub fn pch_l(&mut self) -> PCH_L_W {
        PCH_L_W { w: self }
    }
}
