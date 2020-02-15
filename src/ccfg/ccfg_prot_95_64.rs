#[doc = "Reader of register CCFG_PROT_95_64"]
pub type R = crate::R<u32, super::CCFG_PROT_95_64>;
#[doc = "Writer for register CCFG_PROT_95_64"]
pub type W = crate::W<u32, super::CCFG_PROT_95_64>;
#[doc = "Register CCFG_PROT_95_64 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CCFG_PROT_95_64 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `WRT_PROT_SEC_95`"]
pub type WRT_PROT_SEC_95_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_95`"]
pub struct WRT_PROT_SEC_95_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_95_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_94`"]
pub type WRT_PROT_SEC_94_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_94`"]
pub struct WRT_PROT_SEC_94_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_94_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_93`"]
pub type WRT_PROT_SEC_93_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_93`"]
pub struct WRT_PROT_SEC_93_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_93_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_92`"]
pub type WRT_PROT_SEC_92_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_92`"]
pub struct WRT_PROT_SEC_92_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_92_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_91`"]
pub type WRT_PROT_SEC_91_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_91`"]
pub struct WRT_PROT_SEC_91_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_91_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_90`"]
pub type WRT_PROT_SEC_90_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_90`"]
pub struct WRT_PROT_SEC_90_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_90_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_89`"]
pub type WRT_PROT_SEC_89_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_89`"]
pub struct WRT_PROT_SEC_89_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_89_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_88`"]
pub type WRT_PROT_SEC_88_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_88`"]
pub struct WRT_PROT_SEC_88_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_88_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_87`"]
pub type WRT_PROT_SEC_87_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_87`"]
pub struct WRT_PROT_SEC_87_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_87_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_86`"]
pub type WRT_PROT_SEC_86_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_86`"]
pub struct WRT_PROT_SEC_86_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_86_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_85`"]
pub type WRT_PROT_SEC_85_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_85`"]
pub struct WRT_PROT_SEC_85_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_85_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_84`"]
pub type WRT_PROT_SEC_84_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_84`"]
pub struct WRT_PROT_SEC_84_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_84_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_83`"]
pub type WRT_PROT_SEC_83_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_83`"]
pub struct WRT_PROT_SEC_83_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_83_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_82`"]
pub type WRT_PROT_SEC_82_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_82`"]
pub struct WRT_PROT_SEC_82_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_82_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_81`"]
pub type WRT_PROT_SEC_81_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_81`"]
pub struct WRT_PROT_SEC_81_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_81_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_80`"]
pub type WRT_PROT_SEC_80_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_80`"]
pub struct WRT_PROT_SEC_80_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_80_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_79`"]
pub type WRT_PROT_SEC_79_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_79`"]
pub struct WRT_PROT_SEC_79_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_79_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_78`"]
pub type WRT_PROT_SEC_78_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_78`"]
pub struct WRT_PROT_SEC_78_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_78_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_77`"]
pub type WRT_PROT_SEC_77_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_77`"]
pub struct WRT_PROT_SEC_77_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_77_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_76`"]
pub type WRT_PROT_SEC_76_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_76`"]
pub struct WRT_PROT_SEC_76_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_76_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_75`"]
pub type WRT_PROT_SEC_75_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_75`"]
pub struct WRT_PROT_SEC_75_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_75_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_74`"]
pub type WRT_PROT_SEC_74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_74`"]
pub struct WRT_PROT_SEC_74_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_74_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_73`"]
pub type WRT_PROT_SEC_73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_73`"]
pub struct WRT_PROT_SEC_73_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_73_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_72`"]
pub type WRT_PROT_SEC_72_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_72`"]
pub struct WRT_PROT_SEC_72_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_72_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_71`"]
pub type WRT_PROT_SEC_71_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_71`"]
pub struct WRT_PROT_SEC_71_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_71_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_70`"]
pub type WRT_PROT_SEC_70_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_70`"]
pub struct WRT_PROT_SEC_70_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_70_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_69`"]
pub type WRT_PROT_SEC_69_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_69`"]
pub struct WRT_PROT_SEC_69_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_69_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_68`"]
pub type WRT_PROT_SEC_68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_68`"]
pub struct WRT_PROT_SEC_68_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_68_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_67`"]
pub type WRT_PROT_SEC_67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_67`"]
pub struct WRT_PROT_SEC_67_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_67_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_66`"]
pub type WRT_PROT_SEC_66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_66`"]
pub struct WRT_PROT_SEC_66_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_66_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_65`"]
pub type WRT_PROT_SEC_65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_65`"]
pub struct WRT_PROT_SEC_65_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_65_W<'a> {
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
#[doc = "Reader of field `WRT_PROT_SEC_64`"]
pub type WRT_PROT_SEC_64_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRT_PROT_SEC_64`"]
pub struct WRT_PROT_SEC_64_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_64_W<'a> {
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
    pub fn wrt_prot_sec_95(&self) -> WRT_PROT_SEC_95_R {
        WRT_PROT_SEC_95_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_94(&self) -> WRT_PROT_SEC_94_R {
        WRT_PROT_SEC_94_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_93(&self) -> WRT_PROT_SEC_93_R {
        WRT_PROT_SEC_93_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_92(&self) -> WRT_PROT_SEC_92_R {
        WRT_PROT_SEC_92_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_91(&self) -> WRT_PROT_SEC_91_R {
        WRT_PROT_SEC_91_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_90(&self) -> WRT_PROT_SEC_90_R {
        WRT_PROT_SEC_90_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_89(&self) -> WRT_PROT_SEC_89_R {
        WRT_PROT_SEC_89_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_88(&self) -> WRT_PROT_SEC_88_R {
        WRT_PROT_SEC_88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_87(&self) -> WRT_PROT_SEC_87_R {
        WRT_PROT_SEC_87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_86(&self) -> WRT_PROT_SEC_86_R {
        WRT_PROT_SEC_86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_85(&self) -> WRT_PROT_SEC_85_R {
        WRT_PROT_SEC_85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_84(&self) -> WRT_PROT_SEC_84_R {
        WRT_PROT_SEC_84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_83(&self) -> WRT_PROT_SEC_83_R {
        WRT_PROT_SEC_83_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_82(&self) -> WRT_PROT_SEC_82_R {
        WRT_PROT_SEC_82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_81(&self) -> WRT_PROT_SEC_81_R {
        WRT_PROT_SEC_81_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_80(&self) -> WRT_PROT_SEC_80_R {
        WRT_PROT_SEC_80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_79(&self) -> WRT_PROT_SEC_79_R {
        WRT_PROT_SEC_79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_78(&self) -> WRT_PROT_SEC_78_R {
        WRT_PROT_SEC_78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_77(&self) -> WRT_PROT_SEC_77_R {
        WRT_PROT_SEC_77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_76(&self) -> WRT_PROT_SEC_76_R {
        WRT_PROT_SEC_76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_75(&self) -> WRT_PROT_SEC_75_R {
        WRT_PROT_SEC_75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_74(&self) -> WRT_PROT_SEC_74_R {
        WRT_PROT_SEC_74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_73(&self) -> WRT_PROT_SEC_73_R {
        WRT_PROT_SEC_73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_72(&self) -> WRT_PROT_SEC_72_R {
        WRT_PROT_SEC_72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_71(&self) -> WRT_PROT_SEC_71_R {
        WRT_PROT_SEC_71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_70(&self) -> WRT_PROT_SEC_70_R {
        WRT_PROT_SEC_70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_69(&self) -> WRT_PROT_SEC_69_R {
        WRT_PROT_SEC_69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_68(&self) -> WRT_PROT_SEC_68_R {
        WRT_PROT_SEC_68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_67(&self) -> WRT_PROT_SEC_67_R {
        WRT_PROT_SEC_67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_66(&self) -> WRT_PROT_SEC_66_R {
        WRT_PROT_SEC_66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_65(&self) -> WRT_PROT_SEC_65_R {
        WRT_PROT_SEC_65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_64(&self) -> WRT_PROT_SEC_64_R {
        WRT_PROT_SEC_64_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_95(&mut self) -> WRT_PROT_SEC_95_W {
        WRT_PROT_SEC_95_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_94(&mut self) -> WRT_PROT_SEC_94_W {
        WRT_PROT_SEC_94_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_93(&mut self) -> WRT_PROT_SEC_93_W {
        WRT_PROT_SEC_93_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_92(&mut self) -> WRT_PROT_SEC_92_W {
        WRT_PROT_SEC_92_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_91(&mut self) -> WRT_PROT_SEC_91_W {
        WRT_PROT_SEC_91_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_90(&mut self) -> WRT_PROT_SEC_90_W {
        WRT_PROT_SEC_90_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_89(&mut self) -> WRT_PROT_SEC_89_W {
        WRT_PROT_SEC_89_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_88(&mut self) -> WRT_PROT_SEC_88_W {
        WRT_PROT_SEC_88_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_87(&mut self) -> WRT_PROT_SEC_87_W {
        WRT_PROT_SEC_87_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_86(&mut self) -> WRT_PROT_SEC_86_W {
        WRT_PROT_SEC_86_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_85(&mut self) -> WRT_PROT_SEC_85_W {
        WRT_PROT_SEC_85_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_84(&mut self) -> WRT_PROT_SEC_84_W {
        WRT_PROT_SEC_84_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_83(&mut self) -> WRT_PROT_SEC_83_W {
        WRT_PROT_SEC_83_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_82(&mut self) -> WRT_PROT_SEC_82_W {
        WRT_PROT_SEC_82_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_81(&mut self) -> WRT_PROT_SEC_81_W {
        WRT_PROT_SEC_81_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_80(&mut self) -> WRT_PROT_SEC_80_W {
        WRT_PROT_SEC_80_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_79(&mut self) -> WRT_PROT_SEC_79_W {
        WRT_PROT_SEC_79_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_78(&mut self) -> WRT_PROT_SEC_78_W {
        WRT_PROT_SEC_78_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_77(&mut self) -> WRT_PROT_SEC_77_W {
        WRT_PROT_SEC_77_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_76(&mut self) -> WRT_PROT_SEC_76_W {
        WRT_PROT_SEC_76_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_75(&mut self) -> WRT_PROT_SEC_75_W {
        WRT_PROT_SEC_75_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_74(&mut self) -> WRT_PROT_SEC_74_W {
        WRT_PROT_SEC_74_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_73(&mut self) -> WRT_PROT_SEC_73_W {
        WRT_PROT_SEC_73_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_72(&mut self) -> WRT_PROT_SEC_72_W {
        WRT_PROT_SEC_72_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_71(&mut self) -> WRT_PROT_SEC_71_W {
        WRT_PROT_SEC_71_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_70(&mut self) -> WRT_PROT_SEC_70_W {
        WRT_PROT_SEC_70_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_69(&mut self) -> WRT_PROT_SEC_69_W {
        WRT_PROT_SEC_69_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_68(&mut self) -> WRT_PROT_SEC_68_W {
        WRT_PROT_SEC_68_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_67(&mut self) -> WRT_PROT_SEC_67_W {
        WRT_PROT_SEC_67_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_66(&mut self) -> WRT_PROT_SEC_66_W {
        WRT_PROT_SEC_66_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_65(&mut self) -> WRT_PROT_SEC_65_W {
        WRT_PROT_SEC_65_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_64(&mut self) -> WRT_PROT_SEC_64_W {
        WRT_PROT_SEC_64_W { w: self }
    }
}
