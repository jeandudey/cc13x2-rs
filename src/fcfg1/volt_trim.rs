#[doc = "Reader of register VOLT_TRIM"]
pub type R = crate::R<u32, super::VOLT_TRIM>;
#[doc = "Writer for register VOLT_TRIM"]
pub type W = crate::W<u32, super::VOLT_TRIM>;
#[doc = "Register VOLT_TRIM `reset()`'s with value 0xe0e0_e0e0"]
impl crate::ResetValue for super::VOLT_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe0e0_e0e0
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
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `VDDR_TRIM_HH`"]
pub type VDDR_TRIM_HH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_TRIM_HH`"]
pub struct VDDR_TRIM_HH_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_HH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
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
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `VDDR_TRIM_H`"]
pub type VDDR_TRIM_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_TRIM_H`"]
pub struct VDDR_TRIM_H_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
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
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `VDDR_TRIM_SLEEP_H`"]
pub type VDDR_TRIM_SLEEP_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_TRIM_SLEEP_H`"]
pub struct VDDR_TRIM_SLEEP_H_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `TRIMBOD_H`"]
pub type TRIMBOD_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMBOD_H`"]
pub struct TRIMBOD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMBOD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_hh(&self) -> VDDR_TRIM_HH_R {
        VDDR_TRIM_HH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_h(&self) -> VDDR_TRIM_H_R {
        VDDR_TRIM_H_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&self) -> VDDR_TRIM_SLEEP_H_R {
        VDDR_TRIM_SLEEP_H_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_h(&self) -> TRIMBOD_H_R {
        TRIMBOD_H_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_hh(&mut self) -> VDDR_TRIM_HH_W {
        VDDR_TRIM_HH_W { w: self }
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_h(&mut self) -> VDDR_TRIM_H_W {
        VDDR_TRIM_H_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&mut self) -> VDDR_TRIM_SLEEP_H_W {
        VDDR_TRIM_SLEEP_H_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_h(&mut self) -> TRIMBOD_H_W {
        TRIMBOD_H_W { w: self }
    }
}
