#[doc = "Reader of register RFCPEISL"]
pub type R = crate::R<u32, super::RFCPEISL>;
#[doc = "Writer for register RFCPEISL"]
pub type W = crate::W<u32, super::RFCPEISL>;
#[doc = "Register RFCPEISL `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::RFCPEISL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_ERROR_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<INTERNAL_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTERNAL_ERROR`"]
pub type INTERNAL_ERROR_R = crate::R<bool, INTERNAL_ERROR_A>;
impl INTERNAL_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERNAL_ERROR_A {
        match self.bits {
            true => INTERNAL_ERROR_A::CPE1,
            false => INTERNAL_ERROR_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == INTERNAL_ERROR_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == INTERNAL_ERROR_A::CPE0
    }
}
#[doc = "Write proxy for field `INTERNAL_ERROR`"]
pub struct INTERNAL_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERNAL_ERROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(INTERNAL_ERROR_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(INTERNAL_ERROR_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<BOOT_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_DONE`"]
pub type BOOT_DONE_R = crate::R<bool, BOOT_DONE_A>;
impl BOOT_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_DONE_A {
        match self.bits {
            true => BOOT_DONE_A::CPE1,
            false => BOOT_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == BOOT_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == BOOT_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `BOOT_DONE`"]
pub struct BOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(BOOT_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(BOOT_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODULES_UNLOCKED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<MODULES_UNLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: MODULES_UNLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODULES_UNLOCKED`"]
pub type MODULES_UNLOCKED_R = crate::R<bool, MODULES_UNLOCKED_A>;
impl MODULES_UNLOCKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODULES_UNLOCKED_A {
        match self.bits {
            true => MODULES_UNLOCKED_A::CPE1,
            false => MODULES_UNLOCKED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == MODULES_UNLOCKED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == MODULES_UNLOCKED_A::CPE0
    }
}
#[doc = "Write proxy for field `MODULES_UNLOCKED`"]
pub struct MODULES_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULES_UNLOCKED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODULES_UNLOCKED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNTH_NO_LOCK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<SYNTH_NO_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SYNTH_NO_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNTH_NO_LOCK`"]
pub type SYNTH_NO_LOCK_R = crate::R<bool, SYNTH_NO_LOCK_A>;
impl SYNTH_NO_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNTH_NO_LOCK_A {
        match self.bits {
            true => SYNTH_NO_LOCK_A::CPE1,
            false => SYNTH_NO_LOCK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == SYNTH_NO_LOCK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == SYNTH_NO_LOCK_A::CPE0
    }
}
#[doc = "Write proxy for field `SYNTH_NO_LOCK`"]
pub struct SYNTH_NO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNTH_NO_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNTH_NO_LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ27_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ27_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ27`"]
pub type IRQ27_R = crate::R<bool, IRQ27_A>;
impl IRQ27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ27_A {
        match self.bits {
            true => IRQ27_A::CPE1,
            false => IRQ27_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ27_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ27_A::CPE0
    }
}
#[doc = "Write proxy for field `IRQ27`"]
pub struct IRQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ27_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ27_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ABORTED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_ABORTED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ABORTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_ABORTED`"]
pub type RX_ABORTED_R = crate::R<bool, RX_ABORTED_A>;
impl RX_ABORTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ABORTED_A {
        match self.bits {
            true => RX_ABORTED_A::CPE1,
            false => RX_ABORTED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_ABORTED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_ABORTED_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_ABORTED`"]
pub struct RX_ABORTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ABORTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ABORTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ABORTED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ABORTED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_N_DATA_WRITTEN_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_N_DATA_WRITTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_N_DATA_WRITTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_N_DATA_WRITTEN`"]
pub type RX_N_DATA_WRITTEN_R = crate::R<bool, RX_N_DATA_WRITTEN_A>;
impl RX_N_DATA_WRITTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_N_DATA_WRITTEN_A {
        match self.bits {
            true => RX_N_DATA_WRITTEN_A::CPE1,
            false => RX_N_DATA_WRITTEN_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_N_DATA_WRITTEN_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_N_DATA_WRITTEN_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_N_DATA_WRITTEN`"]
pub struct RX_N_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_N_DATA_WRITTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_N_DATA_WRITTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTEN_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTEN_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_WRITTEN_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_DATA_WRITTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DATA_WRITTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_DATA_WRITTEN`"]
pub type RX_DATA_WRITTEN_R = crate::R<bool, RX_DATA_WRITTEN_A>;
impl RX_DATA_WRITTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DATA_WRITTEN_A {
        match self.bits {
            true => RX_DATA_WRITTEN_A::CPE1,
            false => RX_DATA_WRITTEN_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_DATA_WRITTEN_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_DATA_WRITTEN_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_DATA_WRITTEN`"]
pub struct RX_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_WRITTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DATA_WRITTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTEN_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTEN_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ENTRY_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_ENTRY_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ENTRY_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_ENTRY_DONE`"]
pub type RX_ENTRY_DONE_R = crate::R<bool, RX_ENTRY_DONE_A>;
impl RX_ENTRY_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ENTRY_DONE_A {
        match self.bits {
            true => RX_ENTRY_DONE_A::CPE1,
            false => RX_ENTRY_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_ENTRY_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_ENTRY_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_ENTRY_DONE`"]
pub struct RX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENTRY_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ENTRY_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BUF_FULL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_BUF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BUF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_BUF_FULL`"]
pub type RX_BUF_FULL_R = crate::R<bool, RX_BUF_FULL_A>;
impl RX_BUF_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BUF_FULL_A {
        match self.bits {
            true => RX_BUF_FULL_A::CPE1,
            false => RX_BUF_FULL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_BUF_FULL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_BUF_FULL_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_BUF_FULL`"]
pub struct RX_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BUF_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_BUF_FULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_BUF_FULL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_BUF_FULL_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CTRL_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_CTRL_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CTRL_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_CTRL_ACK`"]
pub type RX_CTRL_ACK_R = crate::R<bool, RX_CTRL_ACK_A>;
impl RX_CTRL_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CTRL_ACK_A {
        match self.bits {
            true => RX_CTRL_ACK_A::CPE1,
            false => RX_CTRL_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_CTRL_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_CTRL_ACK_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_CTRL_ACK`"]
pub struct RX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_CTRL_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CTRL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_CTRL`"]
pub type RX_CTRL_R = crate::R<bool, RX_CTRL_A>;
impl RX_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CTRL_A {
        match self.bits {
            true => RX_CTRL_A::CPE1,
            false => RX_CTRL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_CTRL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_CTRL_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_CTRL`"]
pub struct RX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EMPTY_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_EMPTY`"]
pub type RX_EMPTY_R = crate::R<bool, RX_EMPTY_A>;
impl RX_EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMPTY_A {
        match self.bits {
            true => RX_EMPTY_A::CPE1,
            false => RX_EMPTY_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_EMPTY_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_EMPTY_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_EMPTY`"]
pub struct RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EMPTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_EMPTY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_EMPTY_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_EMPTY_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_IGNORED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_IGNORED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_IGNORED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_IGNORED`"]
pub type RX_IGNORED_R = crate::R<bool, RX_IGNORED_A>;
impl RX_IGNORED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_IGNORED_A {
        match self.bits {
            true => RX_IGNORED_A::CPE1,
            false => RX_IGNORED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_IGNORED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_IGNORED_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_IGNORED`"]
pub struct RX_IGNORED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IGNORED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_IGNORED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_IGNORED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_IGNORED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_NOK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_NOK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_NOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_NOK`"]
pub type RX_NOK_R = crate::R<bool, RX_NOK_A>;
impl RX_NOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_NOK_A {
        match self.bits {
            true => RX_NOK_A::CPE1,
            false => RX_NOK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_NOK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_NOK_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_NOK`"]
pub struct RX_NOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_NOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_NOK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_NOK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_OK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_OK`"]
pub type RX_OK_R = crate::R<bool, RX_OK_A>;
impl RX_OK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OK_A {
        match self.bits {
            true => RX_OK_A::CPE1,
            false => RX_OK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == RX_OK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == RX_OK_A::CPE0
    }
}
#[doc = "Write proxy for field `RX_OK`"]
pub struct RX_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_OK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_OK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ15_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ15_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ15`"]
pub type IRQ15_R = crate::R<bool, IRQ15_A>;
impl IRQ15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ15_A {
        match self.bits {
            true => IRQ15_A::CPE1,
            false => IRQ15_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ15_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ15_A::CPE0
    }
}
#[doc = "Write proxy for field `IRQ15`"]
pub struct IRQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ15_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ15_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ14_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ14_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRQ14`"]
pub type IRQ14_R = crate::R<bool, IRQ14_A>;
impl IRQ14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ14_A {
        match self.bits {
            true => IRQ14_A::CPE1,
            false => IRQ14_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == IRQ14_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == IRQ14_A::CPE0
    }
}
#[doc = "Write proxy for field `IRQ14`"]
pub struct IRQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ14_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ14_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "13:13\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_STARTED interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FG_COMMAND_STARTED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<FG_COMMAND_STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: FG_COMMAND_STARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FG_COMMAND_STARTED`"]
pub type FG_COMMAND_STARTED_R = crate::R<bool, FG_COMMAND_STARTED_A>;
impl FG_COMMAND_STARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FG_COMMAND_STARTED_A {
        match self.bits {
            true => FG_COMMAND_STARTED_A::CPE1,
            false => FG_COMMAND_STARTED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == FG_COMMAND_STARTED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == FG_COMMAND_STARTED_A::CPE0
    }
}
#[doc = "Write proxy for field `FG_COMMAND_STARTED`"]
pub struct FG_COMMAND_STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_STARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FG_COMMAND_STARTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(FG_COMMAND_STARTED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(FG_COMMAND_STARTED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "12:12\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_STARTED interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_STARTED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<COMMAND_STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: COMMAND_STARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMMAND_STARTED`"]
pub type COMMAND_STARTED_R = crate::R<bool, COMMAND_STARTED_A>;
impl COMMAND_STARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMMAND_STARTED_A {
        match self.bits {
            true => COMMAND_STARTED_A::CPE1,
            false => COMMAND_STARTED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == COMMAND_STARTED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == COMMAND_STARTED_A::CPE0
    }
}
#[doc = "Write proxy for field `COMMAND_STARTED`"]
pub struct COMMAND_STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_STARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMMAND_STARTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(COMMAND_STARTED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(COMMAND_STARTED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_BUFFER_CHANGED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_BUFFER_CHANGED_A> for bool {
    #[inline(always)]
    fn from(variant: TX_BUFFER_CHANGED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_BUFFER_CHANGED`"]
pub type TX_BUFFER_CHANGED_R = crate::R<bool, TX_BUFFER_CHANGED_A>;
impl TX_BUFFER_CHANGED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_BUFFER_CHANGED_A {
        match self.bits {
            true => TX_BUFFER_CHANGED_A::CPE1,
            false => TX_BUFFER_CHANGED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_BUFFER_CHANGED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_BUFFER_CHANGED_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_BUFFER_CHANGED`"]
pub struct TX_BUFFER_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BUFFER_CHANGED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_BUFFER_CHANGED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ENTRY_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_ENTRY_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ENTRY_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_ENTRY_DONE`"]
pub type TX_ENTRY_DONE_R = crate::R<bool, TX_ENTRY_DONE_A>;
impl TX_ENTRY_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ENTRY_DONE_A {
        match self.bits {
            true => TX_ENTRY_DONE_A::CPE1,
            false => TX_ENTRY_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_ENTRY_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_ENTRY_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_ENTRY_DONE`"]
pub struct TX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRY_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ENTRY_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_RETRANS_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_RETRANS_A> for bool {
    #[inline(always)]
    fn from(variant: TX_RETRANS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_RETRANS`"]
pub type TX_RETRANS_R = crate::R<bool, TX_RETRANS_A>;
impl TX_RETRANS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_RETRANS_A {
        match self.bits {
            true => TX_RETRANS_A::CPE1,
            false => TX_RETRANS_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_RETRANS_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_RETRANS_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_RETRANS`"]
pub struct TX_RETRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RETRANS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_RETRANS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_RETRANS_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_RETRANS_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_ACK_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_ACK_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_ACK_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_CTRL_ACK_ACK`"]
pub type TX_CTRL_ACK_ACK_R = crate::R<bool, TX_CTRL_ACK_ACK_A>;
impl TX_CTRL_ACK_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_ACK_ACK_A {
        match self.bits {
            true => TX_CTRL_ACK_ACK_A::CPE1,
            false => TX_CTRL_ACK_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_ACK_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_ACK_ACK_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_CTRL_ACK_ACK`"]
pub struct TX_CTRL_ACK_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CTRL_ACK_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_CTRL_ACK`"]
pub type TX_CTRL_ACK_R = crate::R<bool, TX_CTRL_ACK_A>;
impl TX_CTRL_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_ACK_A {
        match self.bits {
            true => TX_CTRL_ACK_A::CPE1,
            false => TX_CTRL_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_ACK_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_CTRL_ACK`"]
pub struct TX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CTRL_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_CTRL`"]
pub type TX_CTRL_R = crate::R<bool, TX_CTRL_A>;
impl TX_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_A {
        match self.bits {
            true => TX_CTRL_A::CPE1,
            false => TX_CTRL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_CTRL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_CTRL_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_CTRL`"]
pub struct TX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_ACK`"]
pub type TX_ACK_R = crate::R<bool, TX_ACK_A>;
impl TX_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ACK_A {
        match self.bits {
            true => TX_ACK_A::CPE1,
            false => TX_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_ACK_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_ACK`"]
pub struct TX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_DONE`"]
pub type TX_DONE_R = crate::R<bool, TX_DONE_A>;
impl TX_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DONE_A {
        match self.bits {
            true => TX_DONE_A::CPE1,
            false => TX_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == TX_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == TX_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `TX_DONE`"]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_FG_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<LAST_FG_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_FG_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LAST_FG_COMMAND_DONE`"]
pub type LAST_FG_COMMAND_DONE_R = crate::R<bool, LAST_FG_COMMAND_DONE_A>;
impl LAST_FG_COMMAND_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_FG_COMMAND_DONE_A {
        match self.bits {
            true => LAST_FG_COMMAND_DONE_A::CPE1,
            false => LAST_FG_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == LAST_FG_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == LAST_FG_COMMAND_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `LAST_FG_COMMAND_DONE`"]
pub struct LAST_FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_FG_COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_FG_COMMAND_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FG_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<FG_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: FG_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FG_COMMAND_DONE`"]
pub type FG_COMMAND_DONE_R = crate::R<bool, FG_COMMAND_DONE_A>;
impl FG_COMMAND_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FG_COMMAND_DONE_A {
        match self.bits {
            true => FG_COMMAND_DONE_A::CPE1,
            false => FG_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == FG_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == FG_COMMAND_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `FG_COMMAND_DONE`"]
pub struct FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FG_COMMAND_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<LAST_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LAST_COMMAND_DONE`"]
pub type LAST_COMMAND_DONE_R = crate::R<bool, LAST_COMMAND_DONE_A>;
impl LAST_COMMAND_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_COMMAND_DONE_A {
        match self.bits {
            true => LAST_COMMAND_DONE_A::CPE1,
            false => LAST_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == LAST_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == LAST_COMMAND_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `LAST_COMMAND_DONE`"]
pub struct LAST_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_COMMAND_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMMAND_DONE`"]
pub type COMMAND_DONE_R = crate::R<bool, COMMAND_DONE_A>;
impl COMMAND_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMMAND_DONE_A {
        match self.bits {
            true => COMMAND_DONE_A::CPE1,
            false => COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        *self == COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        *self == COMMAND_DONE_A::CPE0
    }
}
#[doc = "Write proxy for field `COMMAND_DONE`"]
pub struct COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMMAND_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_STARTED interrupt should use."]
    #[inline(always)]
    pub fn fg_command_started(&self) -> FG_COMMAND_STARTED_R {
        FG_COMMAND_STARTED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_STARTED interrupt should use."]
    #[inline(always)]
    pub fn command_started(&self) -> COMMAND_STARTED_R {
        COMMAND_STARTED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W {
        INTERNAL_ERROR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub fn boot_done(&mut self) -> BOOT_DONE_W {
        BOOT_DONE_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W {
        MODULES_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W {
        SYNTH_NO_LOCK_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub fn irq27(&mut self) -> IRQ27_W {
        IRQ27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W {
        RX_ABORTED_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W {
        RX_N_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W {
        RX_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W {
        RX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W {
        RX_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W {
        RX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W {
        RX_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W {
        RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W {
        RX_IGNORED_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub fn rx_nok(&mut self) -> RX_NOK_W {
        RX_NOK_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RX_OK_W {
        RX_OK_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub fn irq15(&mut self) -> IRQ15_W {
        IRQ15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub fn irq14(&mut self) -> IRQ14_W {
        IRQ14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_STARTED interrupt should use."]
    #[inline(always)]
    pub fn fg_command_started(&mut self) -> FG_COMMAND_STARTED_W {
        FG_COMMAND_STARTED_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_STARTED interrupt should use."]
    #[inline(always)]
    pub fn command_started(&mut self) -> COMMAND_STARTED_W {
        COMMAND_STARTED_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W {
        TX_BUFFER_CHANGED_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W {
        TX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W {
        TX_RETRANS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W {
        TX_CTRL_ACK_ACK_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W {
        TX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W {
        TX_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ack(&mut self) -> TX_ACK_W {
        TX_ACK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W {
        LAST_FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W {
        FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W {
        LAST_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn command_done(&mut self) -> COMMAND_DONE_W {
        COMMAND_DONE_W { w: self }
    }
}
