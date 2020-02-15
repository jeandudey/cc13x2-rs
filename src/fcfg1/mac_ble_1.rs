#[doc = "Reader of register MAC_BLE_1"]
pub type R = crate::R<u32, super::MAC_BLE_1>;
#[doc = "Writer for register MAC_BLE_1"]
pub type W = crate::W<u32, super::MAC_BLE_1>;
#[doc = "Register MAC_BLE_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_BLE_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_32_63`"]
pub type ADDR_32_63_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR_32_63`"]
pub struct ADDR_32_63_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_32_63_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The last 32-bits of the 64-bit MAC BLE address"]
    #[inline(always)]
    pub fn addr_32_63(&self) -> ADDR_32_63_R {
        ADDR_32_63_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The last 32-bits of the 64-bit MAC BLE address"]
    #[inline(always)]
    pub fn addr_32_63(&mut self) -> ADDR_32_63_W {
        ADDR_32_63_W { w: self }
    }
}
