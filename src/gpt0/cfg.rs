#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG_A {
    #[doc = "4: 16-bit timer configuration.  \nConfigure for two 16-bit timers.\nAlso see TAMR.TAMR and TBMR.TBMR."]
    _16BIT_TIMER = 4,
    #[doc = "0: 32-bit timer configuration"]
    _32BIT_TIMER = 0,
}
impl From<CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG`"]
pub type CFG_R = crate::R<u8, CFG_A>;
impl CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFG_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(CFG_A::_16BIT_TIMER),
            0 => Val(CFG_A::_32BIT_TIMER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT_TIMER`"]
    #[inline(always)]
    pub fn is_16bit_timer(&self) -> bool {
        *self == CFG_A::_16BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CFG_A::_32BIT_TIMER
    }
}
#[doc = "Write proxy for field `CFG`"]
pub struct CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    #[inline(always)]
    pub fn _16bit_timer(self) -> &'a mut W {
        self.variant(CFG_A::_16BIT_TIMER)
    }
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CFG_A::_32BIT_TIMER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W { w: self }
    }
}
