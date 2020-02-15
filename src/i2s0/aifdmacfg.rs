#[doc = "Reader of register AIFDMACFG"]
pub type R = crate::R<u32, super::AIFDMACFG>;
#[doc = "Writer for register AIFDMACFG"]
pub type W = crate::W<u32, super::AIFDMACFG>;
#[doc = "Register AIFDMACFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFDMACFG {
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
#[doc = "Reader of field `END_FRAME_IDX`"]
pub type END_FRAME_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `END_FRAME_IDX`"]
pub struct END_FRAME_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> END_FRAME_IDX_W<'a> {
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
Defines the length of the DMA buffer. Writing a non-zero value to this register field enables and initializes AIF. Note that before doing so, all other configuration must have been done, and AIFINPTRNEXT/AIFOUTPTRNEXT must have been loaded."]
    #[inline(always)]
    pub fn end_frame_idx(&self) -> END_FRAME_IDX_R {
        END_FRAME_IDX_R::new((self.bits & 0xff) as u8)
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
Defines the length of the DMA buffer. Writing a non-zero value to this register field enables and initializes AIF. Note that before doing so, all other configuration must have been done, and AIFINPTRNEXT/AIFOUTPTRNEXT must have been loaded."]
    #[inline(always)]
    pub fn end_frame_idx(&mut self) -> END_FRAME_IDX_W {
        END_FRAME_IDX_W { w: self }
    }
}
