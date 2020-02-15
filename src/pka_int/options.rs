#[doc = "Reader of register OPTIONS"]
pub type R = crate::R<u32, super::OPTIONS>;
#[doc = "Writer for register OPTIONS"]
pub type W = crate::W<u32, super::OPTIONS>;
#[doc = "Register OPTIONS `reset()`'s with value 0x0102"]
impl crate::ResetValue for super::OPTIONS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0102
    }
}
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
#[doc = "Reader of field `AIC_PRESENT`"]
pub type AIC_PRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIC_PRESENT`"]
pub struct AIC_PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> AIC_PRESENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `EIP76_PRESENT`"]
pub type EIP76_PRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIP76_PRESENT`"]
pub struct EIP76_PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EIP76_PRESENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EIP28_PRESENT`"]
pub type EIP28_PRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIP28_PRESENT`"]
pub struct EIP28_PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EIP28_PRESENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `AXI_INTERFACE`"]
pub type AXI_INTERFACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_INTERFACE`"]
pub struct AXI_INTERFACE_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_INTERFACE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AHB_IS_ASYNC`"]
pub type AHB_IS_ASYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_IS_ASYNC`"]
pub struct AHB_IS_ASYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_IS_ASYNC_W<'a> {
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
#[doc = "Reader of field `AHB_INTERFACE`"]
pub type AHB_INTERFACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_INTERFACE`"]
pub struct AHB_INTERFACE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_INTERFACE_W<'a> {
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
#[doc = "Reader of field `PLB_INTERFACE`"]
pub type PLB_INTERFACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLB_INTERFACE`"]
pub struct PLB_INTERFACE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLB_INTERFACE_W<'a> {
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
    #[doc = "Bits 11:31 - 31:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 10 - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
    #[inline(always)]
    pub fn aic_present(&self) -> AIC_PRESENT_R {
        AIC_PRESENT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
    #[inline(always)]
    pub fn eip76_present(&self) -> EIP76_PRESENT_R {
        EIP76_PRESENT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
    #[inline(always)]
    pub fn eip28_present(&self) -> EIP28_PRESENT_R {
        EIP28_PRESENT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
    #[inline(always)]
    pub fn axi_interface(&self) -> AXI_INTERFACE_R {
        AXI_INTERFACE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
    #[inline(always)]
    pub fn ahb_is_async(&self) -> AHB_IS_ASYNC_R {
        AHB_IS_ASYNC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
    #[inline(always)]
    pub fn ahb_interface(&self) -> AHB_INTERFACE_R {
        AHB_INTERFACE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
    #[inline(always)]
    pub fn plb_interface(&self) -> PLB_INTERFACE_R {
        PLB_INTERFACE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:31 - 31:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
    #[inline(always)]
    pub fn aic_present(&mut self) -> AIC_PRESENT_W {
        AIC_PRESENT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
    #[inline(always)]
    pub fn eip76_present(&mut self) -> EIP76_PRESENT_W {
        EIP76_PRESENT_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
    #[inline(always)]
    pub fn eip28_present(&mut self) -> EIP28_PRESENT_W {
        EIP28_PRESENT_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
    #[inline(always)]
    pub fn axi_interface(&mut self) -> AXI_INTERFACE_W {
        AXI_INTERFACE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
    #[inline(always)]
    pub fn ahb_is_async(&mut self) -> AHB_IS_ASYNC_W {
        AHB_IS_ASYNC_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
    #[inline(always)]
    pub fn ahb_interface(&mut self) -> AHB_INTERFACE_W {
        AHB_INTERFACE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
    #[inline(always)]
    pub fn plb_interface(&mut self) -> PLB_INTERFACE_W {
        PLB_INTERFACE_W { w: self }
    }
}
