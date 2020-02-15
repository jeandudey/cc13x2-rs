#[doc = "Reader of register CCFG_PROT_63_32"]
pub type R = crate::R<u32, super::CCFG_PROT_63_32>;
#[doc = "Writer for register CCFG_PROT_63_32"]
pub type W = crate::W<u32, super::CCFG_PROT_63_32>;
#[doc = "Register CCFG_PROT_63_32 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CCFG_PROT_63_32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `WRT_PROT_SEC_63`"]
pub type WRT_PROT_SEC_63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_63`"]
pub struct WRT_PROT_SEC_63_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_63_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_62`"]
pub type WRT_PROT_SEC_62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_62`"]
pub struct WRT_PROT_SEC_62_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_62_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_61`"]
pub type WRT_PROT_SEC_61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_61`"]
pub struct WRT_PROT_SEC_61_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_61_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_60`"]
pub type WRT_PROT_SEC_60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_60`"]
pub struct WRT_PROT_SEC_60_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_60_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_59`"]
pub type WRT_PROT_SEC_59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_59`"]
pub struct WRT_PROT_SEC_59_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_59_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_58`"]
pub type WRT_PROT_SEC_58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_58`"]
pub struct WRT_PROT_SEC_58_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_58_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_57`"]
pub type WRT_PROT_SEC_57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_57`"]
pub struct WRT_PROT_SEC_57_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_57_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_56`"]
pub type WRT_PROT_SEC_56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_56`"]
pub struct WRT_PROT_SEC_56_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_56_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_55`"]
pub type WRT_PROT_SEC_55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_55`"]
pub struct WRT_PROT_SEC_55_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_55_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_54`"]
pub type WRT_PROT_SEC_54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_54`"]
pub struct WRT_PROT_SEC_54_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_54_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_53`"]
pub type WRT_PROT_SEC_53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_53`"]
pub struct WRT_PROT_SEC_53_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_53_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_52`"]
pub type WRT_PROT_SEC_52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_52`"]
pub struct WRT_PROT_SEC_52_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_52_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_51`"]
pub type WRT_PROT_SEC_51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_51`"]
pub struct WRT_PROT_SEC_51_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_51_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_50`"]
pub type WRT_PROT_SEC_50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_50`"]
pub struct WRT_PROT_SEC_50_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_50_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_49`"]
pub type WRT_PROT_SEC_49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_49`"]
pub struct WRT_PROT_SEC_49_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_49_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_48`"]
pub type WRT_PROT_SEC_48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_48`"]
pub struct WRT_PROT_SEC_48_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_48_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_47`"]
pub type WRT_PROT_SEC_47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_47`"]
pub struct WRT_PROT_SEC_47_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_47_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_46`"]
pub type WRT_PROT_SEC_46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_46`"]
pub struct WRT_PROT_SEC_46_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_46_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_45`"]
pub type WRT_PROT_SEC_45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_45`"]
pub struct WRT_PROT_SEC_45_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_45_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_44`"]
pub type WRT_PROT_SEC_44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_44`"]
pub struct WRT_PROT_SEC_44_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_44_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_43`"]
pub type WRT_PROT_SEC_43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_43`"]
pub struct WRT_PROT_SEC_43_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_43_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_42`"]
pub type WRT_PROT_SEC_42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_42`"]
pub struct WRT_PROT_SEC_42_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_42_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_41`"]
pub type WRT_PROT_SEC_41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_41`"]
pub struct WRT_PROT_SEC_41_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_41_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_40`"]
pub type WRT_PROT_SEC_40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_40`"]
pub struct WRT_PROT_SEC_40_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_40_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_39`"]
pub type WRT_PROT_SEC_39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_39`"]
pub struct WRT_PROT_SEC_39_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_39_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_38`"]
pub type WRT_PROT_SEC_38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_38`"]
pub struct WRT_PROT_SEC_38_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_38_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_37`"]
pub type WRT_PROT_SEC_37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_37`"]
pub struct WRT_PROT_SEC_37_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_37_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_36`"]
pub type WRT_PROT_SEC_36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_36`"]
pub struct WRT_PROT_SEC_36_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_36_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_35`"]
pub type WRT_PROT_SEC_35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_35`"]
pub struct WRT_PROT_SEC_35_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_35_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_34`"]
pub type WRT_PROT_SEC_34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_34`"]
pub struct WRT_PROT_SEC_34_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_34_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_33`"]
pub type WRT_PROT_SEC_33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_33`"]
pub struct WRT_PROT_SEC_33_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_33_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_32`"]
pub type WRT_PROT_SEC_32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_32`"]
pub struct WRT_PROT_SEC_32_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_32_W<'a> {
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
    pub fn wrt_prot_sec_63(&self) -> WRT_PROT_SEC_63_R {
        WRT_PROT_SEC_63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_62(&self) -> WRT_PROT_SEC_62_R {
        WRT_PROT_SEC_62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_61(&self) -> WRT_PROT_SEC_61_R {
        WRT_PROT_SEC_61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_60(&self) -> WRT_PROT_SEC_60_R {
        WRT_PROT_SEC_60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_59(&self) -> WRT_PROT_SEC_59_R {
        WRT_PROT_SEC_59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_58(&self) -> WRT_PROT_SEC_58_R {
        WRT_PROT_SEC_58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_57(&self) -> WRT_PROT_SEC_57_R {
        WRT_PROT_SEC_57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_56(&self) -> WRT_PROT_SEC_56_R {
        WRT_PROT_SEC_56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_55(&self) -> WRT_PROT_SEC_55_R {
        WRT_PROT_SEC_55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_54(&self) -> WRT_PROT_SEC_54_R {
        WRT_PROT_SEC_54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_53(&self) -> WRT_PROT_SEC_53_R {
        WRT_PROT_SEC_53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_52(&self) -> WRT_PROT_SEC_52_R {
        WRT_PROT_SEC_52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_51(&self) -> WRT_PROT_SEC_51_R {
        WRT_PROT_SEC_51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_50(&self) -> WRT_PROT_SEC_50_R {
        WRT_PROT_SEC_50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_49(&self) -> WRT_PROT_SEC_49_R {
        WRT_PROT_SEC_49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_48(&self) -> WRT_PROT_SEC_48_R {
        WRT_PROT_SEC_48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_47(&self) -> WRT_PROT_SEC_47_R {
        WRT_PROT_SEC_47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_46(&self) -> WRT_PROT_SEC_46_R {
        WRT_PROT_SEC_46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_45(&self) -> WRT_PROT_SEC_45_R {
        WRT_PROT_SEC_45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_44(&self) -> WRT_PROT_SEC_44_R {
        WRT_PROT_SEC_44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_43(&self) -> WRT_PROT_SEC_43_R {
        WRT_PROT_SEC_43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_42(&self) -> WRT_PROT_SEC_42_R {
        WRT_PROT_SEC_42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_41(&self) -> WRT_PROT_SEC_41_R {
        WRT_PROT_SEC_41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_40(&self) -> WRT_PROT_SEC_40_R {
        WRT_PROT_SEC_40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_39(&self) -> WRT_PROT_SEC_39_R {
        WRT_PROT_SEC_39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_38(&self) -> WRT_PROT_SEC_38_R {
        WRT_PROT_SEC_38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_37(&self) -> WRT_PROT_SEC_37_R {
        WRT_PROT_SEC_37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_36(&self) -> WRT_PROT_SEC_36_R {
        WRT_PROT_SEC_36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_35(&self) -> WRT_PROT_SEC_35_R {
        WRT_PROT_SEC_35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_34(&self) -> WRT_PROT_SEC_34_R {
        WRT_PROT_SEC_34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_33(&self) -> WRT_PROT_SEC_33_R {
        WRT_PROT_SEC_33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_32(&self) -> WRT_PROT_SEC_32_R {
        WRT_PROT_SEC_32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_63(&mut self) -> WRT_PROT_SEC_63_W {
        WRT_PROT_SEC_63_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_62(&mut self) -> WRT_PROT_SEC_62_W {
        WRT_PROT_SEC_62_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_61(&mut self) -> WRT_PROT_SEC_61_W {
        WRT_PROT_SEC_61_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_60(&mut self) -> WRT_PROT_SEC_60_W {
        WRT_PROT_SEC_60_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_59(&mut self) -> WRT_PROT_SEC_59_W {
        WRT_PROT_SEC_59_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_58(&mut self) -> WRT_PROT_SEC_58_W {
        WRT_PROT_SEC_58_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_57(&mut self) -> WRT_PROT_SEC_57_W {
        WRT_PROT_SEC_57_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_56(&mut self) -> WRT_PROT_SEC_56_W {
        WRT_PROT_SEC_56_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_55(&mut self) -> WRT_PROT_SEC_55_W {
        WRT_PROT_SEC_55_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_54(&mut self) -> WRT_PROT_SEC_54_W {
        WRT_PROT_SEC_54_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_53(&mut self) -> WRT_PROT_SEC_53_W {
        WRT_PROT_SEC_53_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_52(&mut self) -> WRT_PROT_SEC_52_W {
        WRT_PROT_SEC_52_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_51(&mut self) -> WRT_PROT_SEC_51_W {
        WRT_PROT_SEC_51_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_50(&mut self) -> WRT_PROT_SEC_50_W {
        WRT_PROT_SEC_50_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_49(&mut self) -> WRT_PROT_SEC_49_W {
        WRT_PROT_SEC_49_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_48(&mut self) -> WRT_PROT_SEC_48_W {
        WRT_PROT_SEC_48_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_47(&mut self) -> WRT_PROT_SEC_47_W {
        WRT_PROT_SEC_47_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_46(&mut self) -> WRT_PROT_SEC_46_W {
        WRT_PROT_SEC_46_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_45(&mut self) -> WRT_PROT_SEC_45_W {
        WRT_PROT_SEC_45_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_44(&mut self) -> WRT_PROT_SEC_44_W {
        WRT_PROT_SEC_44_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_43(&mut self) -> WRT_PROT_SEC_43_W {
        WRT_PROT_SEC_43_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_42(&mut self) -> WRT_PROT_SEC_42_W {
        WRT_PROT_SEC_42_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_41(&mut self) -> WRT_PROT_SEC_41_W {
        WRT_PROT_SEC_41_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_40(&mut self) -> WRT_PROT_SEC_40_W {
        WRT_PROT_SEC_40_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_39(&mut self) -> WRT_PROT_SEC_39_W {
        WRT_PROT_SEC_39_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_38(&mut self) -> WRT_PROT_SEC_38_W {
        WRT_PROT_SEC_38_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_37(&mut self) -> WRT_PROT_SEC_37_W {
        WRT_PROT_SEC_37_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_36(&mut self) -> WRT_PROT_SEC_36_W {
        WRT_PROT_SEC_36_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_35(&mut self) -> WRT_PROT_SEC_35_W {
        WRT_PROT_SEC_35_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_34(&mut self) -> WRT_PROT_SEC_34_W {
        WRT_PROT_SEC_34_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_33(&mut self) -> WRT_PROT_SEC_33_W {
        WRT_PROT_SEC_33_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_32(&mut self) -> WRT_PROT_SEC_32_W {
        WRT_PROT_SEC_32_W { w: self }
    }
}
