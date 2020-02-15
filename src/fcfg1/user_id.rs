#[doc = "Reader of register USER_ID"]
pub type R = crate::R<u32, super::USER_ID>;
#[doc = "Writer for register USER_ID"]
pub type W = crate::W<u32, super::USER_ID>;
#[doc = "Register USER_ID `reset()`'s with value 0x3000_0000"]
impl crate::ResetValue for super::USER_ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000_0000
    }
}
#[doc = "Reader of field `PG_REV`"]
pub type PG_REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PG_REV`"]
pub struct PG_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VER`"]
pub struct VER_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `PA`"]
pub type PA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA`"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CC13`"]
pub type CC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC13`"]
pub struct CC13_W<'a> {
    w: &'a mut W,
}
impl<'a> CC13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SEQUENCE`"]
pub type SEQUENCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCE`"]
pub struct SEQUENCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `PKG`"]
pub type PKG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PKG`"]
pub struct PKG_W<'a> {
    w: &'a mut W,
}
impl<'a> PKG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `PROTOCOL`"]
pub type PROTOCOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTOCOL`"]
pub struct PROTOCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTOCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device"]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: CC26xx device type 1: CC13xx device type"]
    #[inline(always)]
    pub fn cc13(&self) -> CC13_R {
        CC13_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    pub fn sequence(&self) -> SEQUENCE_R {
        SEQUENCE_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device"]
    #[inline(always)]
    pub fn pg_rev(&mut self) -> PG_REV_W {
        PG_REV_W { w: self }
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    pub fn ver(&mut self) -> VER_W {
        VER_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: CC26xx device type 1: CC13xx device type"]
    #[inline(always)]
    pub fn cc13(&mut self) -> CC13_W {
        CC13_W { w: self }
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    pub fn sequence(&mut self) -> SEQUENCE_W {
        SEQUENCE_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    pub fn pkg(&mut self) -> PKG_W {
        PKG_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    pub fn protocol(&mut self) -> PROTOCOL_W {
        PROTOCOL_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
