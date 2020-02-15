#[doc = "Reader of register FSM_PGM128"]
pub type R = crate::R<u32, super::FSM_PGM128>;
#[doc = "Writer for register FSM_PGM128"]
pub type W = crate::W<u32, super::FSM_PGM128>;
#[doc = "Register FSM_PGM128 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PGM128 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN_PGM128`"]
pub type EN_PGM128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_PGM128`"]
pub struct EN_PGM128_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_PGM128_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    pub fn en_pgm128(&self) -> EN_PGM128_R {
        EN_PGM128_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    pub fn en_pgm128(&mut self) -> EN_PGM128_W {
        EN_PGM128_W { w: self }
    }
}
