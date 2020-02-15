#[doc = "Reader of register LDO_TRIM"]
pub type R = crate::R<u32, super::LDO_TRIM>;
#[doc = "Writer for register LDO_TRIM"]
pub type W = crate::W<u32, super::LDO_TRIM>;
#[doc = "Register LDO_TRIM `reset()`'s with value 0xe0f8_e0fb"]
impl crate::ResetValue for super::LDO_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe0f8_e0fb
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
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `VDDR_TRIM_SLEEP`"]
pub type VDDR_TRIM_SLEEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_TRIM_SLEEP`"]
pub struct VDDR_TRIM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `GLDO_CURSRC`"]
pub type GLDO_CURSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GLDO_CURSRC`"]
pub struct GLDO_CURSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> GLDO_CURSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `ITRIM_DIGLDO_LOAD`"]
pub type ITRIM_DIGLDO_LOAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITRIM_DIGLDO_LOAD`"]
pub struct ITRIM_DIGLDO_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_DIGLDO_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `ITRIM_UDIGLDO`"]
pub type ITRIM_UDIGLDO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITRIM_UDIGLDO`"]
pub struct ITRIM_UDIGLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_UDIGLDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `VTRIM_DELTA`"]
pub type VTRIM_DELTA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VTRIM_DELTA`"]
pub struct VTRIM_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep(&self) -> VDDR_TRIM_SLEEP_R {
        VDDR_TRIM_SLEEP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldo_cursrc(&self) -> GLDO_CURSRC_R {
        GLDO_CURSRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_digldo_load(&self) -> ITRIM_DIGLDO_LOAD_R {
        ITRIM_DIGLDO_LOAD_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_udigldo(&self) -> ITRIM_UDIGLDO_R {
        ITRIM_UDIGLDO_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_delta(&self) -> VTRIM_DELTA_R {
        VTRIM_DELTA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep(&mut self) -> VDDR_TRIM_SLEEP_W {
        VDDR_TRIM_SLEEP_W { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldo_cursrc(&mut self) -> GLDO_CURSRC_W {
        GLDO_CURSRC_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_digldo_load(&mut self) -> ITRIM_DIGLDO_LOAD_W {
        ITRIM_DIGLDO_LOAD_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_udigldo(&mut self) -> ITRIM_UDIGLDO_W {
        ITRIM_UDIGLDO_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_delta(&mut self) -> VTRIM_DELTA_W {
        VTRIM_DELTA_W { w: self }
    }
}
