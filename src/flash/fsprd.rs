#[doc = "Reader of register FSPRD"]
pub type R = crate::R<u32, super::FSPRD>;
#[doc = "Writer for register FSPRD"]
pub type W = crate::W<u32, super::FSPRD>;
#[doc = "Register FSPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::FSPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIS_PREEMPT`"]
pub type DIS_PREEMPT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIS_PREEMPT`"]
pub struct DIS_PREEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_PREEMPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RMBSEM`"]
pub type RMBSEM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMBSEM`"]
pub struct RMBSEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMBSEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `RM1`"]
pub type RM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RM1`"]
pub struct RM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RM1_W<'a> {
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
#[doc = "Reader of field `RM0`"]
pub type RM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RM0`"]
pub struct RM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RM0_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_preempt(&self) -> DIS_PREEMPT_R {
        DIS_PREEMPT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rmbsem(&self) -> RMBSEM_R {
        RMBSEM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm1(&self) -> RM1_R {
        RM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm0(&self) -> RM0_R {
        RM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_preempt(&mut self) -> DIS_PREEMPT_W {
        DIS_PREEMPT_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rmbsem(&mut self) -> RMBSEM_W {
        RMBSEM_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm1(&mut self) -> RM1_W {
        RM1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm0(&mut self) -> RM0_W {
        RM0_W { w: self }
    }
}
