#[doc = "Reader of register MSTAT"]
pub type R = crate::R<u32, super::MSTAT>;
#[doc = "Writer for register MSTAT"]
pub type W = crate::W<u32, super::MSTAT>;
#[doc = "Register MSTAT `reset()`'s with value 0x20"]
impl crate::ResetValue for super::MSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
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
#[doc = "Reader of field `BUSBSY`"]
pub type BUSBSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSBSY`"]
pub struct BUSBSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSBSY_W<'a> {
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
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
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
#[doc = "Reader of field `ARBLST`"]
pub type ARBLST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLST`"]
pub struct ARBLST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLST_W<'a> {
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
#[doc = "Reader of field `DATACK_N`"]
pub type DATACK_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATACK_N`"]
pub struct DATACK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACK_N_W<'a> {
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
#[doc = "Reader of field `ADRACK_N`"]
pub type ADRACK_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRACK_N`"]
pub struct ADRACK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRACK_N_W<'a> {
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
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&self) -> BUSBSY_R {
        BUSBSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    pub fn datack_n(&self) -> DATACK_N_R {
        DATACK_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    pub fn adrack_n(&self) -> ADRACK_N_R {
        ADRACK_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
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
Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    pub fn busbsy(&mut self) -> BUSBSY_W {
        BUSBSY_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub fn arblst(&mut self) -> ARBLST_W {
        ARBLST_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    pub fn datack_n(&mut self) -> DATACK_N_W {
        DATACK_N_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    pub fn adrack_n(&mut self) -> ADRACK_N_W {
        ADRACK_N_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
