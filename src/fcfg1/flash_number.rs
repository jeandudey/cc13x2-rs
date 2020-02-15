#[doc = "Reader of register FLASH_NUMBER"]
pub type R = crate::R<u32, super::FLASH_NUMBER>;
#[doc = "Writer for register FLASH_NUMBER"]
pub type W = crate::W<u32, super::FLASH_NUMBER>;
#[doc = "Register FLASH_NUMBER `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_NUMBER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOT_NUMBER`"]
pub type LOT_NUMBER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LOT_NUMBER`"]
pub struct LOT_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOT_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub fn lot_number(&self) -> LOT_NUMBER_R {
        LOT_NUMBER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub fn lot_number(&mut self) -> LOT_NUMBER_W {
        LOT_NUMBER_W { w: self }
    }
}
