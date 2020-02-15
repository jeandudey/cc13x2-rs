#[doc = "Reader of register TPR"]
pub type R = crate::R<u32, super::TPR>;
#[doc = "Writer for register TPR"]
pub type W = crate::W<u32, super::TPR>;
#[doc = "Register TPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `PRIVMASK`"]
pub type PRIVMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIVMASK`"]
pub struct PRIVMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
    #[inline(always)]
    pub fn privmask(&self) -> PRIVMASK_R {
        PRIVMASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
    #[inline(always)]
    pub fn privmask(&mut self) -> PRIVMASK_W {
        PRIVMASK_W { w: self }
    }
}
