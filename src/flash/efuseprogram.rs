#[doc = "Reader of register EFUSEPROGRAM"]
pub type R = crate::R<u32, super::EFUSEPROGRAM>;
#[doc = "Writer for register EFUSEPROGRAM"]
pub type W = crate::W<u32, super::EFUSEPROGRAM>;
#[doc = "Register EFUSEPROGRAM `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEPROGRAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED31`"]
pub type RESERVED31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED31`"]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `COMPAREDISABLE`"]
pub type COMPAREDISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPAREDISABLE`"]
pub struct COMPAREDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREDISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CLOCKSTALL`"]
pub type CLOCKSTALL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLOCKSTALL`"]
pub struct CLOCKSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKSTALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | (((value as u32) & 0xffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `VPPTOVDD`"]
pub type VPPTOVDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPPTOVDD`"]
pub struct VPPTOVDD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPPTOVDD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ITERATIONS`"]
pub type ITERATIONS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITERATIONS`"]
pub struct ITERATIONS_W<'a> {
    w: &'a mut W,
}
impl<'a> ITERATIONS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `WRITECLOCK`"]
pub type WRITECLOCK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WRITECLOCK`"]
pub struct WRITECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITECLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comparedisable(&self) -> COMPAREDISABLE_R {
        COMPAREDISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clockstall(&self) -> CLOCKSTALL_R {
        CLOCKSTALL_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vpptovdd(&self) -> VPPTOVDD_R {
        VPPTOVDD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iterations(&self) -> ITERATIONS_R {
        ITERATIONS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn writeclock(&self) -> WRITECLOCK_R {
        WRITECLOCK_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comparedisable(&mut self) -> COMPAREDISABLE_W {
        COMPAREDISABLE_W { w: self }
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clockstall(&mut self) -> CLOCKSTALL_W {
        CLOCKSTALL_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vpptovdd(&mut self) -> VPPTOVDD_W {
        VPPTOVDD_W { w: self }
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iterations(&mut self) -> ITERATIONS_W {
        ITERATIONS_W { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn writeclock(&mut self) -> WRITECLOCK_W {
        WRITECLOCK_W { w: self }
    }
}
