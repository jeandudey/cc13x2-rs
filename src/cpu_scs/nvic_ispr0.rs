#[doc = "Reader of register NVIC_ISPR0"]
pub type R = crate::R<u32, super::NVIC_ISPR0>;
#[doc = "Writer for register NVIC_ISPR0"]
pub type W = crate::W<u32, super::NVIC_ISPR0>;
#[doc = "Register NVIC_ISPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETPEND31`"]
pub type SETPEND31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND31`"]
pub struct SETPEND31_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND31_W<'a> {
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
#[doc = "Reader of field `SETPEND30`"]
pub type SETPEND30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND30`"]
pub struct SETPEND30_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND30_W<'a> {
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
#[doc = "Reader of field `SETPEND29`"]
pub type SETPEND29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND29`"]
pub struct SETPEND29_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND29_W<'a> {
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
#[doc = "Reader of field `SETPEND28`"]
pub type SETPEND28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND28`"]
pub struct SETPEND28_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND28_W<'a> {
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
#[doc = "Reader of field `SETPEND27`"]
pub type SETPEND27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND27`"]
pub struct SETPEND27_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND27_W<'a> {
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
#[doc = "Reader of field `SETPEND26`"]
pub type SETPEND26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND26`"]
pub struct SETPEND26_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND26_W<'a> {
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
#[doc = "Reader of field `SETPEND25`"]
pub type SETPEND25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND25`"]
pub struct SETPEND25_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND25_W<'a> {
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
#[doc = "Reader of field `SETPEND24`"]
pub type SETPEND24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND24`"]
pub struct SETPEND24_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND24_W<'a> {
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
#[doc = "Reader of field `SETPEND23`"]
pub type SETPEND23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND23`"]
pub struct SETPEND23_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND23_W<'a> {
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
#[doc = "Reader of field `SETPEND22`"]
pub type SETPEND22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND22`"]
pub struct SETPEND22_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND22_W<'a> {
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
#[doc = "Reader of field `SETPEND21`"]
pub type SETPEND21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND21`"]
pub struct SETPEND21_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND21_W<'a> {
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
#[doc = "Reader of field `SETPEND20`"]
pub type SETPEND20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND20`"]
pub struct SETPEND20_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND20_W<'a> {
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
#[doc = "Reader of field `SETPEND19`"]
pub type SETPEND19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND19`"]
pub struct SETPEND19_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND19_W<'a> {
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
#[doc = "Reader of field `SETPEND18`"]
pub type SETPEND18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND18`"]
pub struct SETPEND18_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND18_W<'a> {
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
#[doc = "Reader of field `SETPEND17`"]
pub type SETPEND17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND17`"]
pub struct SETPEND17_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND17_W<'a> {
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
#[doc = "Reader of field `SETPEND16`"]
pub type SETPEND16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND16`"]
pub struct SETPEND16_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND16_W<'a> {
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
#[doc = "Reader of field `SETPEND15`"]
pub type SETPEND15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND15`"]
pub struct SETPEND15_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND15_W<'a> {
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
#[doc = "Reader of field `SETPEND14`"]
pub type SETPEND14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND14`"]
pub struct SETPEND14_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND14_W<'a> {
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
#[doc = "Reader of field `SETPEND13`"]
pub type SETPEND13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND13`"]
pub struct SETPEND13_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND13_W<'a> {
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
#[doc = "Reader of field `SETPEND12`"]
pub type SETPEND12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND12`"]
pub struct SETPEND12_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND12_W<'a> {
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
#[doc = "Reader of field `SETPEND11`"]
pub type SETPEND11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND11`"]
pub struct SETPEND11_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND11_W<'a> {
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
#[doc = "Reader of field `SETPEND10`"]
pub type SETPEND10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND10`"]
pub struct SETPEND10_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND10_W<'a> {
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
#[doc = "Reader of field `SETPEND9`"]
pub type SETPEND9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND9`"]
pub struct SETPEND9_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND9_W<'a> {
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
#[doc = "Reader of field `SETPEND8`"]
pub type SETPEND8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND8`"]
pub struct SETPEND8_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND8_W<'a> {
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
#[doc = "Reader of field `SETPEND7`"]
pub type SETPEND7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND7`"]
pub struct SETPEND7_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND7_W<'a> {
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
#[doc = "Reader of field `SETPEND6`"]
pub type SETPEND6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND6`"]
pub struct SETPEND6_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND6_W<'a> {
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
#[doc = "Reader of field `SETPEND5`"]
pub type SETPEND5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND5`"]
pub struct SETPEND5_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND5_W<'a> {
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
#[doc = "Reader of field `SETPEND4`"]
pub type SETPEND4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND4`"]
pub struct SETPEND4_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND4_W<'a> {
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
#[doc = "Reader of field `SETPEND3`"]
pub type SETPEND3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND3`"]
pub struct SETPEND3_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND3_W<'a> {
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
#[doc = "Reader of field `SETPEND2`"]
pub type SETPEND2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND2`"]
pub struct SETPEND2_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND2_W<'a> {
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
#[doc = "Reader of field `SETPEND1`"]
pub type SETPEND1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND1`"]
pub struct SETPEND1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND1_W<'a> {
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
#[doc = "Reader of field `SETPEND0`"]
pub type SETPEND0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETPEND0`"]
pub struct SETPEND0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND0_W<'a> {
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
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend31(&self) -> SETPEND31_R {
        SETPEND31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend30(&self) -> SETPEND30_R {
        SETPEND30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend29(&self) -> SETPEND29_R {
        SETPEND29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend28(&self) -> SETPEND28_R {
        SETPEND28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend27(&self) -> SETPEND27_R {
        SETPEND27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend26(&self) -> SETPEND26_R {
        SETPEND26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend25(&self) -> SETPEND25_R {
        SETPEND25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend24(&self) -> SETPEND24_R {
        SETPEND24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend23(&self) -> SETPEND23_R {
        SETPEND23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend22(&self) -> SETPEND22_R {
        SETPEND22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend21(&self) -> SETPEND21_R {
        SETPEND21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend20(&self) -> SETPEND20_R {
        SETPEND20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend19(&self) -> SETPEND19_R {
        SETPEND19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend18(&self) -> SETPEND18_R {
        SETPEND18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend17(&self) -> SETPEND17_R {
        SETPEND17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend16(&self) -> SETPEND16_R {
        SETPEND16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend15(&self) -> SETPEND15_R {
        SETPEND15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend14(&self) -> SETPEND14_R {
        SETPEND14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend13(&self) -> SETPEND13_R {
        SETPEND13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend12(&self) -> SETPEND12_R {
        SETPEND12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend11(&self) -> SETPEND11_R {
        SETPEND11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend10(&self) -> SETPEND10_R {
        SETPEND10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend9(&self) -> SETPEND9_R {
        SETPEND9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend8(&self) -> SETPEND8_R {
        SETPEND8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend7(&self) -> SETPEND7_R {
        SETPEND7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend6(&self) -> SETPEND6_R {
        SETPEND6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend5(&self) -> SETPEND5_R {
        SETPEND5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend4(&self) -> SETPEND4_R {
        SETPEND4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend3(&self) -> SETPEND3_R {
        SETPEND3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend2(&self) -> SETPEND2_R {
        SETPEND2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend1(&self) -> SETPEND1_R {
        SETPEND1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend0(&self) -> SETPEND0_R {
        SETPEND0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend31(&mut self) -> SETPEND31_W {
        SETPEND31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend30(&mut self) -> SETPEND30_W {
        SETPEND30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend29(&mut self) -> SETPEND29_W {
        SETPEND29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend28(&mut self) -> SETPEND28_W {
        SETPEND28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend27(&mut self) -> SETPEND27_W {
        SETPEND27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend26(&mut self) -> SETPEND26_W {
        SETPEND26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend25(&mut self) -> SETPEND25_W {
        SETPEND25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend24(&mut self) -> SETPEND24_W {
        SETPEND24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend23(&mut self) -> SETPEND23_W {
        SETPEND23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend22(&mut self) -> SETPEND22_W {
        SETPEND22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend21(&mut self) -> SETPEND21_W {
        SETPEND21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend20(&mut self) -> SETPEND20_W {
        SETPEND20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend19(&mut self) -> SETPEND19_W {
        SETPEND19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend18(&mut self) -> SETPEND18_W {
        SETPEND18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend17(&mut self) -> SETPEND17_W {
        SETPEND17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend16(&mut self) -> SETPEND16_W {
        SETPEND16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend15(&mut self) -> SETPEND15_W {
        SETPEND15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend14(&mut self) -> SETPEND14_W {
        SETPEND14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend13(&mut self) -> SETPEND13_W {
        SETPEND13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend12(&mut self) -> SETPEND12_W {
        SETPEND12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend11(&mut self) -> SETPEND11_W {
        SETPEND11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend10(&mut self) -> SETPEND10_W {
        SETPEND10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend9(&mut self) -> SETPEND9_W {
        SETPEND9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend8(&mut self) -> SETPEND8_W {
        SETPEND8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend7(&mut self) -> SETPEND7_W {
        SETPEND7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend6(&mut self) -> SETPEND6_W {
        SETPEND6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend5(&mut self) -> SETPEND5_W {
        SETPEND5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend4(&mut self) -> SETPEND4_W {
        SETPEND4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend3(&mut self) -> SETPEND3_W {
        SETPEND3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend2(&mut self) -> SETPEND2_W {
        SETPEND2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend1(&mut self) -> SETPEND1_W {
        SETPEND1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend0(&mut self) -> SETPEND0_W {
        SETPEND0_W { w: self }
    }
}
