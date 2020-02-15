#[doc = "Reader of register ICEPICK_DEVICE_ID"]
pub type R = crate::R<u32, super::ICEPICK_DEVICE_ID>;
#[doc = "Writer for register ICEPICK_DEVICE_ID"]
pub type W = crate::W<u32, super::ICEPICK_DEVICE_ID>;
#[doc = "Register ICEPICK_DEVICE_ID `reset()`'s with value 0x3bb4_102f"]
impl crate::ResetValue for super::ICEPICK_DEVICE_ID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3bb4_102f
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
#[doc = "Reader of field `WAFER_ID`"]
pub type WAFER_ID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WAFER_ID`"]
pub struct WAFER_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> WAFER_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `MANUFACTURER_ID`"]
pub type MANUFACTURER_ID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MANUFACTURER_ID`"]
pub struct MANUFACTURER_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> MANUFACTURER_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    pub fn wafer_id(&self) -> WAFER_ID_R {
        WAFER_ID_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> MANUFACTURER_ID_R {
        MANUFACTURER_ID_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&mut self) -> PG_REV_W {
        PG_REV_W { w: self }
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    pub fn wafer_id(&mut self) -> WAFER_ID_W {
        WAFER_ID_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    pub fn manufacturer_id(&mut self) -> MANUFACTURER_ID_W {
        MANUFACTURER_ID_W { w: self }
    }
}
