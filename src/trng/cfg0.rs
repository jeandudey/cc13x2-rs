#[doc = "Reader of register CFG0"]
pub type R = crate::R<u32, super::CFG0>;
#[doc = "Writer for register CFG0"]
pub type W = crate::W<u32, super::CFG0>;
#[doc = "Register CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAX_REFILL_CYCLES`"]
pub type MAX_REFILL_CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_REFILL_CYCLES`"]
pub struct MAX_REFILL_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_REFILL_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SMPL_DIV`"]
pub type SMPL_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPL_DIV`"]
pub struct SMPL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MIN_REFILL_CYCLES`"]
pub type MIN_REFILL_CYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_REFILL_CYCLES`"]
pub struct MIN_REFILL_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_REFILL_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    pub fn max_refill_cycles(&self) -> MAX_REFILL_CYCLES_R {
        MAX_REFILL_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    pub fn smpl_div(&self) -> SMPL_DIV_R {
        SMPL_DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
    #[inline(always)]
    pub fn min_refill_cycles(&self) -> MIN_REFILL_CYCLES_R {
        MIN_REFILL_CYCLES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    pub fn max_refill_cycles(&mut self) -> MAX_REFILL_CYCLES_W {
        MAX_REFILL_CYCLES_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    pub fn smpl_div(&mut self) -> SMPL_DIV_W {
        SMPL_DIV_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
    #[inline(always)]
    pub fn min_refill_cycles(&mut self) -> MIN_REFILL_CYCLES_W {
        MIN_REFILL_CYCLES_W { w: self }
    }
}
