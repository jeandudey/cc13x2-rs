#[doc = "Reader of register RESETI2C"]
pub type R = crate::R<u32, super::RESETI2C>;
#[doc = "Writer for register RESETI2C"]
pub type W = crate::W<u32, super::RESETI2C>;
#[doc = "Register RESETI2C `reset()`'s with value 0"]
impl crate::ResetValue for super::RESETI2C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE1`"]
pub type SPARE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPARE1`"]
pub struct SPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `I2C`"]
pub type I2C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C`"]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> SPARE1_R {
        SPARE1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&mut self) -> SPARE1_W {
        SPARE1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
}
