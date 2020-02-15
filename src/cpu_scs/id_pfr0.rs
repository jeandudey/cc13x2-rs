#[doc = "Reader of register ID_PFR0"]
pub type R = crate::R<u32, super::ID_PFR0>;
#[doc = "Writer for register ID_PFR0"]
pub type W = crate::W<u32, super::ID_PFR0>;
#[doc = "Register ID_PFR0 `reset()`'s with value 0x30"]
impl crate::ResetValue for super::ID_PFR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
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
#[doc = "Reader of field `STATE1`"]
pub type STATE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE1`"]
pub struct STATE1_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `STATE0`"]
pub type STATE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE0`"]
pub struct STATE0_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    pub fn state1(&mut self) -> STATE1_W {
        STATE1_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    pub fn state0(&mut self) -> STATE0_W {
        STATE0_W { w: self }
    }
}
