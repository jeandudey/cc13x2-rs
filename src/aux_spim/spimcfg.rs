#[doc = "Reader of register SPIMCFG"]
pub type R = crate::R<u32, super::SPIMCFG>;
#[doc = "Writer for register SPIMCFG"]
pub type W = crate::W<u32, super::SPIMCFG>;
#[doc = "Register SPIMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPIMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `PHA`"]
pub type PHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHA`"]
pub struct PHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHA_W<'a> {
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
#[doc = "Reader of field `POL`"]
pub type POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL`"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 2:7 - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
SCLK divider. Peripheral clock frequency division gives the SCLK clock frequency. The division factor equals (2 * (DIV+1)): 0x00: Divide by 2. 0x01: Divide by 4. 0x02: Divide by 6. ... 0x3F: Divide by 128."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Phase of the MOSI and MISO data signals. 0: Sample MISO at leading (odd) edges and shift MOSI at trailing (even) edges of SCLK. 1: Sample MISO at trailing (even) edges and shift MOSI at leading (odd) edges of SCLK."]
    #[inline(always)]
    pub fn pha(&mut self) -> PHA_W {
        PHA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Polarity of the SCLK signal. 0: SCLK is low when idle, first clock edge rises. 1: SCLK is high when idle, first clock edge falls."]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
}
