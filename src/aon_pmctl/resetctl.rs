#[doc = "Reader of register RESETCTL"]
pub type R = crate::R<u32, super::RESETCTL>;
#[doc = "Writer for register RESETCTL"]
pub type W = crate::W<u32, super::RESETCTL>;
#[doc = "Register RESETCTL `reset()`'s with value 0x01c0"]
impl crate::ResetValue for super::RESETCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01c0
    }
}
#[doc = "Reader of field `SYSRESET`"]
pub type SYSRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRESET`"]
pub struct SYSRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `RESERVED26`"]
pub type RESERVED26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DET_1_CLR`"]
pub type BOOT_DET_1_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_1_CLR`"]
pub struct BOOT_DET_1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_1_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BOOT_DET_0_CLR`"]
pub type BOOT_DET_0_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_0_CLR`"]
pub struct BOOT_DET_0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_0_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `RESERVED18`"]
pub type RESERVED18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED18`"]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DET_1_SET`"]
pub type BOOT_DET_1_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_1_SET`"]
pub struct BOOT_DET_1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_1_SET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BOOT_DET_0_SET`"]
pub type BOOT_DET_0_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_0_SET`"]
pub struct BOOT_DET_0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_0_SET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `WU_FROM_SD`"]
pub type WU_FROM_SD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WU_FROM_SD`"]
pub struct WU_FROM_SD_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_FROM_SD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `GPIO_WU_FROM_SD`"]
pub type GPIO_WU_FROM_SD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_WU_FROM_SD`"]
pub struct GPIO_WU_FROM_SD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WU_FROM_SD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BOOT_DET_1`"]
pub type BOOT_DET_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_1`"]
pub struct BOOT_DET_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `BOOT_DET_0`"]
pub type BOOT_DET_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_0`"]
pub struct BOOT_DET_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `VDDS_LOSS_EN`"]
pub type VDDS_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDS_LOSS_EN`"]
pub struct VDDS_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDS_LOSS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `VDDR_LOSS_EN`"]
pub type VDDR_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_LOSS_EN`"]
pub struct VDDR_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_LOSS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `VDD_LOSS_EN`"]
pub type VDD_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDD_LOSS_EN`"]
pub struct VDD_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_LOSS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CLK_LOSS_EN`"]
pub type CLK_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_LOSS_EN`"]
pub struct CLK_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LOSS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `MCU_WARM_RESET`"]
pub type MCU_WARM_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_WARM_RESET`"]
pub struct MCU_WARM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WARM_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESET_SRC_A {
    #[doc = "7: Software reset via PRCM warm reset request"]
    WARMRESET = 7,
    #[doc = "6: Software reset via SYSRESET or hardware power management timeout detection.\n\nNote: The hardware power management timeout circuit is always enabled."]
    SYSRESET = 6,
    #[doc = "5: SCLK_LF, SCLK_MF or SCLK_HF clock loss detect"]
    CLK_LOSS = 5,
    #[doc = "4: Brown out detect on VDDR"]
    VDDR_LOSS = 4,
    #[doc = "2: Brown out detect on VDDS"]
    VDDS_LOSS = 2,
    #[doc = "1: Reset pin"]
    PIN_RESET = 1,
    #[doc = "0: Power on reset"]
    PWR_ON = 0,
}
impl From<RESET_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RESET_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESET_SRC`"]
pub type RESET_SRC_R = crate::R<u8, RESET_SRC_A>;
impl RESET_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RESET_SRC_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(RESET_SRC_A::WARMRESET),
            6 => Val(RESET_SRC_A::SYSRESET),
            5 => Val(RESET_SRC_A::CLK_LOSS),
            4 => Val(RESET_SRC_A::VDDR_LOSS),
            2 => Val(RESET_SRC_A::VDDS_LOSS),
            1 => Val(RESET_SRC_A::PIN_RESET),
            0 => Val(RESET_SRC_A::PWR_ON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WARMRESET`"]
    #[inline(always)]
    pub fn is_warmreset(&self) -> bool {
        *self == RESET_SRC_A::WARMRESET
    }
    #[doc = "Checks if the value of the field is `SYSRESET`"]
    #[inline(always)]
    pub fn is_sysreset(&self) -> bool {
        *self == RESET_SRC_A::SYSRESET
    }
    #[doc = "Checks if the value of the field is `CLK_LOSS`"]
    #[inline(always)]
    pub fn is_clk_loss(&self) -> bool {
        *self == RESET_SRC_A::CLK_LOSS
    }
    #[doc = "Checks if the value of the field is `VDDR_LOSS`"]
    #[inline(always)]
    pub fn is_vddr_loss(&self) -> bool {
        *self == RESET_SRC_A::VDDR_LOSS
    }
    #[doc = "Checks if the value of the field is `VDDS_LOSS`"]
    #[inline(always)]
    pub fn is_vdds_loss(&self) -> bool {
        *self == RESET_SRC_A::VDDS_LOSS
    }
    #[doc = "Checks if the value of the field is `PIN_RESET`"]
    #[inline(always)]
    pub fn is_pin_reset(&self) -> bool {
        *self == RESET_SRC_A::PIN_RESET
    }
    #[doc = "Checks if the value of the field is `PWR_ON`"]
    #[inline(always)]
    pub fn is_pwr_on(&self) -> bool {
        *self == RESET_SRC_A::PWR_ON
    }
}
#[doc = "Write proxy for field `RESET_SRC`"]
pub struct RESET_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software reset via PRCM warm reset request"]
    #[inline(always)]
    pub fn warmreset(self) -> &'a mut W {
        self.variant(RESET_SRC_A::WARMRESET)
    }
    #[doc = "Software reset via SYSRESET or hardware power management timeout detection. Note: The hardware power management timeout circuit is always enabled."]
    #[inline(always)]
    pub fn sysreset(self) -> &'a mut W {
        self.variant(RESET_SRC_A::SYSRESET)
    }
    #[doc = "SCLK_LF, SCLK_MF or SCLK_HF clock loss detect"]
    #[inline(always)]
    pub fn clk_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::CLK_LOSS)
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline(always)]
    pub fn vddr_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::VDDR_LOSS)
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline(always)]
    pub fn vdds_loss(self) -> &'a mut W {
        self.variant(RESET_SRC_A::VDDS_LOSS)
    }
    #[doc = "Reset pin"]
    #[inline(always)]
    pub fn pin_reset(self) -> &'a mut W {
        self.variant(RESET_SRC_A::PIN_RESET)
    }
    #[doc = "Power on reset"]
    #[inline(always)]
    pub fn pwr_on(self) -> &'a mut W {
        self.variant(RESET_SRC_A::PWR_ON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC"]
    #[inline(always)]
    pub fn sysreset(&self) -> SYSRESET_R {
        SYSRESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_clr(&self) -> BOOT_DET_1_CLR_R {
        BOOT_DET_1_CLR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_clr(&self) -> BOOT_DET_0_CLR_R {
        BOOT_DET_0_CLR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_set(&self) -> BOOT_DET_1_SET_R {
        BOOT_DET_1_SET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_set(&self) -> BOOT_DET_0_SET_R {
        BOOT_DET_0_SET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    pub fn wu_from_sd(&self) -> WU_FROM_SD_R {
        WU_FROM_SD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    pub fn gpio_wu_from_sd(&self) -> GPIO_WU_FROM_SD_R {
        GPIO_WU_FROM_SD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1(&self) -> BOOT_DET_1_R {
        BOOT_DET_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0(&self) -> BOOT_DET_0_R {
        BOOT_DET_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
    #[inline(always)]
    pub fn vdds_loss_en(&self) -> VDDS_LOSS_EN_R {
        VDDS_LOSS_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
    #[inline(always)]
    pub fn vddr_loss_en(&self) -> VDDR_LOSS_EN_R {
        VDDR_LOSS_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
    #[inline(always)]
    pub fn vdd_loss_en(&self) -> VDD_LOSS_EN_R {
        VDD_LOSS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls reset generation in case SCLK_LF, SCLK_MF or SCLK_HF is lost when clock loss detection is enabled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]
0: Clock loss is ignored 1: Clock loss generates system reset Note: Clock loss reset generation must be disabled when changing clock source for SCLK_LF. Failure to do so may result in a spurious system reset. Clock loss reset generation is controlled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]"]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mcu_warm_reset(&self) -> MCU_WARM_RESET_R {
        MCU_WARM_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause."]
    #[inline(always)]
    pub fn reset_src(&self) -> RESET_SRC_R {
        RESET_SRC_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SYSRESET_W {
        SYSRESET_W { w: self }
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_clr(&mut self) -> BOOT_DET_1_CLR_W {
        BOOT_DET_1_CLR_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_clr(&mut self) -> BOOT_DET_0_CLR_W {
        BOOT_DET_0_CLR_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1_set(&mut self) -> BOOT_DET_1_SET_W {
        BOOT_DET_1_SET_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0_set(&mut self) -> BOOT_DET_0_SET_W {
        BOOT_DET_0_SET_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    pub fn wu_from_sd(&mut self) -> WU_FROM_SD_W {
        WU_FROM_SD_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
A wakeup from SHUTDOWN on an IO event has occurred Please refer to IOC:IOCFGn.WU_CFG for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag will be cleared when SLEEPCTL.IO_PAD_SLEEP_DIS is asserted."]
    #[inline(always)]
    pub fn gpio_wu_from_sd(&mut self) -> GPIO_WU_FROM_SD_W {
        GPIO_WU_FROM_SD_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_1(&mut self) -> BOOT_DET_1_W {
        BOOT_DET_1_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_det_0(&mut self) -> BOOT_DET_0_W {
        BOOT_DET_0_W { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
    #[inline(always)]
    pub fn vdds_loss_en(&mut self) -> VDDS_LOSS_EN_W {
        VDDS_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
    #[inline(always)]
    pub fn vddr_loss_en(&mut self) -> VDDR_LOSS_EN_W {
        VDDR_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
    #[inline(always)]
    pub fn vdd_loss_en(&mut self) -> VDD_LOSS_EN_W {
        VDD_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Controls reset generation in case SCLK_LF, SCLK_MF or SCLK_HF is lost when clock loss detection is enabled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]
0: Clock loss is ignored 1: Clock loss generates system reset Note: Clock loss reset generation must be disabled when changing clock source for SCLK_LF. Failure to do so may result in a spurious system reset. Clock loss reset generation is controlled by \\[ANATOP_MMAP:DDI_0_OSC:CTL0.CLK_LOSS_EN\\]"]
    #[inline(always)]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W {
        CLK_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mcu_warm_reset(&mut self) -> MCU_WARM_RESET_W {
        MCU_WARM_RESET_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\]
Shows the root cause of the last system reset. More than the reported reset source can have been active during the last system reset but only the root cause is reported. The capture feature is not rearmed until all off the possible reset sources have been released and the result has been copied to AON_PMCTL. During the copy and rearm process it is one 2MHz period in which and eventual new system reset will be reported as Power on reset regardless of the root cause."]
    #[inline(always)]
    pub fn reset_src(&mut self) -> RESET_SRC_W {
        RESET_SRC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
