#[doc = "Reader of register REG5_4"]
pub type R = crate::R<u32, super::REG5_4>;
#[doc = "Writer for register REG5_4"]
pub type W = crate::W<u32, super::REG5_4>;
#[doc = "Register REG5_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG5_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG5`"]
pub type REG5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG5`"]
pub struct REG5_W<'a> {
    w: &'a mut W,
}
impl<'a> REG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `REG4`"]
pub type REG4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG4`"]
pub struct REG4_W<'a> {
    w: &'a mut W,
}
impl<'a> REG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg5(&self) -> REG5_R {
        REG5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg4(&self) -> REG4_R {
        REG4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg5(&mut self) -> REG5_W {
        REG5_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg4(&mut self) -> REG4_W {
        REG4_W { w: self }
    }
}
