#[doc = "Reader of register DMACH0LEN"]
pub type R = crate::R<u32, super::DMACH0LEN>;
#[doc = "Writer for register DMACH0LEN"]
pub type W = crate::W<u32, super::DMACH0LEN>;
#[doc = "Register DMACH0LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACH0LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMALEN`"]
pub type DMALEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMALEN`"]
pub struct DMALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&self) -> DMALEN_R {
        DMALEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&mut self) -> DMALEN_W {
        DMALEN_W { w: self }
    }
}
