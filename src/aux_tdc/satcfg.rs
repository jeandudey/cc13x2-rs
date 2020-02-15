#[doc = "Reader of register SATCFG"]
pub type R = crate::R<u32, super::SATCFG>;
#[doc = "Writer for register SATCFG"]
pub type W = crate::W<u32, super::SATCFG>;
#[doc = "Register SATCFG `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::SATCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
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
#[doc = "3:0\\]
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LIMIT_A {
    #[doc = "15: Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    R24 = 15,
    #[doc = "14: Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    R23 = 14,
    #[doc = "13: Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    R22 = 13,
    #[doc = "12: Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    R21 = 12,
    #[doc = "11: Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    R20 = 11,
    #[doc = "10: Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    R19 = 10,
    #[doc = "9: Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    R18 = 9,
    #[doc = "8: Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    R17 = 8,
    #[doc = "7: Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    R16 = 7,
    #[doc = "6: Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    R15 = 6,
    #[doc = "5: Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    R14 = 5,
    #[doc = "4: Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    R13 = 4,
    #[doc = "3: Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    R12 = 3,
}
impl From<LIMIT_A> for u8 {
    #[inline(always)]
    fn from(variant: LIMIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LIMIT`"]
pub type LIMIT_R = crate::R<u8, LIMIT_A>;
impl LIMIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LIMIT_A> {
        use crate::Variant::*;
        match self.bits {
            15 => Val(LIMIT_A::R24),
            14 => Val(LIMIT_A::R23),
            13 => Val(LIMIT_A::R22),
            12 => Val(LIMIT_A::R21),
            11 => Val(LIMIT_A::R20),
            10 => Val(LIMIT_A::R19),
            9 => Val(LIMIT_A::R18),
            8 => Val(LIMIT_A::R17),
            7 => Val(LIMIT_A::R16),
            6 => Val(LIMIT_A::R15),
            5 => Val(LIMIT_A::R14),
            4 => Val(LIMIT_A::R13),
            3 => Val(LIMIT_A::R12),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `R24`"]
    #[inline(always)]
    pub fn is_r24(&self) -> bool {
        *self == LIMIT_A::R24
    }
    #[doc = "Checks if the value of the field is `R23`"]
    #[inline(always)]
    pub fn is_r23(&self) -> bool {
        *self == LIMIT_A::R23
    }
    #[doc = "Checks if the value of the field is `R22`"]
    #[inline(always)]
    pub fn is_r22(&self) -> bool {
        *self == LIMIT_A::R22
    }
    #[doc = "Checks if the value of the field is `R21`"]
    #[inline(always)]
    pub fn is_r21(&self) -> bool {
        *self == LIMIT_A::R21
    }
    #[doc = "Checks if the value of the field is `R20`"]
    #[inline(always)]
    pub fn is_r20(&self) -> bool {
        *self == LIMIT_A::R20
    }
    #[doc = "Checks if the value of the field is `R19`"]
    #[inline(always)]
    pub fn is_r19(&self) -> bool {
        *self == LIMIT_A::R19
    }
    #[doc = "Checks if the value of the field is `R18`"]
    #[inline(always)]
    pub fn is_r18(&self) -> bool {
        *self == LIMIT_A::R18
    }
    #[doc = "Checks if the value of the field is `R17`"]
    #[inline(always)]
    pub fn is_r17(&self) -> bool {
        *self == LIMIT_A::R17
    }
    #[doc = "Checks if the value of the field is `R16`"]
    #[inline(always)]
    pub fn is_r16(&self) -> bool {
        *self == LIMIT_A::R16
    }
    #[doc = "Checks if the value of the field is `R15`"]
    #[inline(always)]
    pub fn is_r15(&self) -> bool {
        *self == LIMIT_A::R15
    }
    #[doc = "Checks if the value of the field is `R14`"]
    #[inline(always)]
    pub fn is_r14(&self) -> bool {
        *self == LIMIT_A::R14
    }
    #[doc = "Checks if the value of the field is `R13`"]
    #[inline(always)]
    pub fn is_r13(&self) -> bool {
        *self == LIMIT_A::R13
    }
    #[doc = "Checks if the value of the field is `R12`"]
    #[inline(always)]
    pub fn is_r12(&self) -> bool {
        *self == LIMIT_A::R12
    }
}
#[doc = "Write proxy for field `LIMIT`"]
pub struct LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIMIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\]
is set."]
    #[inline(always)]
    pub fn r24(self) -> &'a mut W {
        self.variant(LIMIT_A::R24)
    }
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\]
is set."]
    #[inline(always)]
    pub fn r23(self) -> &'a mut W {
        self.variant(LIMIT_A::R23)
    }
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\]
is set."]
    #[inline(always)]
    pub fn r22(self) -> &'a mut W {
        self.variant(LIMIT_A::R22)
    }
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\]
is set."]
    #[inline(always)]
    pub fn r21(self) -> &'a mut W {
        self.variant(LIMIT_A::R21)
    }
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\]
is set."]
    #[inline(always)]
    pub fn r20(self) -> &'a mut W {
        self.variant(LIMIT_A::R20)
    }
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\]
is set."]
    #[inline(always)]
    pub fn r19(self) -> &'a mut W {
        self.variant(LIMIT_A::R19)
    }
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\]
is set."]
    #[inline(always)]
    pub fn r18(self) -> &'a mut W {
        self.variant(LIMIT_A::R18)
    }
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\]
is set."]
    #[inline(always)]
    pub fn r17(self) -> &'a mut W {
        self.variant(LIMIT_A::R17)
    }
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\]
is set."]
    #[inline(always)]
    pub fn r16(self) -> &'a mut W {
        self.variant(LIMIT_A::R16)
    }
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\]
is set."]
    #[inline(always)]
    pub fn r15(self) -> &'a mut W {
        self.variant(LIMIT_A::R15)
    }
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\]
is set."]
    #[inline(always)]
    pub fn r14(self) -> &'a mut W {
        self.variant(LIMIT_A::R14)
    }
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\]
is set."]
    #[inline(always)]
    pub fn r13(self) -> &'a mut W {
        self.variant(LIMIT_A::R13)
    }
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\]
is set."]
    #[inline(always)]
    pub fn r12(self) -> &'a mut W {
        self.variant(LIMIT_A::R12)
    }
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
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits & 0x0f) as u8)
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
Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported"]
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W {
        LIMIT_W { w: self }
    }
}
