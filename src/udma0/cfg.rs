#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
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
#[doc = "Reader of field `PRTOCTRL`"]
pub type PRTOCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRTOCTRL`"]
pub struct PRTOCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTOCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `MASTERENABLE`"]
pub type MASTERENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTERENABLE`"]
pub struct MASTERENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTERENABLE_W<'a> {
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
    #[doc = "Bits 5:7 - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    pub fn prtoctrl(&self) -> PRTOCTRL_R {
        PRTOCTRL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
    #[inline(always)]
    pub fn masterenable(&self) -> MASTERENABLE_R {
        MASTERENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\]
as follows: Bit \\[7\\]
Controls HProt\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HProt\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HProt\\[1\\]
to indicate if a privileged access is occurring. When bit \\[n\\]
= 1 then the corresponding HProt bit is high. When bit \\[n\\]
= 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\]
signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\]
for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    pub fn prtoctrl(&mut self) -> PRTOCTRL_W {
        PRTOCTRL_W { w: self }
    }
    #[doc = "Bits 1:4 - 4:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables the controller: 0: Disables the controller 1: Enables the controller"]
    #[inline(always)]
    pub fn masterenable(&mut self) -> MASTERENABLE_W {
        MASTERENABLE_W { w: self }
    }
}
