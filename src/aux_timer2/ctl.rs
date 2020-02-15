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
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `CH3_RESET`"]
pub type CH3_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_RESET`"]
pub struct CH3_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH2_RESET`"]
pub type CH2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_RESET`"]
pub struct CH2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH1_RESET`"]
pub type CH1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_RESET`"]
pub struct CH1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH0_RESET`"]
pub type CH0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_RESET`"]
pub struct CH0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
Select counter target value. You must select TARGET to use shadow target functionality.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARGET_EN_A {
    #[doc = "1: TARGET.VALUE"]
    TARGET = 1,
    #[doc = "0: 65535"]
    CNTR_MAX = 0,
}
impl From<TARGET_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TARGET_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TARGET_EN`"]
pub type TARGET_EN_R = crate::R<bool, TARGET_EN_A>;
impl TARGET_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARGET_EN_A {
        match self.bits {
            true => TARGET_EN_A::TARGET,
            false => TARGET_EN_A::CNTR_MAX,
        }
    }
    #[doc = "Checks if the value of the field is `TARGET`"]
    #[inline(always)]
    pub fn is_target(&self) -> bool {
        *self == TARGET_EN_A::TARGET
    }
    #[doc = "Checks if the value of the field is `CNTR_MAX`"]
    #[inline(always)]
    pub fn is_cntr_max(&self) -> bool {
        *self == TARGET_EN_A::CNTR_MAX
    }
}
#[doc = "Write proxy for field `TARGET_EN`"]
pub struct TARGET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARGET_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TARGET.VALUE"]
    #[inline(always)]
    pub fn target(self) -> &'a mut W {
        self.variant(TARGET_EN_A::TARGET)
    }
    #[doc = "65535"]
    #[inline(always)]
    pub fn cntr_max(self) -> &'a mut W {
        self.variant(TARGET_EN_A::CNTR_MAX)
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
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly.\n\nPeriod =  (target value * 2) * timer clock period"]
    UPDWN_PER = 3,
    #[doc = "2: Count up periodically. The timer increments from 0 to target value, repeatedly.\n\nPeriod = (target value + 1) * timer clock period"]
    UP_PER = 2,
    #[doc = "1: Count up once. The timer increments from 0 to target value,  then stops and sets MODE to DIS."]
    UP_ONCE = 1,
    #[doc = "0: Disable timer. Updates to counter, channels, and events stop."]
    DIS = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            3 => MODE_A::UPDWN_PER,
            2 => MODE_A::UP_PER,
            1 => MODE_A::UP_ONCE,
            0 => MODE_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UPDWN_PER`"]
    #[inline(always)]
    pub fn is_updwn_per(&self) -> bool {
        *self == MODE_A::UPDWN_PER
    }
    #[doc = "Checks if the value of the field is `UP_PER`"]
    #[inline(always)]
    pub fn is_up_per(&self) -> bool {
        *self == MODE_A::UP_PER
    }
    #[doc = "Checks if the value of the field is `UP_ONCE`"]
    #[inline(always)]
    pub fn is_up_once(&self) -> bool {
        *self == MODE_A::UP_ONCE
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MODE_A::DIS
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn updwn_per(self) -> &'a mut W {
        self.variant(MODE_A::UPDWN_PER)
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline(always)]
    pub fn up_per(self) -> &'a mut W {
        self.variant(MODE_A::UP_PER)
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn up_once(self) -> &'a mut W {
        self.variant(MODE_A::UP_ONCE)
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MODE_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch3_reset(&self) -> CH3_RESET_R {
        CH3_RESET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch2_reset(&self) -> CH2_RESET_R {
        CH2_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch1_reset(&self) -> CH1_RESET_R {
        CH1_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch0_reset(&self) -> CH0_RESET_R {
        CH0_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline(always)]
    pub fn target_en(&self) -> TARGET_EN_R {
        TARGET_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Channel 3 reset. 0: No effect. 1: Reset CH3CC, CH3PCC, CH3EVCFG, and CH3CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch3_reset(&mut self) -> CH3_RESET_W {
        CH3_RESET_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Channel 2 reset. 0: No effect. 1: Reset CH2CC, CH2PCC, CH2EVCFG, and CH2CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch2_reset(&mut self) -> CH2_RESET_W {
        CH2_RESET_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 reset. 0: No effect. 1: Reset CH1CC, CH1PCC, CH1EVCFG, and CH1CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch1_reset(&mut self) -> CH1_RESET_W {
        CH1_RESET_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 0 reset. 0: No effect. 1: Reset CH0CC, CH0PCC, CH0EVCFG, and CH0CCFG. Read returns 0."]
    #[inline(always)]
    pub fn ch0_reset(&mut self) -> CH0_RESET_W {
        CH0_RESET_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select counter target value. You must select TARGET to use shadow target functionality."]
    #[inline(always)]
    pub fn target_en(&mut self) -> TARGET_EN_W {
        TARGET_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Timer mode control. The timer restarts from 0 when you set MODE to UP_ONCE, UP_PER, or UPDWN_PER. When you write MODE all internally queued updates to \\[CHnCC.*\\]
and TARGET clear."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
