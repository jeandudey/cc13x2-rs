#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x001f_0000"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001f_0000
    }
}
#[doc = "Reader of field `TEST`"]
pub type TEST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST`"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `RESERVED21`"]
pub type RESERVED21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED21`"]
pub struct RESERVED21_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 21)) | (((value as u32) & 0x7f) << 21);
        self.w
    }
}
#[doc = "Reader of field `TOTALCHANNELS`"]
pub type TOTALCHANNELS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOTALCHANNELS`"]
pub struct TOTALCHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALCHANNELS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `MASTERENABLE`"]
pub type MASTERENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTERENABLE`"]
pub struct MASTERENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTERENABLE_W<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\]
0x0: Controller does not include the integration test logic 0x1: Controller includes the integration test logic 0x2: Undefined ... 0xF: Undefined"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 21:27 - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Register value returns number of available uDMA channels minus one. For example a read out value of: 0x00: Show that the controller is configured to use 1 uDMA channel 0x01: Shows that the controller is configured to use 2 uDMA channels ... 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)"]
    #[inline(always)]
    pub fn totalchannels(&self) -> TOTALCHANNELS_R {
        TOTALCHANNELS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Current state of the control state machine. State can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source data end pointer 0x3: Reading destination data end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA: Peripheral scatter-gather transition 0xB: Undefined ... 0xF: Undefined."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Shows the enable status of the controller as configured by CFG.MASTERENABLE: 0: Controller is disabled 1: Controller is enabled"]
    #[inline(always)]
    pub fn masterenable(&self) -> MASTERENABLE_R {
        MASTERENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
0x0: Controller does not include the integration test logic 0x1: Controller includes the integration test logic 0x2: Undefined ... 0xF: Undefined"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bits 21:27 - 27:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&mut self) -> RESERVED21_W {
        RESERVED21_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Register value returns number of available uDMA channels minus one. For example a read out value of: 0x00: Show that the controller is configured to use 1 uDMA channel 0x01: Shows that the controller is configured to use 2 uDMA channels ... 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)"]
    #[inline(always)]
    pub fn totalchannels(&mut self) -> TOTALCHANNELS_W {
        TOTALCHANNELS_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Current state of the control state machine. State can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source data end pointer 0x3: Reading destination data end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA: Peripheral scatter-gather transition 0xB: Undefined ... 0xF: Undefined."]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Shows the enable status of the controller as configured by CFG.MASTERENABLE: 0: Controller is disabled 1: Controller is enabled"]
    #[inline(always)]
    pub fn masterenable(&mut self) -> MASTERENABLE_W {
        MASTERENABLE_W { w: self }
    }
}
