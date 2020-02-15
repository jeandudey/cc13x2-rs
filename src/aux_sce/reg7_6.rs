#[doc = "Reader of register REG7_6"]
pub type R = crate::R<u32, super::REG7_6>;
#[doc = "Writer for register REG7_6"]
pub type W = crate::W<u32, super::REG7_6>;
#[doc = "Register REG7_6 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG7_6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG7`"]
pub type REG7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG7`"]
pub struct REG7_W<'a> {
    w: &'a mut W,
}
impl<'a> REG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `REG6`"]
pub type REG6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG6`"]
pub struct REG6_W<'a> {
    w: &'a mut W,
}
impl<'a> REG6_W<'a> {
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
    pub fn reg7(&self) -> REG7_R {
        REG7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg6(&self) -> REG6_R {
        REG6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg7(&mut self) -> REG7_W {
        REG7_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg6(&mut self) -> REG6_W {
        REG6_W { w: self }
    }
}
