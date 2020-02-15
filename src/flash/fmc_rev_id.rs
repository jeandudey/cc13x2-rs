#[doc = "Reader of register FMC_REV_ID"]
pub type R = crate::R<u32, super::FMC_REV_ID>;
#[doc = "Writer for register FMC_REV_ID"]
pub type W = crate::W<u32, super::FMC_REV_ID>;
#[doc = "Register FMC_REV_ID `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_REV_ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOD_VERSION`"]
pub type MOD_VERSION_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MOD_VERSION`"]
pub struct MOD_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `CONFIG_CRC`"]
pub type CONFIG_CRC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONFIG_CRC`"]
pub struct CONFIG_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_CRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mod_version(&self) -> MOD_VERSION_R {
        MOD_VERSION_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn config_crc(&self) -> CONFIG_CRC_R {
        CONFIG_CRC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mod_version(&mut self) -> MOD_VERSION_W {
        MOD_VERSION_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn config_crc(&mut self) -> CONFIG_CRC_W {
        CONFIG_CRC_W { w: self }
    }
}
