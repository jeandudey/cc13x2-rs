#[doc = "Reader of register EEPROM_CFG"]
pub type R = crate::R<u32, super::EEPROM_CFG>;
#[doc = "Writer for register EEPROM_CFG"]
pub type W = crate::W<u32, super::EEPROM_CFG>;
#[doc = "Register EEPROM_CFG `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::EEPROM_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `AUTOSTART_GRACE`"]
pub type AUTOSTART_GRACE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AUTOSTART_GRACE`"]
pub struct AUTOSTART_GRACE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTART_GRACE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn autostart_grace(&self) -> AUTOSTART_GRACE_R {
        AUTOSTART_GRACE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn autostart_grace(&mut self) -> AUTOSTART_GRACE_W {
        AUTOSTART_GRACE_W { w: self }
    }
}
