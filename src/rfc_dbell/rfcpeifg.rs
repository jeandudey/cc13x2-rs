#[doc = "Reader of register RFCPEIFG"]
pub type R = crate::R<u32, super::RFCPEIFG>;
#[doc = "Writer for register RFCPEIFG"]
pub type W = crate::W<u32, super::RFCPEIFG>;
#[doc = "Register RFCPEIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCPEIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERNAL_ERROR`"]
pub type INTERNAL_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERNAL_ERROR`"]
pub struct INTERNAL_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_ERROR_W<'a> {
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
#[doc = "Reader of field `BOOT_DONE`"]
pub type BOOT_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DONE`"]
pub struct BOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DONE_W<'a> {
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
#[doc = "Reader of field `MODULES_UNLOCKED`"]
pub type MODULES_UNLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODULES_UNLOCKED`"]
pub struct MODULES_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULES_UNLOCKED_W<'a> {
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
#[doc = "Reader of field `SYNTH_NO_LOCK`"]
pub type SYNTH_NO_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNTH_NO_LOCK`"]
pub struct SYNTH_NO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNTH_NO_LOCK_W<'a> {
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
#[doc = "Reader of field `IRQ27`"]
pub type IRQ27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ27`"]
pub struct IRQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ27_W<'a> {
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
#[doc = "Reader of field `RX_ABORTED`"]
pub type RX_ABORTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ABORTED`"]
pub struct RX_ABORTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ABORTED_W<'a> {
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
#[doc = "Reader of field `RX_N_DATA_WRITTEN`"]
pub type RX_N_DATA_WRITTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_N_DATA_WRITTEN`"]
pub struct RX_N_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_N_DATA_WRITTEN_W<'a> {
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
#[doc = "Reader of field `RX_DATA_WRITTEN`"]
pub type RX_DATA_WRITTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DATA_WRITTEN`"]
pub struct RX_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_WRITTEN_W<'a> {
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
#[doc = "Reader of field `RX_ENTRY_DONE`"]
pub type RX_ENTRY_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ENTRY_DONE`"]
pub struct RX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENTRY_DONE_W<'a> {
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
#[doc = "Reader of field `RX_BUF_FULL`"]
pub type RX_BUF_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_BUF_FULL`"]
pub struct RX_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BUF_FULL_W<'a> {
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
#[doc = "Reader of field `RX_CTRL_ACK`"]
pub type RX_CTRL_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CTRL_ACK`"]
pub struct RX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_ACK_W<'a> {
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
#[doc = "Reader of field `RX_CTRL`"]
pub type RX_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CTRL`"]
pub struct RX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_W<'a> {
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
#[doc = "Reader of field `RX_EMPTY`"]
pub type RX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_EMPTY`"]
pub struct RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EMPTY_W<'a> {
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
#[doc = "Reader of field `RX_IGNORED`"]
pub type RX_IGNORED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_IGNORED`"]
pub struct RX_IGNORED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IGNORED_W<'a> {
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
#[doc = "Reader of field `RX_NOK`"]
pub type RX_NOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_NOK`"]
pub struct RX_NOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NOK_W<'a> {
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
#[doc = "Reader of field `RX_OK`"]
pub type RX_OK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_OK`"]
pub struct RX_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OK_W<'a> {
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
#[doc = "Reader of field `IRQ15`"]
pub type IRQ15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ15`"]
pub struct IRQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ15_W<'a> {
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
#[doc = "Reader of field `IRQ14`"]
pub type IRQ14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ14`"]
pub struct IRQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ14_W<'a> {
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
#[doc = "Reader of field `FG_COMMAND_STARTED`"]
pub type FG_COMMAND_STARTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FG_COMMAND_STARTED`"]
pub struct FG_COMMAND_STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_STARTED_W<'a> {
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
#[doc = "Reader of field `COMMAND_STARTED`"]
pub type COMMAND_STARTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMMAND_STARTED`"]
pub struct COMMAND_STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_STARTED_W<'a> {
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
#[doc = "Reader of field `TX_BUFFER_CHANGED`"]
pub type TX_BUFFER_CHANGED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BUFFER_CHANGED`"]
pub struct TX_BUFFER_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BUFFER_CHANGED_W<'a> {
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
#[doc = "Reader of field `TX_ENTRY_DONE`"]
pub type TX_ENTRY_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ENTRY_DONE`"]
pub struct TX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRY_DONE_W<'a> {
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
#[doc = "Reader of field `TX_RETRANS`"]
pub type TX_RETRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_RETRANS`"]
pub struct TX_RETRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RETRANS_W<'a> {
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
#[doc = "Reader of field `TX_CTRL_ACK_ACK`"]
pub type TX_CTRL_ACK_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CTRL_ACK_ACK`"]
pub struct TX_CTRL_ACK_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_ACK_W<'a> {
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
#[doc = "Reader of field `TX_CTRL_ACK`"]
pub type TX_CTRL_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CTRL_ACK`"]
pub struct TX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_W<'a> {
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
#[doc = "Reader of field `TX_CTRL`"]
pub type TX_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CTRL`"]
pub struct TX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_W<'a> {
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
#[doc = "Reader of field `TX_ACK`"]
pub type TX_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ACK`"]
pub struct TX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_W<'a> {
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
#[doc = "Reader of field `TX_DONE`"]
pub type TX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DONE`"]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
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
#[doc = "Reader of field `LAST_FG_COMMAND_DONE`"]
pub type LAST_FG_COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LAST_FG_COMMAND_DONE`"]
pub struct LAST_FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_FG_COMMAND_DONE_W<'a> {
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
#[doc = "Reader of field `FG_COMMAND_DONE`"]
pub type FG_COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FG_COMMAND_DONE`"]
pub struct FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_DONE_W<'a> {
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
#[doc = "Reader of field `LAST_COMMAND_DONE`"]
pub type LAST_COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LAST_COMMAND_DONE`"]
pub struct LAST_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_COMMAND_DONE_W<'a> {
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
#[doc = "Reader of field `COMMAND_DONE`"]
pub type COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMMAND_DONE`"]
pub struct COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_DONE_W<'a> {
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
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt flag 13. IEEE 802.15.4 mode only: A foreground radio operation command has gone into active state."]
    #[inline(always)]
    pub fn fg_command_started(&self) -> FG_COMMAND_STARTED_R {
        FG_COMMAND_STARTED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt flag 12. A radio operation command has gone into active state."]
    #[inline(always)]
    pub fn command_started(&self) -> COMMAND_STARTED_R {
        COMMAND_STARTED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W {
        INTERNAL_ERROR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn boot_done(&mut self) -> BOOT_DONE_W {
        BOOT_DONE_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W {
        MODULES_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W {
        SYNTH_NO_LOCK_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq27(&mut self) -> IRQ27_W {
        IRQ27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W {
        RX_ABORTED_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W {
        RX_N_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W {
        RX_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W {
        RX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W {
        RX_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W {
        RX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W {
        RX_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W {
        RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W {
        RX_IGNORED_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_nok(&mut self) -> RX_NOK_W {
        RX_NOK_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RX_OK_W {
        RX_OK_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq15(&mut self) -> IRQ15_W {
        IRQ15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn irq14(&mut self) -> IRQ14_W {
        IRQ14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt flag 13. IEEE 802.15.4 mode only: A foreground radio operation command has gone into active state."]
    #[inline(always)]
    pub fn fg_command_started(&mut self) -> FG_COMMAND_STARTED_W {
        FG_COMMAND_STARTED_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt flag 12. A radio operation command has gone into active state."]
    #[inline(always)]
    pub fn command_started(&mut self) -> COMMAND_STARTED_W {
        COMMAND_STARTED_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W {
        TX_BUFFER_CHANGED_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W {
        TX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W {
        TX_RETRANS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W {
        TX_CTRL_ACK_ACK_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W {
        TX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W {
        TX_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_ack(&mut self) -> TX_ACK_W {
        TX_ACK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W {
        LAST_FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W {
        FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W {
        LAST_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn command_done(&mut self) -> COMMAND_DONE_W {
        COMMAND_DONE_W { w: self }
    }
}
