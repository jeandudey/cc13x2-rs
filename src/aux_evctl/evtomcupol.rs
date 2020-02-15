#[doc = "Reader of register EVTOMCUPOL"]
pub type R = crate::R<u32, super::EVTOMCUPOL>;
#[doc = "Writer for register EVTOMCUPOL"]
pub type W = crate::W<u32, super::EVTOMCUPOL>;
#[doc = "Register EVTOMCUPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTOMCUPOL {
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
#[doc = "15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_PULSE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_PULSE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_PULSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER2_PULSE`"]
pub type AUX_TIMER2_PULSE_R = crate::R<bool, AUX_TIMER2_PULSE_A>;
impl AUX_TIMER2_PULSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_PULSE_A {
        match self.bits {
            true => AUX_TIMER2_PULSE_A::LOW,
            false => AUX_TIMER2_PULSE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_PULSE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_PULSE_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER2_PULSE`"]
pub struct AUX_TIMER2_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_PULSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER2_PULSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_PULSE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_PULSE_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV3_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV3_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER2_EV3`"]
pub type AUX_TIMER2_EV3_R = crate::R<bool, AUX_TIMER2_EV3_A>;
impl AUX_TIMER2_EV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV3_A {
        match self.bits {
            true => AUX_TIMER2_EV3_A::LOW,
            false => AUX_TIMER2_EV3_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV3_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER2_EV3`"]
pub struct AUX_TIMER2_EV3_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER2_EV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV3_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV3_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV2_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV2_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER2_EV2`"]
pub type AUX_TIMER2_EV2_R = crate::R<bool, AUX_TIMER2_EV2_A>;
impl AUX_TIMER2_EV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV2_A {
        match self.bits {
            true => AUX_TIMER2_EV2_A::LOW,
            false => AUX_TIMER2_EV2_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV2_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER2_EV2`"]
pub struct AUX_TIMER2_EV2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER2_EV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV2_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV2_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV1_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV1_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER2_EV1`"]
pub type AUX_TIMER2_EV1_R = crate::R<bool, AUX_TIMER2_EV1_A>;
impl AUX_TIMER2_EV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV1_A {
        match self.bits {
            true => AUX_TIMER2_EV1_A::LOW,
            false => AUX_TIMER2_EV1_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV1_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER2_EV1`"]
pub struct AUX_TIMER2_EV1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER2_EV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV1_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV1_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER2_EV0_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER2_EV0_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER2_EV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER2_EV0`"]
pub type AUX_TIMER2_EV0_R = crate::R<bool, AUX_TIMER2_EV0_A>;
impl AUX_TIMER2_EV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER2_EV0_A {
        match self.bits {
            true => AUX_TIMER2_EV0_A::LOW,
            false => AUX_TIMER2_EV0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER2_EV0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER2_EV0_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER2_EV0`"]
pub struct AUX_TIMER2_EV0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_EV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER2_EV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV0_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER2_EV0_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_IRQ_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_ADC_IRQ`"]
pub type AUX_ADC_IRQ_R = crate::R<bool, AUX_ADC_IRQ_A>;
impl AUX_ADC_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_IRQ_A {
        match self.bits {
            true => AUX_ADC_IRQ_A::LOW,
            false => AUX_ADC_IRQ_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_IRQ_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_IRQ_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_ADC_IRQ`"]
pub struct AUX_ADC_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_ADC_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_IRQ_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_IRQ_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCU_OBSMUX0_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<MCU_OBSMUX0_A> for bool {
    #[inline(always)]
    fn from(variant: MCU_OBSMUX0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCU_OBSMUX0`"]
pub type MCU_OBSMUX0_R = crate::R<bool, MCU_OBSMUX0_A>;
impl MCU_OBSMUX0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCU_OBSMUX0_A {
        match self.bits {
            true => MCU_OBSMUX0_A::LOW,
            false => MCU_OBSMUX0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == MCU_OBSMUX0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == MCU_OBSMUX0_A::HIGH
    }
}
#[doc = "Write proxy for field `MCU_OBSMUX0`"]
pub struct MCU_OBSMUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OBSMUX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCU_OBSMUX0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(MCU_OBSMUX0_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(MCU_OBSMUX0_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_FIFO_ALMOST_FULL_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_FIFO_ALMOST_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_FIFO_ALMOST_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_ADC_FIFO_ALMOST_FULL`"]
pub type AUX_ADC_FIFO_ALMOST_FULL_R = crate::R<bool, AUX_ADC_FIFO_ALMOST_FULL_A>;
impl AUX_ADC_FIFO_ALMOST_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_FIFO_ALMOST_FULL_A {
        match self.bits {
            true => AUX_ADC_FIFO_ALMOST_FULL_A::LOW,
            false => AUX_ADC_FIFO_ALMOST_FULL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_FIFO_ALMOST_FULL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_FIFO_ALMOST_FULL_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_ADC_FIFO_ALMOST_FULL`"]
pub struct AUX_ADC_FIFO_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_FIFO_ALMOST_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_ADC_FIFO_ALMOST_FULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_FIFO_ALMOST_FULL_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_FIFO_ALMOST_FULL_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_ADC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_ADC_DONE`"]
pub type AUX_ADC_DONE_R = crate::R<bool, AUX_ADC_DONE_A>;
impl AUX_ADC_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADC_DONE_A {
        match self.bits {
            true => AUX_ADC_DONE_A::LOW,
            false => AUX_ADC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_ADC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_ADC_DONE_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_ADC_DONE`"]
pub struct AUX_ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADC_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_ADC_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_ADC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_ADC_DONE_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_SMPH_AUTOTAKE_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_SMPH_AUTOTAKE_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_SMPH_AUTOTAKE_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_SMPH_AUTOTAKE_DONE`"]
pub type AUX_SMPH_AUTOTAKE_DONE_R = crate::R<bool, AUX_SMPH_AUTOTAKE_DONE_A>;
impl AUX_SMPH_AUTOTAKE_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_SMPH_AUTOTAKE_DONE_A {
        match self.bits {
            true => AUX_SMPH_AUTOTAKE_DONE_A::LOW,
            false => AUX_SMPH_AUTOTAKE_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_SMPH_AUTOTAKE_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_SMPH_AUTOTAKE_DONE_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_SMPH_AUTOTAKE_DONE`"]
pub struct AUX_SMPH_AUTOTAKE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SMPH_AUTOTAKE_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_SMPH_AUTOTAKE_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_SMPH_AUTOTAKE_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_SMPH_AUTOTAKE_DONE_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER1_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER1_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER1_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER1_EV`"]
pub type AUX_TIMER1_EV_R = crate::R<bool, AUX_TIMER1_EV_A>;
impl AUX_TIMER1_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER1_EV_A {
        match self.bits {
            true => AUX_TIMER1_EV_A::LOW,
            false => AUX_TIMER1_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER1_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER1_EV_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER1_EV`"]
pub struct AUX_TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER1_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER1_EV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER1_EV_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TIMER0_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TIMER0_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TIMER0_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TIMER0_EV`"]
pub type AUX_TIMER0_EV_R = crate::R<bool, AUX_TIMER0_EV_A>;
impl AUX_TIMER0_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TIMER0_EV_A {
        match self.bits {
            true => AUX_TIMER0_EV_A::LOW,
            false => AUX_TIMER0_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TIMER0_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TIMER0_EV_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TIMER0_EV`"]
pub struct AUX_TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER0_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TIMER0_EV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TIMER0_EV_A::HIGH)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_TDC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_TDC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_TDC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_TDC_DONE`"]
pub type AUX_TDC_DONE_R = crate::R<bool, AUX_TDC_DONE_A>;
impl AUX_TDC_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_TDC_DONE_A {
        match self.bits {
            true => AUX_TDC_DONE_A::LOW,
            false => AUX_TDC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_TDC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_TDC_DONE_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_TDC_DONE`"]
pub struct AUX_TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TDC_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_TDC_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_TDC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_TDC_DONE_A::HIGH)
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
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPB_A {
    #[doc = "1: Falling edge"]
    FALL = 1,
    #[doc = "0: Rising edge"]
    RISE = 0,
}
impl From<AUX_COMPB_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, AUX_COMPB_A>;
impl AUX_COMPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPB_A {
        match self.bits {
            true => AUX_COMPB_A::FALL,
            false => AUX_COMPB_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPB_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPB_A::RISE
    }
}
#[doc = "Write proxy for field `AUX_COMPB`"]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::RISE)
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
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPA_A {
    #[doc = "1: Falling edge"]
    FALL = 1,
    #[doc = "0: Rising edge"]
    RISE = 0,
}
impl From<AUX_COMPA_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, AUX_COMPA_A>;
impl AUX_COMPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPA_A {
        match self.bits {
            true => AUX_COMPA_A::FALL,
            false => AUX_COMPA_A::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == AUX_COMPA_A::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == AUX_COMPA_A::RISE
    }
}
#[doc = "Write proxy for field `AUX_COMPA`"]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::FALL)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::RISE)
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
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_WU_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_WU_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_WU_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUX_WU_EV`"]
pub type AUX_WU_EV_R = crate::R<bool, AUX_WU_EV_A>;
impl AUX_WU_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_WU_EV_A {
        match self.bits {
            true => AUX_WU_EV_A::LOW,
            false => AUX_WU_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == AUX_WU_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == AUX_WU_EV_A::HIGH
    }
}
#[doc = "Write proxy for field `AUX_WU_EV`"]
pub struct AUX_WU_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_WU_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_WU_EV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_WU_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_WU_EV_A::HIGH)
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
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSE_R {
        AUX_TIMER2_PULSE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3_R {
        AUX_TIMER2_EV3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2_R {
        AUX_TIMER2_EV2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1_R {
        AUX_TIMER2_EV1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0_R {
        AUX_TIMER2_EV0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQ_R {
        AUX_ADC_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0_R {
        MCU_OBSMUX0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULL_R {
        AUX_ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONE_R {
        AUX_SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
    #[inline(always)]
    pub fn aux_wu_ev(&self) -> AUX_WU_EV_R {
        AUX_WU_EV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_PULSE."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&mut self) -> AUX_TIMER2_PULSE_W {
        AUX_TIMER2_PULSE_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&mut self) -> AUX_TIMER2_EV3_W {
        AUX_TIMER2_EV3_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV2."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&mut self) -> AUX_TIMER2_EV2_W {
        AUX_TIMER2_EV2_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV1."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&mut self) -> AUX_TIMER2_EV1_W {
        AUX_TIMER2_EV1_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER2_EV0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&mut self) -> AUX_TIMER2_EV0_W {
        AUX_TIMER2_EV0_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_IRQ."]
    #[inline(always)]
    pub fn aux_adc_irq(&mut self) -> AUX_ADC_IRQ_W {
        AUX_ADC_IRQ_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.MCU_OBSMUX0."]
    #[inline(always)]
    pub fn mcu_obsmux0(&mut self) -> MCU_OBSMUX0_W {
        MCU_OBSMUX0_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AUX_ADC_FIFO_ALMOST_FULL_W {
        AUX_ADC_FIFO_ALMOST_FULL_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_ADC_DONE."]
    #[inline(always)]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W {
        AUX_ADC_DONE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&mut self) -> AUX_SMPH_AUTOTAKE_DONE_W {
        AUX_SMPH_AUTOTAKE_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER1_EV."]
    #[inline(always)]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W {
        AUX_TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TIMER0_EV."]
    #[inline(always)]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W {
        AUX_TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_TDC_DONE."]
    #[inline(always)]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W {
        AUX_TDC_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source edge that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_WU_EV."]
    #[inline(always)]
    pub fn aux_wu_ev(&mut self) -> AUX_WU_EV_W {
        AUX_WU_EV_W { w: self }
    }
}
