#[doc = "Reader of register AIFDIRCFG"]
pub type R = crate::R<u32, super::AIFDIRCFG>;
#[doc = "Writer for register AIFDIRCFG"]
pub type W = crate::W<u32, super::AIFDIRCFG>;
#[doc = "Register AIFDIRCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFDIRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AD1_A {
    #[doc = "2: Output mode"]
    OUT = 2,
    #[doc = "1: Input mode"]
    IN = 1,
    #[doc = "0: Not in use (disabled)"]
    DIS = 0,
}
impl From<AD1_A> for u8 {
    #[inline(always)]
    fn from(variant: AD1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AD1`"]
pub type AD1_R = crate::R<u8, AD1_A>;
impl AD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AD1_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(AD1_A::OUT),
            1 => Val(AD1_A::IN),
            0 => Val(AD1_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == AD1_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == AD1_A::IN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AD1_A::DIS
    }
}
#[doc = "Write proxy for field `AD1`"]
pub struct AD1_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(AD1_A::OUT)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(AD1_A::IN)
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AD1_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AD0_A {
    #[doc = "2: Output mode"]
    OUT = 2,
    #[doc = "1: Input mode"]
    IN = 1,
    #[doc = "0: Not in use (disabled)"]
    DIS = 0,
}
impl From<AD0_A> for u8 {
    #[inline(always)]
    fn from(variant: AD0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AD0`"]
pub type AD0_R = crate::R<u8, AD0_A>;
impl AD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AD0_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(AD0_A::OUT),
            1 => Val(AD0_A::IN),
            0 => Val(AD0_A::DIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == AD0_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == AD0_A::IN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AD0_A::DIS
    }
}
#[doc = "Write proxy for field `AD0`"]
pub struct AD0_W<'a> {
    w: &'a mut W,
}
impl<'a> AD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output mode"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(AD0_A::OUT)
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(AD0_A::IN)
    }
    #[doc = "Not in use (disabled)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AD0_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad1(&self) -> AD1_R {
        AD1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad0(&self) -> AD0_R {
        AD0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad1(&mut self) -> AD1_W {
        AD1_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline(always)]
    pub fn ad0(&mut self) -> AD0_W {
        AD0_W { w: self }
    }
}
