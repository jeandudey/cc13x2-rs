#[doc = "Reader of register NVIC_ISER0"]
pub type R = crate::R<u32, super::NVIC_ISER0>;
#[doc = "Writer for register NVIC_ISER0"]
pub type W = crate::W<u32, super::NVIC_ISER0>;
#[doc = "Register NVIC_ISER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETENA31`"]
pub type SETENA31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA31`"]
pub struct SETENA31_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SETENA30`"]
pub type SETENA30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA30`"]
pub struct SETENA30_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SETENA29`"]
pub type SETENA29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA29`"]
pub struct SETENA29_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SETENA28`"]
pub type SETENA28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA28`"]
pub struct SETENA28_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SETENA27`"]
pub type SETENA27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA27`"]
pub struct SETENA27_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SETENA26`"]
pub type SETENA26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA26`"]
pub struct SETENA26_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SETENA25`"]
pub type SETENA25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA25`"]
pub struct SETENA25_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SETENA24`"]
pub type SETENA24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA24`"]
pub struct SETENA24_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SETENA23`"]
pub type SETENA23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA23`"]
pub struct SETENA23_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SETENA22`"]
pub type SETENA22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA22`"]
pub struct SETENA22_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SETENA21`"]
pub type SETENA21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA21`"]
pub struct SETENA21_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SETENA20`"]
pub type SETENA20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA20`"]
pub struct SETENA20_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SETENA19`"]
pub type SETENA19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA19`"]
pub struct SETENA19_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SETENA18`"]
pub type SETENA18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA18`"]
pub struct SETENA18_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SETENA17`"]
pub type SETENA17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA17`"]
pub struct SETENA17_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SETENA16`"]
pub type SETENA16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA16`"]
pub struct SETENA16_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SETENA15`"]
pub type SETENA15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA15`"]
pub struct SETENA15_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SETENA14`"]
pub type SETENA14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA14`"]
pub struct SETENA14_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SETENA13`"]
pub type SETENA13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA13`"]
pub struct SETENA13_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SETENA12`"]
pub type SETENA12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA12`"]
pub struct SETENA12_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SETENA11`"]
pub type SETENA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA11`"]
pub struct SETENA11_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SETENA10`"]
pub type SETENA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA10`"]
pub struct SETENA10_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SETENA9`"]
pub type SETENA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA9`"]
pub struct SETENA9_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SETENA8`"]
pub type SETENA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA8`"]
pub struct SETENA8_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SETENA7`"]
pub type SETENA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA7`"]
pub struct SETENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SETENA6`"]
pub type SETENA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA6`"]
pub struct SETENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SETENA5`"]
pub type SETENA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA5`"]
pub struct SETENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SETENA4`"]
pub type SETENA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA4`"]
pub struct SETENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SETENA3`"]
pub type SETENA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA3`"]
pub struct SETENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SETENA2`"]
pub type SETENA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA2`"]
pub struct SETENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SETENA1`"]
pub type SETENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA1`"]
pub struct SETENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SETENA0`"]
pub type SETENA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA0`"]
pub struct SETENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA0_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena31(&self) -> SETENA31_R {
        SETENA31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena30(&self) -> SETENA30_R {
        SETENA30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena29(&self) -> SETENA29_R {
        SETENA29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena28(&self) -> SETENA28_R {
        SETENA28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena27(&self) -> SETENA27_R {
        SETENA27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena26(&self) -> SETENA26_R {
        SETENA26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena25(&self) -> SETENA25_R {
        SETENA25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena24(&self) -> SETENA24_R {
        SETENA24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena23(&self) -> SETENA23_R {
        SETENA23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena22(&self) -> SETENA22_R {
        SETENA22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena21(&self) -> SETENA21_R {
        SETENA21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena20(&self) -> SETENA20_R {
        SETENA20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena19(&self) -> SETENA19_R {
        SETENA19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena18(&self) -> SETENA18_R {
        SETENA18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena17(&self) -> SETENA17_R {
        SETENA17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena16(&self) -> SETENA16_R {
        SETENA16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena15(&self) -> SETENA15_R {
        SETENA15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena14(&self) -> SETENA14_R {
        SETENA14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena13(&self) -> SETENA13_R {
        SETENA13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena12(&self) -> SETENA12_R {
        SETENA12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena11(&self) -> SETENA11_R {
        SETENA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena10(&self) -> SETENA10_R {
        SETENA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena9(&self) -> SETENA9_R {
        SETENA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena8(&self) -> SETENA8_R {
        SETENA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena7(&self) -> SETENA7_R {
        SETENA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena6(&self) -> SETENA6_R {
        SETENA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena5(&self) -> SETENA5_R {
        SETENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena4(&self) -> SETENA4_R {
        SETENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena3(&self) -> SETENA3_R {
        SETENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena2(&self) -> SETENA2_R {
        SETENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena1(&self) -> SETENA1_R {
        SETENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena0(&self) -> SETENA0_R {
        SETENA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena31(&mut self) -> SETENA31_W {
        SETENA31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena30(&mut self) -> SETENA30_W {
        SETENA30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena29(&mut self) -> SETENA29_W {
        SETENA29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena28(&mut self) -> SETENA28_W {
        SETENA28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena27(&mut self) -> SETENA27_W {
        SETENA27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena26(&mut self) -> SETENA26_W {
        SETENA26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena25(&mut self) -> SETENA25_W {
        SETENA25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena24(&mut self) -> SETENA24_W {
        SETENA24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena23(&mut self) -> SETENA23_W {
        SETENA23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena22(&mut self) -> SETENA22_W {
        SETENA22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena21(&mut self) -> SETENA21_W {
        SETENA21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena20(&mut self) -> SETENA20_W {
        SETENA20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena19(&mut self) -> SETENA19_W {
        SETENA19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena18(&mut self) -> SETENA18_W {
        SETENA18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena17(&mut self) -> SETENA17_W {
        SETENA17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena16(&mut self) -> SETENA16_W {
        SETENA16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena15(&mut self) -> SETENA15_W {
        SETENA15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena14(&mut self) -> SETENA14_W {
        SETENA14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena13(&mut self) -> SETENA13_W {
        SETENA13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena12(&mut self) -> SETENA12_W {
        SETENA12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena11(&mut self) -> SETENA11_W {
        SETENA11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena10(&mut self) -> SETENA10_W {
        SETENA10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena9(&mut self) -> SETENA9_W {
        SETENA9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena8(&mut self) -> SETENA8_W {
        SETENA8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena7(&mut self) -> SETENA7_W {
        SETENA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena6(&mut self) -> SETENA6_W {
        SETENA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena5(&mut self) -> SETENA5_W {
        SETENA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena4(&mut self) -> SETENA4_W {
        SETENA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena3(&mut self) -> SETENA3_W {
        SETENA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena2(&mut self) -> SETENA2_W {
        SETENA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena1(&mut self) -> SETENA1_W {
        SETENA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena0(&mut self) -> SETENA0_W {
        SETENA0_W { w: self }
    }
}
