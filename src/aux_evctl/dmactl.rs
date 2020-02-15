#[doc = "Reader of register DMACTL"]
pub type R = crate::R<u32, super::DMACTL>;
#[doc = "Writer for register DMACTL"]
pub type W = crate::W<u32, super::DMACTL>;
#[doc = "Register DMACTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "2:2\\]
UDMA0 Request mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQ_MODE_A {
    #[doc = "1: Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    SINGLE = 1,
    #[doc = "0: Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    BURST = 0,
}
impl From<REQ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REQ_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REQ_MODE`"]
pub type REQ_MODE_R = crate::R<bool, REQ_MODE_A>;
impl REQ_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_MODE_A {
        match self.bits {
            true => REQ_MODE_A::SINGLE,
            false => REQ_MODE_A::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == REQ_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == REQ_MODE_A::BURST
    }
}
#[doc = "Write proxy for field `REQ_MODE`"]
pub struct REQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQ_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(REQ_MODE_A::SINGLE)
    }
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(REQ_MODE_A::BURST)
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "1: UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    AUX_ADC_FIFO_ALMOST_FULL = 1,
    #[doc = "0: UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    AUX_ADC_FIFO_NOT_EMPTY = 0,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            true => SEL_A::AUX_ADC_FIFO_ALMOST_FULL,
            false => SEL_A::AUX_ADC_FIFO_NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == SEL_A::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == SEL_A::AUX_ADC_FIFO_NOT_EMPTY
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(SEL_A::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(SEL_A::AUX_ADC_FIFO_NOT_EMPTY)
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
UDMA0 Request mode"]
    #[inline(always)]
    pub fn req_mode(&self) -> REQ_MODE_R {
        REQ_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
UDMA0 Request mode"]
    #[inline(always)]
    pub fn req_mode(&mut self) -> REQ_MODE_W {
        REQ_MODE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
