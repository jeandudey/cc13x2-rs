#[doc = "Reader of register MEASCFG"]
pub type R = crate::R<u32, super::MEASCFG>;
#[doc = "Writer for register MEASCFG"]
pub type W = crate::W<u32, super::MEASCFG>;
#[doc = "Register MEASCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEASCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PER_A {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _32CYC = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _16CYC = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _8CYC = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    CONT = 0,
}
impl From<PER_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<u8, PER_A>;
impl PER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            3 => PER_A::_32CYC,
            2 => PER_A::_16CYC,
            1 => PER_A::_8CYC,
            0 => PER_A::CONT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32CYC`"]
    #[inline(always)]
    pub fn is_32cyc(&self) -> bool {
        *self == PER_A::_32CYC
    }
    #[doc = "Checks if the value of the field is `_16CYC`"]
    #[inline(always)]
    pub fn is_16cyc(&self) -> bool {
        *self == PER_A::_16CYC
    }
    #[doc = "Checks if the value of the field is `_8CYC`"]
    #[inline(always)]
    pub fn is_8cyc(&self) -> bool {
        *self == PER_A::_8CYC
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == PER_A::CONT
    }
}
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _32cyc(self) -> &'a mut W {
        self.variant(PER_A::_32CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _16cyc(self) -> &'a mut W {
        self.variant(PER_A::_16CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _8cyc(self) -> &'a mut W {
        self.variant(PER_A::_8CYC)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(PER_A::CONT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
}
