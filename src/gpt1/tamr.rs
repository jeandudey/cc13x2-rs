#[doc = "Reader of register TAMR"]
pub type R = crate::R<u32, super::TAMR>;
#[doc = "Writer for register TAMR"]
pub type W = crate::W<u32, super::TAMR>;
#[doc = "Register TAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "15:13\\]
Timer Compare Action Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCACT_A {
    #[doc = "7: Clear CCP output pin immediately and set on Time-Out"]
    CLRSET_ON_TO = 7,
    #[doc = "6: Set CCP output pin immediately and clear on Time-Out"]
    SETCLR_ON_TO = 6,
    #[doc = "5: Clear CCP output pin immediately and toggle on Time-Out"]
    CLRTOG_ON_TO = 5,
    #[doc = "4: Set CCP output pin immediately and toggle on Time-Out"]
    SETTOG_ON_TO = 4,
    #[doc = "3: Set CCP output pin on Time-Out"]
    SET_ON_TO = 3,
    #[doc = "2: Clear CCP output pin on Time-Out"]
    CLR_ON_TO = 2,
    #[doc = "1: Toggle State on Time-Out"]
    TOG_ON_TO = 1,
    #[doc = "0: Disable compare operations"]
    DIS_CMP = 0,
}
impl From<TCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCACT`"]
pub type TCACT_R = crate::R<u8, TCACT_A>;
impl TCACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCACT_A {
        match self.bits {
            7 => TCACT_A::CLRSET_ON_TO,
            6 => TCACT_A::SETCLR_ON_TO,
            5 => TCACT_A::CLRTOG_ON_TO,
            4 => TCACT_A::SETTOG_ON_TO,
            3 => TCACT_A::SET_ON_TO,
            2 => TCACT_A::CLR_ON_TO,
            1 => TCACT_A::TOG_ON_TO,
            0 => TCACT_A::DIS_CMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLRSET_ON_TO`"]
    #[inline(always)]
    pub fn is_clrset_on_to(&self) -> bool {
        *self == TCACT_A::CLRSET_ON_TO
    }
    #[doc = "Checks if the value of the field is `SETCLR_ON_TO`"]
    #[inline(always)]
    pub fn is_setclr_on_to(&self) -> bool {
        *self == TCACT_A::SETCLR_ON_TO
    }
    #[doc = "Checks if the value of the field is `CLRTOG_ON_TO`"]
    #[inline(always)]
    pub fn is_clrtog_on_to(&self) -> bool {
        *self == TCACT_A::CLRTOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `SETTOG_ON_TO`"]
    #[inline(always)]
    pub fn is_settog_on_to(&self) -> bool {
        *self == TCACT_A::SETTOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `SET_ON_TO`"]
    #[inline(always)]
    pub fn is_set_on_to(&self) -> bool {
        *self == TCACT_A::SET_ON_TO
    }
    #[doc = "Checks if the value of the field is `CLR_ON_TO`"]
    #[inline(always)]
    pub fn is_clr_on_to(&self) -> bool {
        *self == TCACT_A::CLR_ON_TO
    }
    #[doc = "Checks if the value of the field is `TOG_ON_TO`"]
    #[inline(always)]
    pub fn is_tog_on_to(&self) -> bool {
        *self == TCACT_A::TOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `DIS_CMP`"]
    #[inline(always)]
    pub fn is_dis_cmp(&self) -> bool {
        *self == TCACT_A::DIS_CMP
    }
}
#[doc = "Write proxy for field `TCACT`"]
pub struct TCACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    #[inline(always)]
    pub fn clrset_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::CLRSET_ON_TO)
    }
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn setclr_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::SETCLR_ON_TO)
    }
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn clrtog_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::CLRTOG_ON_TO)
    }
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn settog_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::SETTOG_ON_TO)
    }
    #[doc = "Set CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn set_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::SET_ON_TO)
    }
    #[doc = "Clear CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn clr_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::CLR_ON_TO)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn tog_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::TOG_ON_TO)
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn dis_cmp(self) -> &'a mut W {
        self.variant(TCACT_A::DIS_CMP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "12:12\\]
One-Shot/Periodic Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACINTD_A {
    #[doc = "1: Time-out interrupt are disabled"]
    DIS_TO_INTR = 1,
    #[doc = "0: Time-out interrupt function as normal"]
    EN_TO_INTR = 0,
}
impl From<TACINTD_A> for bool {
    #[inline(always)]
    fn from(variant: TACINTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TACINTD`"]
pub type TACINTD_R = crate::R<bool, TACINTD_A>;
impl TACINTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TACINTD_A {
        match self.bits {
            true => TACINTD_A::DIS_TO_INTR,
            false => TACINTD_A::EN_TO_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_TO_INTR`"]
    #[inline(always)]
    pub fn is_dis_to_intr(&self) -> bool {
        *self == TACINTD_A::DIS_TO_INTR
    }
    #[doc = "Checks if the value of the field is `EN_TO_INTR`"]
    #[inline(always)]
    pub fn is_en_to_intr(&self) -> bool {
        *self == TACINTD_A::EN_TO_INTR
    }
}
#[doc = "Write proxy for field `TACINTD`"]
pub struct TACINTD_W<'a> {
    w: &'a mut W,
}
impl<'a> TACINTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACINTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Time-out interrupt are disabled"]
    #[inline(always)]
    pub fn dis_to_intr(self) -> &'a mut W {
        self.variant(TACINTD_A::DIS_TO_INTR)
    }
    #[doc = "Time-out interrupt function as normal"]
    #[inline(always)]
    pub fn en_to_intr(self) -> &'a mut W {
        self.variant(TACINTD_A::EN_TO_INTR)
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
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPLO_A {
    #[doc = "1: CCP output pin is set to 1 on time-out"]
    CCP_ON_TO = 1,
    #[doc = "0: Legacy operation"]
    LEGACY = 0,
}
impl From<TAPLO_A> for bool {
    #[inline(always)]
    fn from(variant: TAPLO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAPLO`"]
pub type TAPLO_R = crate::R<bool, TAPLO_A>;
impl TAPLO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPLO_A {
        match self.bits {
            true => TAPLO_A::CCP_ON_TO,
            false => TAPLO_A::LEGACY,
        }
    }
    #[doc = "Checks if the value of the field is `CCP_ON_TO`"]
    #[inline(always)]
    pub fn is_ccp_on_to(&self) -> bool {
        *self == TAPLO_A::CCP_ON_TO
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == TAPLO_A::LEGACY
    }
}
#[doc = "Write proxy for field `TAPLO`"]
pub struct TAPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPLO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline(always)]
    pub fn ccp_on_to(self) -> &'a mut W {
        self.variant(TAPLO_A::CCP_ON_TO)
    }
    #[doc = "Legacy operation"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(TAPLO_A::LEGACY)
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
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMRSU_A {
    #[doc = "1: Update TAMATCHR and TAPR, if used, on the next time-out."]
    TOUPDATE = 1,
    #[doc = "0: Update TAMATCHR and TAPR, if used, on the next cycle."]
    CYCLEUPDATE = 0,
}
impl From<TAMRSU_A> for bool {
    #[inline(always)]
    fn from(variant: TAMRSU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAMRSU`"]
pub type TAMRSU_R = crate::R<bool, TAMRSU_A>;
impl TAMRSU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMRSU_A {
        match self.bits {
            true => TAMRSU_A::TOUPDATE,
            false => TAMRSU_A::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == TAMRSU_A::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TAMRSU_A::CYCLEUPDATE
    }
}
#[doc = "Write proxy for field `TAMRSU`"]
pub struct TAMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMRSU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMRSU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TAMRSU_A::TOUPDATE)
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TAMRSU_A::CYCLEUPDATE)
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
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPWMIE_A {
    #[doc = "1: Interrupt is enabled.  This bit is only valid in PWM mode."]
    EN = 1,
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
}
impl From<TAPWMIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAPWMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAPWMIE`"]
pub type TAPWMIE_R = crate::R<bool, TAPWMIE_A>;
impl TAPWMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPWMIE_A {
        match self.bits {
            true => TAPWMIE_A::EN,
            false => TAPWMIE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAPWMIE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAPWMIE_A::DIS
    }
}
#[doc = "Write proxy for field `TAPWMIE`"]
pub struct TAPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPWMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAPWMIE_A::EN)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAPWMIE_A::DIS)
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
GPT Timer A PWM Interval Load Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAILD_A {
    #[doc = "1: Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    TOUPDATE = 1,
    #[doc = "0: Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    CYCLEUPDATE = 0,
}
impl From<TAILD_A> for bool {
    #[inline(always)]
    fn from(variant: TAILD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAILD`"]
pub type TAILD_R = crate::R<bool, TAILD_A>;
impl TAILD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAILD_A {
        match self.bits {
            true => TAILD_A::TOUPDATE,
            false => TAILD_A::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == TAILD_A::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TAILD_A::CYCLEUPDATE
    }
}
#[doc = "Write proxy for field `TAILD`"]
pub struct TAILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAILD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TAILD_A::TOUPDATE)
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TAILD_A::CYCLEUPDATE)
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
GPT Timer A Snap-Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASNAPS_A {
    #[doc = "1: If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    EN = 1,
    #[doc = "0: Snap-shot mode is disabled."]
    DIS = 0,
}
impl From<TASNAPS_A> for bool {
    #[inline(always)]
    fn from(variant: TASNAPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TASNAPS`"]
pub type TASNAPS_R = crate::R<bool, TASNAPS_A>;
impl TASNAPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASNAPS_A {
        match self.bits {
            true => TASNAPS_A::EN,
            false => TASNAPS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TASNAPS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TASNAPS_A::DIS
    }
}
#[doc = "Write proxy for field `TASNAPS`"]
pub struct TASNAPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TASNAPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASNAPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TASNAPS_A::EN)
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TASNAPS_A::DIS)
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
GPT Timer A Wait-On-Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAWOT_A {
    #[doc = "1: If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    WAIT = 1,
    #[doc = "0: Timer A begins counting as soon as it is enabled."]
    NOWAIT = 0,
}
impl From<TAWOT_A> for bool {
    #[inline(always)]
    fn from(variant: TAWOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAWOT`"]
pub type TAWOT_R = crate::R<bool, TAWOT_A>;
impl TAWOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAWOT_A {
        match self.bits {
            true => TAWOT_A::WAIT,
            false => TAWOT_A::NOWAIT,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == TAWOT_A::WAIT
    }
    #[doc = "Checks if the value of the field is `NOWAIT`"]
    #[inline(always)]
    pub fn is_nowait(&self) -> bool {
        *self == TAWOT_A::NOWAIT
    }
}
#[doc = "Write proxy for field `TAWOT`"]
pub struct TAWOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAWOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAWOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut W {
        self.variant(TAWOT_A::WAIT)
    }
    #[doc = "Timer A begins counting as soon as it is enabled."]
    #[inline(always)]
    pub fn nowait(self) -> &'a mut W {
        self.variant(TAWOT_A::NOWAIT)
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
GPT Timer A Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMIE_A {
    #[doc = "1: An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    EN = 1,
    #[doc = "0: The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS = 0,
}
impl From<TAMIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAMIE`"]
pub type TAMIE_R = crate::R<bool, TAMIE_A>;
impl TAMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMIE_A {
        match self.bits {
            true => TAMIE_A::EN,
            false => TAMIE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAMIE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAMIE_A::DIS
    }
}
#[doc = "Write proxy for field `TAMIE`"]
pub struct TAMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAMIE_A::EN)
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAMIE_A::DIS)
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
GPT Timer A Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACDIR_A {
    #[doc = "1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP = 1,
    #[doc = "0: The timer counts down."]
    DOWN = 0,
}
impl From<TACDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TACDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TACDIR`"]
pub type TACDIR_R = crate::R<bool, TACDIR_A>;
impl TACDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TACDIR_A {
        match self.bits {
            true => TACDIR_A::UP,
            false => TACDIR_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == TACDIR_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == TACDIR_A::DOWN
    }
}
#[doc = "Write proxy for field `TACDIR`"]
pub struct TACDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(TACDIR_A::UP)
    }
    #[doc = "The timer counts down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(TACDIR_A::DOWN)
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
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAAMS_A {
    #[doc = "1: PWM mode is enabled"]
    PWM = 1,
    #[doc = "0: Capture/Compare mode is enabled."]
    CAP_COMP = 0,
}
impl From<TAAMS_A> for bool {
    #[inline(always)]
    fn from(variant: TAAMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAAMS`"]
pub type TAAMS_R = crate::R<bool, TAAMS_A>;
impl TAAMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAAMS_A {
        match self.bits {
            true => TAAMS_A::PWM,
            false => TAAMS_A::CAP_COMP,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == TAAMS_A::PWM
    }
    #[doc = "Checks if the value of the field is `CAP_COMP`"]
    #[inline(always)]
    pub fn is_cap_comp(&self) -> bool {
        *self == TAAMS_A::CAP_COMP
    }
}
#[doc = "Write proxy for field `TAAMS`"]
pub struct TAAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAAMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAAMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM mode is enabled"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(TAAMS_A::PWM)
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline(always)]
    pub fn cap_comp(self) -> &'a mut W {
        self.variant(TAAMS_A::CAP_COMP)
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
GPT Timer A Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACM_A {
    #[doc = "1: Edge-Time mode"]
    EDGTIME = 1,
    #[doc = "0: Edge-Count mode"]
    EDGCNT = 0,
}
impl From<TACM_A> for bool {
    #[inline(always)]
    fn from(variant: TACM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TACM`"]
pub type TACM_R = crate::R<bool, TACM_A>;
impl TACM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TACM_A {
        match self.bits {
            true => TACM_A::EDGTIME,
            false => TACM_A::EDGCNT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGTIME`"]
    #[inline(always)]
    pub fn is_edgtime(&self) -> bool {
        *self == TACM_A::EDGTIME
    }
    #[doc = "Checks if the value of the field is `EDGCNT`"]
    #[inline(always)]
    pub fn is_edgcnt(&self) -> bool {
        *self == TACM_A::EDGCNT
    }
}
#[doc = "Write proxy for field `TACM`"]
pub struct TACM_W<'a> {
    w: &'a mut W,
}
impl<'a> TACM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge-Time mode"]
    #[inline(always)]
    pub fn edgtime(self) -> &'a mut W {
        self.variant(TACM_A::EDGTIME)
    }
    #[doc = "Edge-Count mode"]
    #[inline(always)]
    pub fn edgcnt(self) -> &'a mut W {
        self.variant(TACM_A::EDGCNT)
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
#[doc = "1:0\\]
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMR_A {
    #[doc = "3: Capture mode"]
    CAPTURE = 3,
    #[doc = "2: Periodic Timer mode"]
    PERIODIC = 2,
    #[doc = "1: One-Shot Timer mode"]
    ONE_SHOT = 1,
}
impl From<TAMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAMR`"]
pub type TAMR_R = crate::R<u8, TAMR_A>;
impl TAMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAMR_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(TAMR_A::CAPTURE),
            2 => Val(TAMR_A::PERIODIC),
            1 => Val(TAMR_A::ONE_SHOT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == TAMR_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == TAMR_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == TAMR_A::ONE_SHOT
    }
}
#[doc = "Write proxy for field `TAMR`"]
pub struct TAMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(TAMR_A::CAPTURE)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(TAMR_A::PERIODIC)
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(TAMR_A::ONE_SHOT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&self) -> TCACT_R {
        TCACT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tacintd(&self) -> TACINTD_R {
        TACINTD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn taplo(&self) -> TAPLO_R {
        TAPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&self) -> TAMRSU_R {
        TAMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&self) -> TAPWMIE_R {
        TAPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer A PWM Interval Load Write"]
    #[inline(always)]
    pub fn taild(&self) -> TAILD_R {
        TAILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn tasnaps(&self) -> TASNAPS_R {
        TASNAPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A Wait-On-Trigger"]
    #[inline(always)]
    pub fn tawot(&self) -> TAWOT_R {
        TAWOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn tamie(&self) -> TAMIE_R {
        TAMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Count Direction"]
    #[inline(always)]
    pub fn tacdir(&self) -> TACDIR_R {
        TACDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline(always)]
    pub fn taams(&self) -> TAAMS_R {
        TAAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode"]
    #[inline(always)]
    pub fn tacm(&self) -> TACM_R {
        TACM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&mut self) -> TCACT_W {
        TCACT_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tacintd(&mut self) -> TACINTD_W {
        TACINTD_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn taplo(&mut self) -> TAPLO_W {
        TAPLO_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&mut self) -> TAMRSU_W {
        TAMRSU_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&mut self) -> TAPWMIE_W {
        TAPWMIE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer A PWM Interval Load Write"]
    #[inline(always)]
    pub fn taild(&mut self) -> TAILD_W {
        TAILD_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn tasnaps(&mut self) -> TASNAPS_W {
        TASNAPS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A Wait-On-Trigger"]
    #[inline(always)]
    pub fn tawot(&mut self) -> TAWOT_W {
        TAWOT_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn tamie(&mut self) -> TAMIE_W {
        TAMIE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Count Direction"]
    #[inline(always)]
    pub fn tacdir(&mut self) -> TACDIR_W {
        TACDIR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline(always)]
    pub fn taams(&mut self) -> TAAMS_W {
        TAAMS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode"]
    #[inline(always)]
    pub fn tacm(&mut self) -> TACM_W {
        TACM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    pub fn tamr(&mut self) -> TAMR_W {
        TAMR_W { w: self }
    }
}
