#[doc = "Reader of register MPU_CTRL"]
pub type R = crate::R<u32, super::MPU_CTRL>;
#[doc = "Writer for register MPU_CTRL"]
pub type W = crate::W<u32, super::MPU_CTRL>;
#[doc = "Register MPU_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `PRIVDEFENA`"]
pub type PRIVDEFENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIVDEFENA`"]
pub struct PRIVDEFENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVDEFENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `HFNMIENA`"]
pub type HFNMIENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFNMIENA`"]
pub struct HFNMIENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HFNMIENA_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables the default memory map for privileged access, as a background region, when the MPU is enabled. The background region acts as if it was region number 1 before any settable regions. Any region that is set up overlays this default map, and overrides it. If this bit is not set, the default memory map is disabled, and memory not covered by a region faults. This applies to memory type, Execute Never (XN), cache and shareable rules. However, this only applies to privileged mode (fetch and data access). User mode code faults unless a region has been set up for its code and data. When the MPU is disabled, the default map acts on both privileged and user mode code. XN and SO rules always apply to the system partition whether this enable is set or not. If the MPU is disabled, this bit is ignored."]
    #[inline(always)]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W {
        PRIVDEFENA_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables the MPU when in Hard Fault, NMI, and FAULTMASK escalated handlers. If this bit and ENABLE are set, the MPU is enabled when in these handlers. If this bit is not set, the MPU is disabled when in these handlers, regardless of the value of ENABLE bit. If this bit is set and ENABLE is not set, behavior is unpredictable."]
    #[inline(always)]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W {
        HFNMIENA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable MPU 0: MPU disabled 1: MPU enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
