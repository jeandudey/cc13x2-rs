#[doc = "Reader of register REG3_2"]
pub type R = crate::R<u32, super::REG3_2>;
#[doc = "Writer for register REG3_2"]
pub type W = crate::W<u32, super::REG3_2>;
#[doc = "Register REG3_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG3_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG3`"]
pub type REG3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG3`"]
pub struct REG3_W<'a> {
    w: &'a mut W,
}
impl<'a> REG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `REG2`"]
pub type REG2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG2`"]
pub struct REG2_W<'a> {
    w: &'a mut W,
}
impl<'a> REG2_W<'a> {
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
    pub fn reg3(&self) -> REG3_R {
        REG3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg2(&self) -> REG2_R {
        REG2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg3(&mut self) -> REG3_W {
        REG3_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg2(&mut self) -> REG2_W {
        REG2_W { w: self }
    }
}
