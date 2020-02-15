#[doc = "Reader of register CCFG_PROT_31_0"]
pub type R = crate::R<u32, super::CCFG_PROT_31_0>;
#[doc = "Writer for register CCFG_PROT_31_0"]
pub type W = crate::W<u32, super::CCFG_PROT_31_0>;
#[doc = "Register CCFG_PROT_31_0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CCFG_PROT_31_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `WRT_PROT_SEC_31`"]
pub type WRT_PROT_SEC_31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_31`"]
pub struct WRT_PROT_SEC_31_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_31_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_30`"]
pub type WRT_PROT_SEC_30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_30`"]
pub struct WRT_PROT_SEC_30_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_30_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_29`"]
pub type WRT_PROT_SEC_29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_29`"]
pub struct WRT_PROT_SEC_29_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_29_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_28`"]
pub type WRT_PROT_SEC_28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_28`"]
pub struct WRT_PROT_SEC_28_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_28_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_27`"]
pub type WRT_PROT_SEC_27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_27`"]
pub struct WRT_PROT_SEC_27_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_27_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_26`"]
pub type WRT_PROT_SEC_26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_26`"]
pub struct WRT_PROT_SEC_26_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_26_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_25`"]
pub type WRT_PROT_SEC_25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_25`"]
pub struct WRT_PROT_SEC_25_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_25_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_24`"]
pub type WRT_PROT_SEC_24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_24`"]
pub struct WRT_PROT_SEC_24_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_24_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_23`"]
pub type WRT_PROT_SEC_23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_23`"]
pub struct WRT_PROT_SEC_23_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_23_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_22`"]
pub type WRT_PROT_SEC_22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_22`"]
pub struct WRT_PROT_SEC_22_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_22_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_21`"]
pub type WRT_PROT_SEC_21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_21`"]
pub struct WRT_PROT_SEC_21_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_21_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_20`"]
pub type WRT_PROT_SEC_20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_20`"]
pub struct WRT_PROT_SEC_20_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_20_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_19`"]
pub type WRT_PROT_SEC_19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_19`"]
pub struct WRT_PROT_SEC_19_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_19_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_18`"]
pub type WRT_PROT_SEC_18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_18`"]
pub struct WRT_PROT_SEC_18_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_18_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_17`"]
pub type WRT_PROT_SEC_17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_17`"]
pub struct WRT_PROT_SEC_17_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_17_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_16`"]
pub type WRT_PROT_SEC_16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_16`"]
pub struct WRT_PROT_SEC_16_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_16_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_15`"]
pub type WRT_PROT_SEC_15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_15`"]
pub struct WRT_PROT_SEC_15_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_15_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_14`"]
pub type WRT_PROT_SEC_14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_14`"]
pub struct WRT_PROT_SEC_14_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_14_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_13`"]
pub type WRT_PROT_SEC_13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_13`"]
pub struct WRT_PROT_SEC_13_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_13_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_12`"]
pub type WRT_PROT_SEC_12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_12`"]
pub struct WRT_PROT_SEC_12_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_12_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_11`"]
pub type WRT_PROT_SEC_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_11`"]
pub struct WRT_PROT_SEC_11_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_11_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_10`"]
pub type WRT_PROT_SEC_10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_10`"]
pub struct WRT_PROT_SEC_10_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_10_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_9`"]
pub type WRT_PROT_SEC_9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_9`"]
pub struct WRT_PROT_SEC_9_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_9_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_8`"]
pub type WRT_PROT_SEC_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_8`"]
pub struct WRT_PROT_SEC_8_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_8_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_7`"]
pub type WRT_PROT_SEC_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_7`"]
pub struct WRT_PROT_SEC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_7_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_6`"]
pub type WRT_PROT_SEC_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_6`"]
pub struct WRT_PROT_SEC_6_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_6_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_5`"]
pub type WRT_PROT_SEC_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_5`"]
pub struct WRT_PROT_SEC_5_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_5_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_4`"]
pub type WRT_PROT_SEC_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_4`"]
pub struct WRT_PROT_SEC_4_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_4_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_3`"]
pub type WRT_PROT_SEC_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_3`"]
pub struct WRT_PROT_SEC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_3_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_2`"]
pub type WRT_PROT_SEC_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_2`"]
pub struct WRT_PROT_SEC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_2_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_1`"]
pub type WRT_PROT_SEC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_1`"]
pub struct WRT_PROT_SEC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_1_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_0`"]
pub type WRT_PROT_SEC_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_0`"]
pub struct WRT_PROT_SEC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_0_W<'a> {
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
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_31(&self) -> WRT_PROT_SEC_31_R {
        WRT_PROT_SEC_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_30(&self) -> WRT_PROT_SEC_30_R {
        WRT_PROT_SEC_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_29(&self) -> WRT_PROT_SEC_29_R {
        WRT_PROT_SEC_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_28(&self) -> WRT_PROT_SEC_28_R {
        WRT_PROT_SEC_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_27(&self) -> WRT_PROT_SEC_27_R {
        WRT_PROT_SEC_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_26(&self) -> WRT_PROT_SEC_26_R {
        WRT_PROT_SEC_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_25(&self) -> WRT_PROT_SEC_25_R {
        WRT_PROT_SEC_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_24(&self) -> WRT_PROT_SEC_24_R {
        WRT_PROT_SEC_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_23(&self) -> WRT_PROT_SEC_23_R {
        WRT_PROT_SEC_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_22(&self) -> WRT_PROT_SEC_22_R {
        WRT_PROT_SEC_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_21(&self) -> WRT_PROT_SEC_21_R {
        WRT_PROT_SEC_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_20(&self) -> WRT_PROT_SEC_20_R {
        WRT_PROT_SEC_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_19(&self) -> WRT_PROT_SEC_19_R {
        WRT_PROT_SEC_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_18(&self) -> WRT_PROT_SEC_18_R {
        WRT_PROT_SEC_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_17(&self) -> WRT_PROT_SEC_17_R {
        WRT_PROT_SEC_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_16(&self) -> WRT_PROT_SEC_16_R {
        WRT_PROT_SEC_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_15(&self) -> WRT_PROT_SEC_15_R {
        WRT_PROT_SEC_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_14(&self) -> WRT_PROT_SEC_14_R {
        WRT_PROT_SEC_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_13(&self) -> WRT_PROT_SEC_13_R {
        WRT_PROT_SEC_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_12(&self) -> WRT_PROT_SEC_12_R {
        WRT_PROT_SEC_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_11(&self) -> WRT_PROT_SEC_11_R {
        WRT_PROT_SEC_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_10(&self) -> WRT_PROT_SEC_10_R {
        WRT_PROT_SEC_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_9(&self) -> WRT_PROT_SEC_9_R {
        WRT_PROT_SEC_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_8(&self) -> WRT_PROT_SEC_8_R {
        WRT_PROT_SEC_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_7(&self) -> WRT_PROT_SEC_7_R {
        WRT_PROT_SEC_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_6(&self) -> WRT_PROT_SEC_6_R {
        WRT_PROT_SEC_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_5(&self) -> WRT_PROT_SEC_5_R {
        WRT_PROT_SEC_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_4(&self) -> WRT_PROT_SEC_4_R {
        WRT_PROT_SEC_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_3(&self) -> WRT_PROT_SEC_3_R {
        WRT_PROT_SEC_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_2(&self) -> WRT_PROT_SEC_2_R {
        WRT_PROT_SEC_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_1(&self) -> WRT_PROT_SEC_1_R {
        WRT_PROT_SEC_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_0(&self) -> WRT_PROT_SEC_0_R {
        WRT_PROT_SEC_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_31(&mut self) -> WRT_PROT_SEC_31_W {
        WRT_PROT_SEC_31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_30(&mut self) -> WRT_PROT_SEC_30_W {
        WRT_PROT_SEC_30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_29(&mut self) -> WRT_PROT_SEC_29_W {
        WRT_PROT_SEC_29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_28(&mut self) -> WRT_PROT_SEC_28_W {
        WRT_PROT_SEC_28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_27(&mut self) -> WRT_PROT_SEC_27_W {
        WRT_PROT_SEC_27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_26(&mut self) -> WRT_PROT_SEC_26_W {
        WRT_PROT_SEC_26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_25(&mut self) -> WRT_PROT_SEC_25_W {
        WRT_PROT_SEC_25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_24(&mut self) -> WRT_PROT_SEC_24_W {
        WRT_PROT_SEC_24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_23(&mut self) -> WRT_PROT_SEC_23_W {
        WRT_PROT_SEC_23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_22(&mut self) -> WRT_PROT_SEC_22_W {
        WRT_PROT_SEC_22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_21(&mut self) -> WRT_PROT_SEC_21_W {
        WRT_PROT_SEC_21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_20(&mut self) -> WRT_PROT_SEC_20_W {
        WRT_PROT_SEC_20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_19(&mut self) -> WRT_PROT_SEC_19_W {
        WRT_PROT_SEC_19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_18(&mut self) -> WRT_PROT_SEC_18_W {
        WRT_PROT_SEC_18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_17(&mut self) -> WRT_PROT_SEC_17_W {
        WRT_PROT_SEC_17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_16(&mut self) -> WRT_PROT_SEC_16_W {
        WRT_PROT_SEC_16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_15(&mut self) -> WRT_PROT_SEC_15_W {
        WRT_PROT_SEC_15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_14(&mut self) -> WRT_PROT_SEC_14_W {
        WRT_PROT_SEC_14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_13(&mut self) -> WRT_PROT_SEC_13_W {
        WRT_PROT_SEC_13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_12(&mut self) -> WRT_PROT_SEC_12_W {
        WRT_PROT_SEC_12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_11(&mut self) -> WRT_PROT_SEC_11_W {
        WRT_PROT_SEC_11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_10(&mut self) -> WRT_PROT_SEC_10_W {
        WRT_PROT_SEC_10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_9(&mut self) -> WRT_PROT_SEC_9_W {
        WRT_PROT_SEC_9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_8(&mut self) -> WRT_PROT_SEC_8_W {
        WRT_PROT_SEC_8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_7(&mut self) -> WRT_PROT_SEC_7_W {
        WRT_PROT_SEC_7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_6(&mut self) -> WRT_PROT_SEC_6_W {
        WRT_PROT_SEC_6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_5(&mut self) -> WRT_PROT_SEC_5_W {
        WRT_PROT_SEC_5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_4(&mut self) -> WRT_PROT_SEC_4_W {
        WRT_PROT_SEC_4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_3(&mut self) -> WRT_PROT_SEC_3_W {
        WRT_PROT_SEC_3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_2(&mut self) -> WRT_PROT_SEC_2_W {
        WRT_PROT_SEC_2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_1(&mut self) -> WRT_PROT_SEC_1_W {
        WRT_PROT_SEC_1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_0(&mut self) -> WRT_PROT_SEC_0_W {
        WRT_PROT_SEC_0_W { w: self }
    }
}
