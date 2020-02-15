#[doc = "Reader of register AIFINPTRNEXT"]
pub type R = crate::R<u32, super::AIFINPTRNEXT>;
#[doc = "Writer for register AIFINPTRNEXT"]
pub type W = crate::W<u32, super::AIFINPTRNEXT>;
#[doc = "Register AIFINPTRNEXT `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFINPTRNEXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTR`"]
pub type PTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PTR`"]
pub struct PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
    #[inline(always)]
    pub fn ptr(&mut self) -> PTR_W {
        PTR_W { w: self }
    }
}
