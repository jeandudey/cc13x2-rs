#[doc = "Reader of register MASK2"]
pub type R = crate::R<u32, super::MASK2>;
#[doc = "Writer for register MASK2"]
pub type W = crate::W<u32, super::MASK2>;
#[doc = "Register MASK2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK2 {
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
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
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
Mask on data address when matching against COMP2. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP2. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP2 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x0f) as u8)
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
Mask on data address when matching against COMP2. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP2. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP2 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
}
