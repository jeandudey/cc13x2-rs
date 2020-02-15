#[doc = "Reader of register MCTRL"]
pub type R = crate::R<u32, super::MCTRL>;
#[doc = "Writer for register MCTRL"]
pub type W = crate::W<u32, super::MCTRL>;
#[doc = "Register MCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_A {
    #[doc = "1: Enable acknowledge"]
    EN = 1,
    #[doc = "0: Disable acknowledge"]
    DIS = 0,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, ACK_A>;
impl ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            true => ACK_A::EN,
            false => ACK_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACK_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACK_A::DIS
    }
}
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable acknowledge"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACK_A::EN)
    }
    #[doc = "Disable acknowledge"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACK_A::DIS)
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
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "1: Enable STOP"]
    EN = 1,
    #[doc = "0: Disable STOP"]
    DIS = 0,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, STOP_A>;
impl STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            true => STOP_A::EN,
            false => STOP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STOP_A::DIS
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable STOP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOP_A::EN)
    }
    #[doc = "Disable STOP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOP_A::DIS)
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
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "1: Enable START"]
    EN = 1,
    #[doc = "0: Disable START"]
    DIS = 0,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            true => START_A::EN,
            false => START_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == START_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == START_A::DIS
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable START"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(START_A::EN)
    }
    #[doc = "Disable START"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(START_A::DIS)
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
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "1: Enable Master"]
    EN = 1,
    #[doc = "0: Disable Master"]
    DIS = 0,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, RUN_A>;
impl RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            true => RUN_A::EN,
            false => RUN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RUN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RUN_A::DIS
    }
}
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Master"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RUN_A::EN)
    }
    #[doc = "Disable Master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RUN_A::DIS)
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
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
}
