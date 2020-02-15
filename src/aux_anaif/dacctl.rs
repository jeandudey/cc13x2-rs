#[doc = "Reader of register DACCTL"]
pub type R = crate::R<u32, super::DACCTL>;
#[doc = "Writer for register DACCTL"]
pub type W = crate::W<u32, super::DACCTL>;
#[doc = "Register DACCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DACCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `DAC_EN`"]
pub type DAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_EN`"]
pub struct DAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_EN_W<'a> {
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
#[doc = "Reader of field `DAC_BUFFER_EN`"]
pub type DAC_BUFFER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_BUFFER_EN`"]
pub struct DAC_BUFFER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_BUFFER_EN_W<'a> {
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
#[doc = "Reader of field `DAC_PRECHARGE_EN`"]
pub type DAC_PRECHARGE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_PRECHARGE_EN`"]
pub struct DAC_PRECHARGE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_PRECHARGE_EN_W<'a> {
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
#[doc = "2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAC_VOUT_SEL_A {
    #[doc = "4: Connect to COMPA_IN analog node.\n\nRequired setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    COMPA_IN = 4,
    #[doc = "2: Connect to COMPA_REF analog node.\n\nIt is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    COMPA_REF = 2,
    #[doc = "1: Connect to COMPB_REF analog node.\n\nRequired setting to use Comparator B."]
    COMPB_REF = 1,
    #[doc = "0: Connect to nothing\n\nIt is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    NC = 0,
}
impl From<DAC_VOUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_VOUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DAC_VOUT_SEL`"]
pub type DAC_VOUT_SEL_R = crate::R<u8, DAC_VOUT_SEL_A>;
impl DAC_VOUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DAC_VOUT_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(DAC_VOUT_SEL_A::COMPA_IN),
            2 => Val(DAC_VOUT_SEL_A::COMPA_REF),
            1 => Val(DAC_VOUT_SEL_A::COMPB_REF),
            0 => Val(DAC_VOUT_SEL_A::NC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPA_IN`"]
    #[inline(always)]
    pub fn is_compa_in(&self) -> bool {
        *self == DAC_VOUT_SEL_A::COMPA_IN
    }
    #[doc = "Checks if the value of the field is `COMPA_REF`"]
    #[inline(always)]
    pub fn is_compa_ref(&self) -> bool {
        *self == DAC_VOUT_SEL_A::COMPA_REF
    }
    #[doc = "Checks if the value of the field is `COMPB_REF`"]
    #[inline(always)]
    pub fn is_compb_ref(&self) -> bool {
        *self == DAC_VOUT_SEL_A::COMPB_REF
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DAC_VOUT_SEL_A::NC
    }
}
#[doc = "Write proxy for field `DAC_VOUT_SEL`"]
pub struct DAC_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_VOUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC_VOUT_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    #[inline(always)]
    pub fn compa_in(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::COMPA_IN)
    }
    #[doc = "Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    #[inline(always)]
    pub fn compa_ref(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::COMPA_REF)
    }
    #[doc = "Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    #[inline(always)]
    pub fn compb_ref(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::COMPB_REF)
    }
    #[doc = "Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(DAC_VOUT_SEL_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
    #[inline(always)]
    pub fn dac_en(&self) -> DAC_EN_R {
        DAC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline(always)]
    pub fn dac_buffer_en(&self) -> DAC_BUFFER_EN_R {
        DAC_BUFFER_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline(always)]
    pub fn dac_precharge_en(&self) -> DAC_PRECHARGE_EN_R {
        DAC_PRECHARGE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline(always)]
    pub fn dac_vout_sel(&self) -> DAC_VOUT_SEL_R {
        DAC_VOUT_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The Sensor Controller must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
    #[inline(always)]
    pub fn dac_en(&mut self) -> DAC_EN_W {
        DAC_EN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline(always)]
    pub fn dac_buffer_en(&mut self) -> DAC_BUFFER_EN_W {
        DAC_BUFFER_EN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline(always)]
    pub fn dac_precharge_en(&mut self) -> DAC_PRECHARGE_EN_W {
        DAC_PRECHARGE_EN_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline(always)]
    pub fn dac_vout_sel(&mut self) -> DAC_VOUT_SEL_W {
        DAC_VOUT_SEL_W { w: self }
    }
}
