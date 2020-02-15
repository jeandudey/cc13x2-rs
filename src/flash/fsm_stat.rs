#[doc = "Reader of register FSM_STAT"]
pub type R = crate::R<u32, super::FSM_STAT>;
#[doc = "Writer for register FSM_STAT"]
pub type W = crate::W<u32, super::FSM_STAT>;
#[doc = "Register FSM_STAT `reset()`'s with value 0x04"]
impl crate::ResetValue for super::FSM_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
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
#[doc = "Reader of field `NON_OP`"]
pub type NON_OP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NON_OP`"]
pub struct NON_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> NON_OP_W<'a> {
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
#[doc = "Reader of field `OVR_PUL_CNT`"]
pub type OVR_PUL_CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR_PUL_CNT`"]
pub struct OVR_PUL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_PUL_CNT_W<'a> {
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
#[doc = "Reader of field `INV_DAT`"]
pub type INV_DAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV_DAT`"]
pub struct INV_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_DAT_W<'a> {
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn non_op(&self) -> NON_OP_R {
        NON_OP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr_pul_cnt(&self) -> OVR_PUL_CNT_R {
        OVR_PUL_CNT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_dat(&self) -> INV_DAT_R {
        INV_DAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn non_op(&mut self) -> NON_OP_W {
        NON_OP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr_pul_cnt(&mut self) -> OVR_PUL_CNT_W {
        OVR_PUL_CNT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_dat(&mut self) -> INV_DAT_W {
        INV_DAT_W { w: self }
    }
}
