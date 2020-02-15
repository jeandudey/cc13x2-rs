#[doc = "Reader of register IEEE_BLE_0"]
pub type R = crate::R<u32, super::IEEE_BLE_0>;
#[doc = "Writer for register IEEE_BLE_0"]
pub type W = crate::W<u32, super::IEEE_BLE_0>;
#[doc = "Register IEEE_BLE_0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IEEE_BLE_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[31:0\\]
of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[31:0\\]
of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
