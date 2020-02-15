#[doc = "Reader of register REG1_0"]
pub type R = crate::R<u32, super::REG1_0>;
#[doc = "Writer for register REG1_0"]
pub type W = crate::W<u32, super::REG1_0>;
#[doc = "Register REG1_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG1_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG1`"]
pub type REG1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG1`"]
pub struct REG1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `REG0`"]
pub type REG0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG0`"]
pub struct REG0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG0_W<'a> {
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
    pub fn reg1(&self) -> REG1_R {
        REG1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg0(&self) -> REG0_R {
        REG0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg1(&mut self) -> REG1_W {
        REG1_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg0(&mut self) -> REG0_W {
        REG0_W { w: self }
    }
}
