#[doc = "Reader of register MEM_CTL"]
pub type R = crate::R<u32, super::MEM_CTL>;
#[doc = "Writer for register MEM_CTL"]
pub type W = crate::W<u32, super::MEM_CTL>;
#[doc = "Register MEM_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `MEM_BUSY`"]
pub type MEM_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_BUSY`"]
pub struct MEM_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MEM_CLR_EN`"]
pub type MEM_CLR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_CLR_EN`"]
pub struct MEM_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CLR_EN_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
    #[inline(always)]
    pub fn mem_busy(&self) -> MEM_BUSY_R {
        MEM_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
    #[inline(always)]
    pub fn mem_clr_en(&self) -> MEM_CLR_EN_R {
        MEM_CLR_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Busy status 0: Memory accepts transfers 1: Memory controller is busy during initialization. Read and write transfers are not performed."]
    #[inline(always)]
    pub fn mem_busy(&mut self) -> MEM_BUSY_W {
        MEM_BUSY_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Memory Contents Initialization enable Writing 1 to MEM_CLR_EN will start memory initialization. The contents of all byte locations will be initialized to 0x00. MEM_BUSY will be 1 until memory initialization has completed."]
    #[inline(always)]
    pub fn mem_clr_en(&mut self) -> MEM_CLR_EN_W {
        MEM_CLR_EN_W { w: self }
    }
}
