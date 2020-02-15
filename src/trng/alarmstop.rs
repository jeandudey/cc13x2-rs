#[doc = "Reader of register ALARMSTOP"]
pub type R = crate::R<u32, super::ALARMSTOP>;
#[doc = "Writer for register ALARMSTOP"]
pub type W = crate::W<u32, super::ALARMSTOP>;
#[doc = "Register ALARMSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::ALARMSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `FRO_FLAGS`"]
pub type FRO_FLAGS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRO_FLAGS`"]
pub struct FRO_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_FLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    pub fn fro_flags(&self) -> FRO_FLAGS_R {
        FRO_FLAGS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    pub fn fro_flags(&mut self) -> FRO_FLAGS_W {
        FRO_FLAGS_W { w: self }
    }
}
