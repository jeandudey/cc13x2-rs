#[doc = "Reader of register FLASH_VHV_E"]
pub type R = crate::R<u32, super::FLASH_VHV_E>;
#[doc = "Writer for register FLASH_VHV_E"]
pub type W = crate::W<u32, super::FLASH_VHV_E>;
#[doc = "Register FLASH_VHV_E `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FLASH_VHV_E {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `VHV_E_START`"]
pub type VHV_E_START_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VHV_E_START`"]
pub struct VHV_E_START_W<'a> {
    w: &'a mut W,
}
impl<'a> VHV_E_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VHV_E_STEP_HIGHT`"]
pub type VHV_E_STEP_HIGHT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VHV_E_STEP_HIGHT`"]
pub struct VHV_E_STEP_HIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> VHV_E_STEP_HIGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_start(&self) -> VHV_E_START_R {
        VHV_E_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_step_hight(&self) -> VHV_E_STEP_HIGHT_R {
        VHV_E_STEP_HIGHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_start(&mut self) -> VHV_E_START_W {
        VHV_E_START_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_e_step_hight(&mut self) -> VHV_E_STEP_HIGHT_W {
        VHV_E_STEP_HIGHT_W { w: self }
    }
}
