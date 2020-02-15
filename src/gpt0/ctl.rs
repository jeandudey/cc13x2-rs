#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED15`"]
pub type RESERVED15_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED15`"]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 15)) | (((value as u32) & 0x0001_ffff) << 15);
        self.w
    }
}
#[doc = "14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBPWML_A {
    #[doc = "1: Inverted"]
    INVERTED = 1,
    #[doc = "0: Not inverted"]
    NORMAL = 0,
}
impl From<TBPWML_A> for bool {
    #[inline(always)]
    fn from(variant: TBPWML_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBPWML`"]
pub type TBPWML_R = crate::R<bool, TBPWML_A>;
impl TBPWML_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBPWML_A {
        match self.bits {
            true => TBPWML_A::INVERTED,
            false => TBPWML_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TBPWML_A::INVERTED
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TBPWML_A::NORMAL
    }
}
#[doc = "Write proxy for field `TBPWML`"]
pub struct TBPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWML_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBPWML_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TBPWML_A::INVERTED)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TBPWML_A::NORMAL)
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
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBEVENT_A {
    #[doc = "3: Both edges"]
    BOTH = 3,
    #[doc = "1: Negative edge"]
    NEG = 1,
    #[doc = "0: Positive edge"]
    POS = 0,
}
impl From<TBEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TBEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBEVENT`"]
pub type TBEVENT_R = crate::R<u8, TBEVENT_A>;
impl TBEVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TBEVENT_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(TBEVENT_A::BOTH),
            1 => Val(TBEVENT_A::NEG),
            0 => Val(TBEVENT_A::POS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == TBEVENT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == TBEVENT_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == TBEVENT_A::POS
    }
}
#[doc = "Write proxy for field `TBEVENT`"]
pub struct TBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBEVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(TBEVENT_A::BOTH)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(TBEVENT_A::NEG)
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(TBEVENT_A::POS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "9:9\\]
GPT Timer B Stall Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBSTALL_A {
    #[doc = "1: Timer B freezes counting while the processor is halted by the debugger."]
    EN = 1,
    #[doc = "0: Timer B continues counting while the processor is halted by the debugger."]
    DIS = 0,
}
impl From<TBSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TBSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBSTALL`"]
pub type TBSTALL_R = crate::R<bool, TBSTALL_A>;
impl TBSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSTALL_A {
        match self.bits {
            true => TBSTALL_A::EN,
            false => TBSTALL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBSTALL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBSTALL_A::DIS
    }
}
#[doc = "Write proxy for field `TBSTALL`"]
pub struct TBSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBSTALL_A::EN)
    }
    #[doc = "Timer B continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBSTALL_A::DIS)
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
GPT Timer B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEN_A {
    #[doc = "1: Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    EN = 1,
    #[doc = "0: Timer B is disabled."]
    DIS = 0,
}
impl From<TBEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBEN`"]
pub type TBEN_R = crate::R<bool, TBEN_A>;
impl TBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEN_A {
        match self.bits {
            true => TBEN_A::EN,
            false => TBEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBEN_A::DIS
    }
}
#[doc = "Write proxy for field `TBEN`"]
pub struct TBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBEN_A::EN)
    }
    #[doc = "Timer B is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBEN_A::DIS)
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
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
GPT Timer A PWM Output Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPWML_A {
    #[doc = "1: Inverted"]
    INVERTED = 1,
    #[doc = "0: Not inverted"]
    NORMAL = 0,
}
impl From<TAPWML_A> for bool {
    #[inline(always)]
    fn from(variant: TAPWML_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAPWML`"]
pub type TAPWML_R = crate::R<bool, TAPWML_A>;
impl TAPWML_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPWML_A {
        match self.bits {
            true => TAPWML_A::INVERTED,
            false => TAPWML_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TAPWML_A::INVERTED
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TAPWML_A::NORMAL
    }
}
#[doc = "Write proxy for field `TAPWML`"]
pub struct TAPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWML_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPWML_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TAPWML_A::INVERTED)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TAPWML_A::NORMAL)
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
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAEVENT_A {
    #[doc = "3: Both edges"]
    BOTH = 3,
    #[doc = "1: Negative edge"]
    NEG = 1,
    #[doc = "0: Positive edge"]
    POS = 0,
}
impl From<TAEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAEVENT`"]
pub type TAEVENT_R = crate::R<u8, TAEVENT_A>;
impl TAEVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAEVENT_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(TAEVENT_A::BOTH),
            1 => Val(TAEVENT_A::NEG),
            0 => Val(TAEVENT_A::POS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == TAEVENT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == TAEVENT_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == TAEVENT_A::POS
    }
}
#[doc = "Write proxy for field `TAEVENT`"]
pub struct TAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAEVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(TAEVENT_A::BOTH)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(TAEVENT_A::NEG)
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(TAEVENT_A::POS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "1:1\\]
GPT Timer A Stall Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASTALL_A {
    #[doc = "1: Timer A freezes counting while the processor is halted by the debugger."]
    EN = 1,
    #[doc = "0: Timer A continues counting while the processor is halted by the debugger."]
    DIS = 0,
}
impl From<TASTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TASTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TASTALL`"]
pub type TASTALL_R = crate::R<bool, TASTALL_A>;
impl TASTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASTALL_A {
        match self.bits {
            true => TASTALL_A::EN,
            false => TASTALL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TASTALL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TASTALL_A::DIS
    }
}
#[doc = "Write proxy for field `TASTALL`"]
pub struct TASTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TASTALL_A::EN)
    }
    #[doc = "Timer A continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TASTALL_A::DIS)
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
GPT Timer A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAEN_A {
    #[doc = "1: Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    EN = 1,
    #[doc = "0: Timer A is disabled."]
    DIS = 0,
}
impl From<TAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAEN`"]
pub type TAEN_R = crate::R<bool, TAEN_A>;
impl TAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAEN_A {
        match self.bits {
            true => TAEN_A::EN,
            false => TAEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAEN_A::DIS
    }
}
#[doc = "Write proxy for field `TAEN`"]
pub struct TAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAEN_A::EN)
    }
    #[doc = "Timer A is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAEN_A::DIS)
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
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 14 - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TBPWML_R {
        TBPWML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn tbevent(&self) -> TBEVENT_R {
        TBEVENT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Stall Enable"]
    #[inline(always)]
    pub fn tbstall(&self) -> TBSTALL_R {
        TBSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Enable"]
    #[inline(always)]
    pub fn tben(&self) -> TBEN_R {
        TBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A PWM Output Level"]
    #[inline(always)]
    pub fn tapwml(&self) -> TAPWML_R {
        TAPWML_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn taevent(&self) -> TAEVENT_R {
        TAEVENT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Stall Enable"]
    #[inline(always)]
    pub fn tastall(&self) -> TASTALL_R {
        TASTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Enable"]
    #[inline(always)]
    pub fn taen(&self) -> TAEN_R {
        TAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&mut self) -> TBPWML_W {
        TBPWML_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn tbevent(&mut self) -> TBEVENT_W {
        TBEVENT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Stall Enable"]
    #[inline(always)]
    pub fn tbstall(&mut self) -> TBSTALL_W {
        TBSTALL_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Enable"]
    #[inline(always)]
    pub fn tben(&mut self) -> TBEN_W {
        TBEN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A PWM Output Level"]
    #[inline(always)]
    pub fn tapwml(&mut self) -> TAPWML_W {
        TAPWML_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn taevent(&mut self) -> TAEVENT_W {
        TAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Stall Enable"]
    #[inline(always)]
    pub fn tastall(&mut self) -> TASTALL_W {
        TASTALL_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Enable"]
    #[inline(always)]
    pub fn taen(&mut self) -> TAEN_W {
        TAEN_W { w: self }
    }
}
