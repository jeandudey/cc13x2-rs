#[doc = "Reader of register DACSMPLCFG1"]
pub type R = crate::R<u32, super::DACSMPLCFG1>;
#[doc = "Writer for register DACSMPLCFG1"]
pub type W = crate::W<u32, super::DACSMPLCFG1>;
#[doc = "Register DACSMPLCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DACSMPLCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED15`"]
pub type RESERVED15_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED15`"]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 15)) | (((value as u32) & 0x0001_ffff) << 15);
        self.w
    }
}
#[doc = "Reader of field `H_PER`"]
pub type H_PER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `H_PER`"]
pub struct H_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> H_PER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `L_PER`"]
pub type L_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `L_PER`"]
pub struct L_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> L_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SETUP_CNT`"]
pub type SETUP_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SETUP_CNT`"]
pub struct SETUP_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOLD_INTERVAL`"]
pub type HOLD_INTERVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOLD_INTERVAL`"]
pub struct HOLD_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 14 - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
    #[inline(always)]
    pub fn h_per(&self) -> H_PER_R {
        H_PER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
    #[inline(always)]
    pub fn l_per(&self) -> L_PER_R {
        L_PER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
    #[inline(always)]
    pub fn setup_cnt(&self) -> SETUP_CNT_R {
        SETUP_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
    #[inline(always)]
    pub fn hold_interval(&self) -> HOLD_INTERVAL_R {
        HOLD_INTERVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
High time. The sample clock period is high for this many base periods. 0: 2 periods 1: 4 periods"]
    #[inline(always)]
    pub fn h_per(&mut self) -> H_PER_W {
        H_PER_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Low time. The sample clock period is low for this many base periods. 0: 1 period 1: 2 periods 2: 3 periods 3: 4 periods"]
    #[inline(always)]
    pub fn l_per(&mut self) -> L_PER_W {
        L_PER_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Setup count. Number of active sample clock periods during the setup phase. 0: 1 sample clock period 1: 2 sample clock periods ... 15 : 16 sample clock periods"]
    #[inline(always)]
    pub fn setup_cnt(&mut self) -> SETUP_CNT_W {
        SETUP_CNT_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Hold interval. Number of inactive sample clock periods between each active sample clock period during hold phase. The sample clock is low when inactive. The range is 0 to 255."]
    #[inline(always)]
    pub fn hold_interval(&mut self) -> HOLD_INTERVAL_W {
        HOLD_INTERVAL_W { w: self }
    }
}
