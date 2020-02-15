#[doc = "Reader of register ICTR"]
pub type R = crate::R<u32, super::ICTR>;
#[doc = "Writer for register ICTR"]
pub type W = crate::W<u32, super::ICTR>;
#[doc = "Register ICTR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::ICTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
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
#[doc = "Reader of field `INTLINESNUM`"]
pub type INTLINESNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTLINESNUM`"]
pub struct INTLINESNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLINESNUM_W<'a> {
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
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
    #[inline(always)]
    pub fn intlinesnum(&self) -> INTLINESNUM_R {
        INTLINESNUM_R::new((self.bits & 0x07) as u8)
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
Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256"]
    #[inline(always)]
    pub fn intlinesnum(&mut self) -> INTLINESNUM_W {
        INTLINESNUM_W { w: self }
    }
}
