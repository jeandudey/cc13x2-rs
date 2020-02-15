#[doc = "Reader of register STCVR"]
pub type R = crate::R<u32, super::STCVR>;
#[doc = "Writer for register STCVR"]
pub type W = crate::W<u32, super::STCVR>;
#[doc = "Register STCVR `reset()`'s with value 0"]
impl crate::ResetValue for super::STCVR {
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
#[doc = "Reader of field `CURRENT`"]
pub type CURRENT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CURRENT`"]
pub struct CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_W<'a> {
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
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new((self.bits & 0x00ff_ffff) as u32)
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
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W {
        CURRENT_W { w: self }
    }
}
