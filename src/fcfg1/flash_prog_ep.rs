#[doc = "Reader of register FLASH_PROG_EP"]
pub type R = crate::R<u32, super::FLASH_PROG_EP>;
#[doc = "Writer for register FLASH_PROG_EP"]
pub type W = crate::W<u32, super::FLASH_PROG_EP>;
#[doc = "Register FLASH_PROG_EP `reset()`'s with value 0x0fa0_0010"]
impl crate::ResetValue for super::FLASH_PROG_EP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fa0_0010
    }
}
#[doc = "Reader of field `MAX_EP`"]
pub type MAX_EP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_EP`"]
pub struct MAX_EP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PROGRAM_PW`"]
pub type PROGRAM_PW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PROGRAM_PW`"]
pub struct PROGRAM_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGRAM_PW_W<'a> {
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
    pub fn max_ep(&self) -> MAX_EP_R {
        MAX_EP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn program_pw(&self) -> PROGRAM_PW_R {
        PROGRAM_PW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ep(&mut self) -> MAX_EP_W {
        MAX_EP_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn program_pw(&mut self) -> PROGRAM_PW_W {
        PROGRAM_PW_W { w: self }
    }
}
