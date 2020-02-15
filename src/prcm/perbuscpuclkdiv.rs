#[doc = "Reader of register PERBUSCPUCLKDIV"]
pub type R = crate::R<u32, super::PERBUSCPUCLKDIV>;
#[doc = "Writer for register PERBUSCPUCLKDIV"]
pub type W = crate::W<u32, super::PERBUSCPUCLKDIV>;
#[doc = "Register PERBUSCPUCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::PERBUSCPUCLKDIV {
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
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    DIV256 = 8,
    #[doc = "7: Internal. Only to be used through TI provided API."]
    DIV128 = 7,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    DIV64 = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    DIV32 = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    DIV16 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    DIV8 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    DIV4 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DIV2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    DIV1 = 0,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<u8, RATIO_A>;
impl RATIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RATIO_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(RATIO_A::DIV256),
            7 => Val(RATIO_A::DIV128),
            6 => Val(RATIO_A::DIV64),
            5 => Val(RATIO_A::DIV32),
            4 => Val(RATIO_A::DIV16),
            3 => Val(RATIO_A::DIV8),
            2 => Val(RATIO_A::DIV4),
            1 => Val(RATIO_A::DIV2),
            0 => Val(RATIO_A::DIV1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == RATIO_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == RATIO_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == RATIO_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RATIO_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RATIO_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RATIO_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RATIO_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RATIO_A::DIV1
    }
}
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(RATIO_A::DIV256)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(RATIO_A::DIV128)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(RATIO_A::DIV64)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIO_A::DIV32)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RATIO_A::DIV16)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIO_A::DIV8)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RATIO_A::DIV4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
