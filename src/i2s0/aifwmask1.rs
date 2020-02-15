#[doc = "Reader of register AIFWMASK1"]
pub type R = crate::R<u32, super::AIFWMASK1>;
#[doc = "Writer for register AIFWMASK1"]
pub type W = crate::W<u32, super::AIFWMASK1>;
#[doc = "Register AIFWMASK1 `reset()`'s with value 0x03"]
impl crate::ResetValue for super::AIFWMASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
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
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
    #[doc = "Bits 0:7 - 7:0\\]
Bit-mask indicating valid channels in a frame on AD1. In single-phase mode, each bit represents one channel, starting with LSB for the first word in the frame. A frame can contain up to 8 channels. Channels that are not included in the mask will not be sampled and stored in memory, and clocked out as '0'. In dual-phase mode, only the two LSBs are considered. For a stereo configuration, set both bits. For a mono configuration, set bit 0 only. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated when clocked out. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated in the second phase when clocked out. If all bits are zero, no input words will be stored to memory, and the output data lines will be constant '0'. This can be utilized when PWM debug output is desired without any actively used output pins."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Bit-mask indicating valid channels in a frame on AD1. In single-phase mode, each bit represents one channel, starting with LSB for the first word in the frame. A frame can contain up to 8 channels. Channels that are not included in the mask will not be sampled and stored in memory, and clocked out as '0'. In dual-phase mode, only the two LSBs are considered. For a stereo configuration, set both bits. For a mono configuration, set bit 0 only. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated when clocked out. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated in the second phase when clocked out. If all bits are zero, no input words will be stored to memory, and the output data lines will be constant '0'. This can be utilized when PWM debug output is desired without any actively used output pins."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
