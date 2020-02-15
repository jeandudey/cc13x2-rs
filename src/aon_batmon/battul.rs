#[doc = "Reader of register BATTUL"]
pub type R = crate::R<u32, super::BATTUL>;
#[doc = "Writer for register BATTUL"]
pub type W = crate::W<u32, super::BATTUL>;
#[doc = "Register BATTUL `reset()`'s with value 0x07ff"]
impl crate::ResetValue for super::BATTUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07ff
    }
}
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAC`"]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Integer part: 0x0: 0V + fractional part ... 0x3: 3V + fractional part 0x4: 4V + fractional part"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Fractional part, standard binary fractional encoding. 0x00: .0V ... 0x20: 1/8 = .125V 0x40: 1/4 = .25V 0x80: 1/2 = .5V ... 0xA0: 1/2 + 1/8 = .625V ... 0xFF: Max"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Integer part: 0x0: 0V + fractional part ... 0x3: 3V + fractional part 0x4: 4V + fractional part"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Fractional part, standard binary fractional encoding. 0x00: .0V ... 0x20: 1/8 = .125V 0x40: 1/4 = .25V 0x80: 1/2 = .5V ... 0xA0: 1/2 + 1/8 = .625V ... 0xFF: Max"]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W {
        FRAC_W { w: self }
    }
}
