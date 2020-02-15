#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Reader of field `ATBID`"]
pub type ATBID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATBID`"]
pub struct ATBID_W<'a> {
    w: &'a mut W,
}
impl<'a> ATBID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED10`"]
pub type RESERVED10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED10`"]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "9:8\\]
Timestamp prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSPRESCALE_A {
    #[doc = "3: Divide by 64"]
    DIV64 = 3,
    #[doc = "2: Divide by 16"]
    DIV16 = 2,
    #[doc = "1: Divide by 4"]
    DIV4 = 1,
    #[doc = "0: No prescaling"]
    NOPRESCALING = 0,
}
impl From<TSPRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: TSPRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSPRESCALE`"]
pub type TSPRESCALE_R = crate::R<u8, TSPRESCALE_A>;
impl TSPRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSPRESCALE_A {
        match self.bits {
            3 => TSPRESCALE_A::DIV64,
            2 => TSPRESCALE_A::DIV16,
            1 => TSPRESCALE_A::DIV4,
            0 => TSPRESCALE_A::NOPRESCALING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == TSPRESCALE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == TSPRESCALE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == TSPRESCALE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `NOPRESCALING`"]
    #[inline(always)]
    pub fn is_noprescaling(&self) -> bool {
        *self == TSPRESCALE_A::NOPRESCALING
    }
}
#[doc = "Write proxy for field `TSPRESCALE`"]
pub struct TSPRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSPRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::DIV64)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::DIV16)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::DIV4)
    }
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn noprescaling(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::NOPRESCALING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `SWOENA`"]
pub type SWOENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWOENA`"]
pub struct SWOENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SWOENA_W<'a> {
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
#[doc = "Reader of field `DWTENA`"]
pub type DWTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DWTENA`"]
pub struct DWTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTENA_W<'a> {
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
#[doc = "Reader of field `SYNCENA`"]
pub type SYNCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCENA`"]
pub struct SYNCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCENA_W<'a> {
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
#[doc = "Reader of field `TSENA`"]
pub type TSENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENA`"]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
#[doc = "Reader of field `ITMENA`"]
pub type ITMENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITMENA`"]
pub struct ITMENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMENA_W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Set when ITM events present and being drained."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    pub fn atbid(&self) -> ATBID_R {
        ATBID_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Timestamp prescaler"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TSPRESCALE_R {
        TSPRESCALE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    pub fn swoena(&self) -> SWOENA_R {
        SWOENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline(always)]
    pub fn dwtena(&self) -> DWTENA_R {
        DWTENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    pub fn syncena(&self) -> SYNCENA_R {
        SYNCENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    pub fn itmena(&self) -> ITMENA_R {
        ITMENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Set when ITM events present and being drained."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 16:22 - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    pub fn atbid(&mut self) -> ATBID_W {
        ATBID_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Timestamp prescaler"]
    #[inline(always)]
    pub fn tsprescale(&mut self) -> TSPRESCALE_W {
        TSPRESCALE_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    pub fn swoena(&mut self) -> SWOENA_W {
        SWOENA_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline(always)]
    pub fn dwtena(&mut self) -> DWTENA_W {
        DWTENA_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    pub fn syncena(&mut self) -> SYNCENA_W {
        SYNCENA_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    pub fn itmena(&mut self) -> ITMENA_W {
        ITMENA_W { w: self }
    }
}
