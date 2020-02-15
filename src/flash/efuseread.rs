#[doc = "Reader of register EFUSEREAD"]
pub type R = crate::R<u32, super::EFUSEREAD>;
#[doc = "Writer for register EFUSEREAD"]
pub type W = crate::W<u32, super::EFUSEREAD>;
#[doc = "Register EFUSEREAD `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEREAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `DATABIT`"]
pub type DATABIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATABIT`"]
pub struct DATABIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `READCLOCK`"]
pub type READCLOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `READCLOCK`"]
pub struct READCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> READCLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DEBUG`"]
pub type DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUG`"]
pub struct DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_W<'a> {
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
#[doc = "Reader of field `SPARE`"]
pub type SPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
#[doc = "Reader of field `MARGIN`"]
pub type MARGIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MARGIN`"]
pub struct MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn databit(&self) -> DATABIT_R {
        DATABIT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn readclock(&self) -> READCLOCK_R {
        READCLOCK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn debug(&self) -> DEBUG_R {
        DEBUG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn margin(&self) -> MARGIN_R {
        MARGIN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn databit(&mut self) -> DATABIT_W {
        DATABIT_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn readclock(&mut self) -> READCLOCK_W {
        READCLOCK_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn debug(&mut self) -> DEBUG_W {
        DEBUG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn margin(&mut self) -> MARGIN_W {
        MARGIN_W { w: self }
    }
}
