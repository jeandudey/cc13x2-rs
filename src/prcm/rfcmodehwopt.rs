#[doc = "Reader of register RFCMODEHWOPT"]
pub type R = crate::R<u32, super::RFCMODEHWOPT>;
#[doc = "Writer for register RFCMODEHWOPT"]
pub type W = crate::W<u32, super::RFCMODEHWOPT>;
#[doc = "Register RFCMODEHWOPT `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCMODEHWOPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "7:0\\]
Permitted RFC modes. More than one mode can be permitted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AVAIL_A {
    #[doc = "128: Mode 7 permitted"]
    MODE7 = 128,
    #[doc = "64: Mode 6 permitted"]
    MODE6 = 64,
    #[doc = "32: Mode 5 permitted"]
    MODE5 = 32,
    #[doc = "16: Mode 4 permitted"]
    MODE4 = 16,
    #[doc = "8: Mode 3 permitted"]
    MODE3 = 8,
    #[doc = "4: Mode 2 permitted"]
    MODE2 = 4,
    #[doc = "2: Mode 1 permitted"]
    MODE1 = 2,
    #[doc = "1: Mode 0 permitted"]
    MODE0 = 1,
}
impl From<AVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: AVAIL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AVAIL`"]
pub type AVAIL_R = crate::R<u8, AVAIL_A>;
impl AVAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AVAIL_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(AVAIL_A::MODE7),
            64 => Val(AVAIL_A::MODE6),
            32 => Val(AVAIL_A::MODE5),
            16 => Val(AVAIL_A::MODE4),
            8 => Val(AVAIL_A::MODE3),
            4 => Val(AVAIL_A::MODE2),
            2 => Val(AVAIL_A::MODE1),
            1 => Val(AVAIL_A::MODE0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE7`"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == AVAIL_A::MODE7
    }
    #[doc = "Checks if the value of the field is `MODE6`"]
    #[inline(always)]
    pub fn is_mode6(&self) -> bool {
        *self == AVAIL_A::MODE6
    }
    #[doc = "Checks if the value of the field is `MODE5`"]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == AVAIL_A::MODE5
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == AVAIL_A::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == AVAIL_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == AVAIL_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == AVAIL_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == AVAIL_A::MODE0
    }
}
#[doc = "Write proxy for field `AVAIL`"]
pub struct AVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> AVAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVAIL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Mode 7 permitted"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE7)
    }
    #[doc = "Mode 6 permitted"]
    #[inline(always)]
    pub fn mode6(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE6)
    }
    #[doc = "Mode 5 permitted"]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE5)
    }
    #[doc = "Mode 4 permitted"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE4)
    }
    #[doc = "Mode 3 permitted"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE3)
    }
    #[doc = "Mode 2 permitted"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE2)
    }
    #[doc = "Mode 1 permitted"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE1)
    }
    #[doc = "Mode 0 permitted"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(AVAIL_A::MODE0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    pub fn avail(&self) -> AVAIL_R {
        AVAIL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    pub fn avail(&mut self) -> AVAIL_W {
        AVAIL_W { w: self }
    }
}
