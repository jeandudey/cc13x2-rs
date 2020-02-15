#[doc = "Reader of register NVIC_ICPR0"]
pub type R = crate::R<u32, super::NVIC_ICPR0>;
#[doc = "Writer for register NVIC_ICPR0"]
pub type W = crate::W<u32, super::NVIC_ICPR0>;
#[doc = "Register NVIC_ICPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRPEND31`"]
pub type CLRPEND31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND31`"]
pub struct CLRPEND31_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND31_W<'a> {
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
#[doc = "Reader of field `CLRPEND30`"]
pub type CLRPEND30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND30`"]
pub struct CLRPEND30_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND30_W<'a> {
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
#[doc = "Reader of field `CLRPEND29`"]
pub type CLRPEND29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND29`"]
pub struct CLRPEND29_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND29_W<'a> {
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
#[doc = "Reader of field `CLRPEND28`"]
pub type CLRPEND28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND28`"]
pub struct CLRPEND28_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND28_W<'a> {
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
#[doc = "Reader of field `CLRPEND27`"]
pub type CLRPEND27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND27`"]
pub struct CLRPEND27_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND27_W<'a> {
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
#[doc = "Reader of field `CLRPEND26`"]
pub type CLRPEND26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND26`"]
pub struct CLRPEND26_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND26_W<'a> {
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
#[doc = "Reader of field `CLRPEND25`"]
pub type CLRPEND25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND25`"]
pub struct CLRPEND25_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND25_W<'a> {
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
#[doc = "Reader of field `CLRPEND24`"]
pub type CLRPEND24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND24`"]
pub struct CLRPEND24_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND24_W<'a> {
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
#[doc = "Reader of field `CLRPEND23`"]
pub type CLRPEND23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND23`"]
pub struct CLRPEND23_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND23_W<'a> {
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
#[doc = "Reader of field `CLRPEND22`"]
pub type CLRPEND22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND22`"]
pub struct CLRPEND22_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND22_W<'a> {
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
#[doc = "Reader of field `CLRPEND21`"]
pub type CLRPEND21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND21`"]
pub struct CLRPEND21_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND21_W<'a> {
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
#[doc = "Reader of field `CLRPEND20`"]
pub type CLRPEND20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND20`"]
pub struct CLRPEND20_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND20_W<'a> {
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
#[doc = "Reader of field `CLRPEND19`"]
pub type CLRPEND19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND19`"]
pub struct CLRPEND19_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND19_W<'a> {
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
#[doc = "Reader of field `CLRPEND18`"]
pub type CLRPEND18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND18`"]
pub struct CLRPEND18_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND18_W<'a> {
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
#[doc = "Reader of field `CLRPEND17`"]
pub type CLRPEND17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND17`"]
pub struct CLRPEND17_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND17_W<'a> {
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
#[doc = "Reader of field `CLRPEND16`"]
pub type CLRPEND16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND16`"]
pub struct CLRPEND16_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND16_W<'a> {
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
#[doc = "Reader of field `CLRPEND15`"]
pub type CLRPEND15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND15`"]
pub struct CLRPEND15_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND15_W<'a> {
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
#[doc = "Reader of field `CLRPEND14`"]
pub type CLRPEND14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND14`"]
pub struct CLRPEND14_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND14_W<'a> {
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
#[doc = "Reader of field `CLRPEND13`"]
pub type CLRPEND13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND13`"]
pub struct CLRPEND13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND13_W<'a> {
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
#[doc = "Reader of field `CLRPEND12`"]
pub type CLRPEND12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND12`"]
pub struct CLRPEND12_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND12_W<'a> {
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
#[doc = "Reader of field `CLRPEND11`"]
pub type CLRPEND11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND11`"]
pub struct CLRPEND11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND11_W<'a> {
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
#[doc = "Reader of field `CLRPEND10`"]
pub type CLRPEND10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND10`"]
pub struct CLRPEND10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND10_W<'a> {
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
#[doc = "Reader of field `CLRPEND9`"]
pub type CLRPEND9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND9`"]
pub struct CLRPEND9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND9_W<'a> {
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
#[doc = "Reader of field `CLRPEND8`"]
pub type CLRPEND8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND8`"]
pub struct CLRPEND8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND8_W<'a> {
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
#[doc = "Reader of field `CLRPEND7`"]
pub type CLRPEND7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND7`"]
pub struct CLRPEND7_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND7_W<'a> {
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
#[doc = "Reader of field `CLRPEND6`"]
pub type CLRPEND6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND6`"]
pub struct CLRPEND6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND6_W<'a> {
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
#[doc = "Reader of field `CLRPEND5`"]
pub type CLRPEND5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND5`"]
pub struct CLRPEND5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND5_W<'a> {
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
#[doc = "Reader of field `CLRPEND4`"]
pub type CLRPEND4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND4`"]
pub struct CLRPEND4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND4_W<'a> {
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
#[doc = "Reader of field `CLRPEND3`"]
pub type CLRPEND3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND3`"]
pub struct CLRPEND3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND3_W<'a> {
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
#[doc = "Reader of field `CLRPEND2`"]
pub type CLRPEND2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND2`"]
pub struct CLRPEND2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND2_W<'a> {
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
#[doc = "Reader of field `CLRPEND1`"]
pub type CLRPEND1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND1`"]
pub struct CLRPEND1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND1_W<'a> {
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
#[doc = "Reader of field `CLRPEND0`"]
pub type CLRPEND0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND0`"]
pub struct CLRPEND0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND0_W<'a> {
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
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend31(&self) -> CLRPEND31_R {
        CLRPEND31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend30(&self) -> CLRPEND30_R {
        CLRPEND30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend29(&self) -> CLRPEND29_R {
        CLRPEND29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend28(&self) -> CLRPEND28_R {
        CLRPEND28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend27(&self) -> CLRPEND27_R {
        CLRPEND27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend26(&self) -> CLRPEND26_R {
        CLRPEND26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend25(&self) -> CLRPEND25_R {
        CLRPEND25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend24(&self) -> CLRPEND24_R {
        CLRPEND24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend23(&self) -> CLRPEND23_R {
        CLRPEND23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend22(&self) -> CLRPEND22_R {
        CLRPEND22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend21(&self) -> CLRPEND21_R {
        CLRPEND21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend20(&self) -> CLRPEND20_R {
        CLRPEND20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend19(&self) -> CLRPEND19_R {
        CLRPEND19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend18(&self) -> CLRPEND18_R {
        CLRPEND18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend17(&self) -> CLRPEND17_R {
        CLRPEND17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend16(&self) -> CLRPEND16_R {
        CLRPEND16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend15(&self) -> CLRPEND15_R {
        CLRPEND15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend14(&self) -> CLRPEND14_R {
        CLRPEND14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend13(&self) -> CLRPEND13_R {
        CLRPEND13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend12(&self) -> CLRPEND12_R {
        CLRPEND12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend11(&self) -> CLRPEND11_R {
        CLRPEND11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend10(&self) -> CLRPEND10_R {
        CLRPEND10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend9(&self) -> CLRPEND9_R {
        CLRPEND9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend8(&self) -> CLRPEND8_R {
        CLRPEND8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend7(&self) -> CLRPEND7_R {
        CLRPEND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend6(&self) -> CLRPEND6_R {
        CLRPEND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend5(&self) -> CLRPEND5_R {
        CLRPEND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend4(&self) -> CLRPEND4_R {
        CLRPEND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend3(&self) -> CLRPEND3_R {
        CLRPEND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend2(&self) -> CLRPEND2_R {
        CLRPEND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend1(&self) -> CLRPEND1_R {
        CLRPEND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend0(&self) -> CLRPEND0_R {
        CLRPEND0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend31(&mut self) -> CLRPEND31_W {
        CLRPEND31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend30(&mut self) -> CLRPEND30_W {
        CLRPEND30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend29(&mut self) -> CLRPEND29_W {
        CLRPEND29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend28(&mut self) -> CLRPEND28_W {
        CLRPEND28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend27(&mut self) -> CLRPEND27_W {
        CLRPEND27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend26(&mut self) -> CLRPEND26_W {
        CLRPEND26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend25(&mut self) -> CLRPEND25_W {
        CLRPEND25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend24(&mut self) -> CLRPEND24_W {
        CLRPEND24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend23(&mut self) -> CLRPEND23_W {
        CLRPEND23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend22(&mut self) -> CLRPEND22_W {
        CLRPEND22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend21(&mut self) -> CLRPEND21_W {
        CLRPEND21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend20(&mut self) -> CLRPEND20_W {
        CLRPEND20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend19(&mut self) -> CLRPEND19_W {
        CLRPEND19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend18(&mut self) -> CLRPEND18_W {
        CLRPEND18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend17(&mut self) -> CLRPEND17_W {
        CLRPEND17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend16(&mut self) -> CLRPEND16_W {
        CLRPEND16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend15(&mut self) -> CLRPEND15_W {
        CLRPEND15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend14(&mut self) -> CLRPEND14_W {
        CLRPEND14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend13(&mut self) -> CLRPEND13_W {
        CLRPEND13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend12(&mut self) -> CLRPEND12_W {
        CLRPEND12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend11(&mut self) -> CLRPEND11_W {
        CLRPEND11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend10(&mut self) -> CLRPEND10_W {
        CLRPEND10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend9(&mut self) -> CLRPEND9_W {
        CLRPEND9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend8(&mut self) -> CLRPEND8_W {
        CLRPEND8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend7(&mut self) -> CLRPEND7_W {
        CLRPEND7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend6(&mut self) -> CLRPEND6_W {
        CLRPEND6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend5(&mut self) -> CLRPEND5_W {
        CLRPEND5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend4(&mut self) -> CLRPEND4_W {
        CLRPEND4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend3(&mut self) -> CLRPEND3_W {
        CLRPEND3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend2(&mut self) -> CLRPEND2_W {
        CLRPEND2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend1(&mut self) -> CLRPEND1_W {
        CLRPEND1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend0(&mut self) -> CLRPEND0_W {
        CLRPEND0_W { w: self }
    }
}
