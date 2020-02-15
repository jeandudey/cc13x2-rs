#[doc = "Reader of register STIM1"]
pub type R = crate::R<u32, super::STIM1>;
#[doc = "Writer for register STIM1"]
pub type W = crate::W<u32, super::STIM1>;
#[doc = "Register STIM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM1`"]
pub type STIM1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM1`"]
pub struct STIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim1(&self) -> STIM1_R {
        STIM1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim1(&mut self) -> STIM1_W {
        STIM1_W { w: self }
    }
}
