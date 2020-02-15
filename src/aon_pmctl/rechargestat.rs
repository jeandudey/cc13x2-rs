#[doc = "Reader of register RECHARGESTAT"]
pub type R = crate::R<u32, super::RECHARGESTAT>;
#[doc = "Writer for register RECHARGESTAT"]
pub type W = crate::W<u32, super::RECHARGESTAT>;
#[doc = "Register RECHARGESTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::RECHARGESTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `VDDR_SMPLS`"]
pub type VDDR_SMPLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_SMPLS`"]
pub struct VDDR_SMPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_SMPLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_USED_PER`"]
pub type MAX_USED_PER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_USED_PER`"]
pub struct MAX_USED_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_USED_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
    #[inline(always)]
    pub fn vddr_smpls(&self) -> VDDR_SMPLS_R {
        VDDR_SMPLS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn max_used_per(&self) -> MAX_USED_PER_R {
        MAX_USED_PER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
    #[inline(always)]
    pub fn vddr_smpls(&mut self) -> VDDR_SMPLS_W {
        VDDR_SMPLS_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn max_used_per(&mut self) -> MAX_USED_PER_W {
        MAX_USED_PER_W { w: self }
    }
}
