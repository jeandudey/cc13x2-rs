#[doc = "Reader of register NVIC_ICER0"]
pub type R = crate::R<u32, super::NVIC_ICER0>;
#[doc = "Writer for register NVIC_ICER0"]
pub type W = crate::W<u32, super::NVIC_ICER0>;
#[doc = "Register NVIC_ICER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRENA31`"]
pub type CLRENA31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA31`"]
pub struct CLRENA31_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA31_W<'a> {
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
#[doc = "Reader of field `CLRENA30`"]
pub type CLRENA30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA30`"]
pub struct CLRENA30_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA30_W<'a> {
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
#[doc = "Reader of field `CLRENA29`"]
pub type CLRENA29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA29`"]
pub struct CLRENA29_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA29_W<'a> {
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
#[doc = "Reader of field `CLRENA28`"]
pub type CLRENA28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA28`"]
pub struct CLRENA28_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA28_W<'a> {
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
#[doc = "Reader of field `CLRENA27`"]
pub type CLRENA27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA27`"]
pub struct CLRENA27_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA27_W<'a> {
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
#[doc = "Reader of field `CLRENA26`"]
pub type CLRENA26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA26`"]
pub struct CLRENA26_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA26_W<'a> {
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
#[doc = "Reader of field `CLRENA25`"]
pub type CLRENA25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA25`"]
pub struct CLRENA25_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA25_W<'a> {
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
#[doc = "Reader of field `CLRENA24`"]
pub type CLRENA24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA24`"]
pub struct CLRENA24_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA24_W<'a> {
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
#[doc = "Reader of field `CLRENA23`"]
pub type CLRENA23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA23`"]
pub struct CLRENA23_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA23_W<'a> {
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
#[doc = "Reader of field `CLRENA22`"]
pub type CLRENA22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA22`"]
pub struct CLRENA22_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA22_W<'a> {
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
#[doc = "Reader of field `CLRENA21`"]
pub type CLRENA21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA21`"]
pub struct CLRENA21_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA21_W<'a> {
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
#[doc = "Reader of field `CLRENA20`"]
pub type CLRENA20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA20`"]
pub struct CLRENA20_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA20_W<'a> {
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
#[doc = "Reader of field `CLRENA19`"]
pub type CLRENA19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA19`"]
pub struct CLRENA19_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA19_W<'a> {
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
#[doc = "Reader of field `CLRENA18`"]
pub type CLRENA18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA18`"]
pub struct CLRENA18_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA18_W<'a> {
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
#[doc = "Reader of field `CLRENA17`"]
pub type CLRENA17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA17`"]
pub struct CLRENA17_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA17_W<'a> {
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
#[doc = "Reader of field `CLRENA16`"]
pub type CLRENA16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA16`"]
pub struct CLRENA16_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA16_W<'a> {
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
#[doc = "Reader of field `CLRENA15`"]
pub type CLRENA15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA15`"]
pub struct CLRENA15_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA15_W<'a> {
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
#[doc = "Reader of field `CLRENA14`"]
pub type CLRENA14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA14`"]
pub struct CLRENA14_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA14_W<'a> {
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
#[doc = "Reader of field `CLRENA13`"]
pub type CLRENA13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA13`"]
pub struct CLRENA13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA13_W<'a> {
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
#[doc = "Reader of field `CLRENA12`"]
pub type CLRENA12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA12`"]
pub struct CLRENA12_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA12_W<'a> {
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
#[doc = "Reader of field `CLRENA11`"]
pub type CLRENA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA11`"]
pub struct CLRENA11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA11_W<'a> {
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
#[doc = "Reader of field `CLRENA10`"]
pub type CLRENA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA10`"]
pub struct CLRENA10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA10_W<'a> {
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
#[doc = "Reader of field `CLRENA9`"]
pub type CLRENA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA9`"]
pub struct CLRENA9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA9_W<'a> {
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
#[doc = "Reader of field `CLRENA8`"]
pub type CLRENA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA8`"]
pub struct CLRENA8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA8_W<'a> {
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
#[doc = "Reader of field `CLRENA7`"]
pub type CLRENA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA7`"]
pub struct CLRENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA7_W<'a> {
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
#[doc = "Reader of field `CLRENA6`"]
pub type CLRENA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA6`"]
pub struct CLRENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA6_W<'a> {
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
#[doc = "Reader of field `CLRENA5`"]
pub type CLRENA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA5`"]
pub struct CLRENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA5_W<'a> {
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
#[doc = "Reader of field `CLRENA4`"]
pub type CLRENA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA4`"]
pub struct CLRENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA4_W<'a> {
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
#[doc = "Reader of field `CLRENA3`"]
pub type CLRENA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA3`"]
pub struct CLRENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA3_W<'a> {
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
#[doc = "Reader of field `CLRENA2`"]
pub type CLRENA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA2`"]
pub struct CLRENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA2_W<'a> {
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
#[doc = "Reader of field `CLRENA1`"]
pub type CLRENA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA1`"]
pub struct CLRENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA1_W<'a> {
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
#[doc = "Reader of field `CLRENA0`"]
pub type CLRENA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA0`"]
pub struct CLRENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA0_W<'a> {
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
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena31(&self) -> CLRENA31_R {
        CLRENA31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena30(&self) -> CLRENA30_R {
        CLRENA30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena29(&self) -> CLRENA29_R {
        CLRENA29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena28(&self) -> CLRENA28_R {
        CLRENA28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena27(&self) -> CLRENA27_R {
        CLRENA27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena26(&self) -> CLRENA26_R {
        CLRENA26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena25(&self) -> CLRENA25_R {
        CLRENA25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena24(&self) -> CLRENA24_R {
        CLRENA24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena23(&self) -> CLRENA23_R {
        CLRENA23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena22(&self) -> CLRENA22_R {
        CLRENA22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena21(&self) -> CLRENA21_R {
        CLRENA21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena20(&self) -> CLRENA20_R {
        CLRENA20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena19(&self) -> CLRENA19_R {
        CLRENA19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena18(&self) -> CLRENA18_R {
        CLRENA18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena17(&self) -> CLRENA17_R {
        CLRENA17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena16(&self) -> CLRENA16_R {
        CLRENA16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena15(&self) -> CLRENA15_R {
        CLRENA15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena14(&self) -> CLRENA14_R {
        CLRENA14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena13(&self) -> CLRENA13_R {
        CLRENA13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena12(&self) -> CLRENA12_R {
        CLRENA12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena11(&self) -> CLRENA11_R {
        CLRENA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena10(&self) -> CLRENA10_R {
        CLRENA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena9(&self) -> CLRENA9_R {
        CLRENA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena8(&self) -> CLRENA8_R {
        CLRENA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena7(&self) -> CLRENA7_R {
        CLRENA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena6(&self) -> CLRENA6_R {
        CLRENA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena5(&self) -> CLRENA5_R {
        CLRENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena4(&self) -> CLRENA4_R {
        CLRENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena3(&self) -> CLRENA3_R {
        CLRENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena2(&self) -> CLRENA2_R {
        CLRENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena1(&self) -> CLRENA1_R {
        CLRENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena0(&self) -> CLRENA0_R {
        CLRENA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena31(&mut self) -> CLRENA31_W {
        CLRENA31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena30(&mut self) -> CLRENA30_W {
        CLRENA30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena29(&mut self) -> CLRENA29_W {
        CLRENA29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena28(&mut self) -> CLRENA28_W {
        CLRENA28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena27(&mut self) -> CLRENA27_W {
        CLRENA27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena26(&mut self) -> CLRENA26_W {
        CLRENA26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena25(&mut self) -> CLRENA25_W {
        CLRENA25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena24(&mut self) -> CLRENA24_W {
        CLRENA24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena23(&mut self) -> CLRENA23_W {
        CLRENA23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena22(&mut self) -> CLRENA22_W {
        CLRENA22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena21(&mut self) -> CLRENA21_W {
        CLRENA21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena20(&mut self) -> CLRENA20_W {
        CLRENA20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena19(&mut self) -> CLRENA19_W {
        CLRENA19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena18(&mut self) -> CLRENA18_W {
        CLRENA18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena17(&mut self) -> CLRENA17_W {
        CLRENA17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena16(&mut self) -> CLRENA16_W {
        CLRENA16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena15(&mut self) -> CLRENA15_W {
        CLRENA15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena14(&mut self) -> CLRENA14_W {
        CLRENA14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena13(&mut self) -> CLRENA13_W {
        CLRENA13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena12(&mut self) -> CLRENA12_W {
        CLRENA12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena11(&mut self) -> CLRENA11_W {
        CLRENA11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena10(&mut self) -> CLRENA10_W {
        CLRENA10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena9(&mut self) -> CLRENA9_W {
        CLRENA9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena8(&mut self) -> CLRENA8_W {
        CLRENA8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena7(&mut self) -> CLRENA7_W {
        CLRENA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena6(&mut self) -> CLRENA6_W {
        CLRENA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena5(&mut self) -> CLRENA5_W {
        CLRENA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena4(&mut self) -> CLRENA4_W {
        CLRENA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena3(&mut self) -> CLRENA3_W {
        CLRENA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena2(&mut self) -> CLRENA2_W {
        CLRENA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena1(&mut self) -> CLRENA1_W {
        CLRENA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena0(&mut self) -> CLRENA0_W {
        CLRENA0_W { w: self }
    }
}
