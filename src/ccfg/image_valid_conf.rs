#[doc = "Reader of register IMAGE_VALID_CONF"]
pub type R = crate::R<u32, super::IMAGE_VALID_CONF>;
#[doc = "Writer for register IMAGE_VALID_CONF"]
pub type W = crate::W<u32, super::IMAGE_VALID_CONF>;
#[doc = "Register IMAGE_VALID_CONF `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IMAGE_VALID_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `IMAGE_VALID`"]
pub type IMAGE_VALID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IMAGE_VALID`"]
pub struct IMAGE_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAGE_VALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field must have the address value of the start of the flash vector table in order to enable the boot FW in ROM to transfer control to a flash image. Any illegal vector table start address value will force the boot FW in ROM to transfer control to the serial boot loader in ROM."]
    #[inline(always)]
    pub fn image_valid(&self) -> IMAGE_VALID_R {
        IMAGE_VALID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field must have the address value of the start of the flash vector table in order to enable the boot FW in ROM to transfer control to a flash image. Any illegal vector table start address value will force the boot FW in ROM to transfer control to the serial boot loader in ROM."]
    #[inline(always)]
    pub fn image_valid(&mut self) -> IMAGE_VALID_W {
        IMAGE_VALID_W { w: self }
    }
}
