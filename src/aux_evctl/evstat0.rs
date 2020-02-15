#[doc = "Reader of register EVSTAT0"]
pub type R = crate::R<u32, super::EVSTAT0>;
#[doc = "Writer for register EVSTAT0"]
pub type W = crate::W<u32, super::EVSTAT0>;
#[doc = "Register EVSTAT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSTAT0 {
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
#[doc = "Reader of field `AUXIO15`"]
pub type AUXIO15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO15`"]
pub struct AUXIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO15_W<'a> {
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
#[doc = "Reader of field `AUXIO14`"]
pub type AUXIO14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO14`"]
pub struct AUXIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO14_W<'a> {
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
#[doc = "Reader of field `AUXIO13`"]
pub type AUXIO13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO13`"]
pub struct AUXIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO13_W<'a> {
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
#[doc = "Reader of field `AUXIO12`"]
pub type AUXIO12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO12`"]
pub struct AUXIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO12_W<'a> {
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
#[doc = "Reader of field `AUXIO11`"]
pub type AUXIO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO11`"]
pub struct AUXIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO11_W<'a> {
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
#[doc = "Reader of field `AUXIO10`"]
pub type AUXIO10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO10`"]
pub struct AUXIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO10_W<'a> {
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
#[doc = "Reader of field `AUXIO9`"]
pub type AUXIO9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO9`"]
pub struct AUXIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO9_W<'a> {
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
#[doc = "Reader of field `AUXIO8`"]
pub type AUXIO8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO8`"]
pub struct AUXIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO8_W<'a> {
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
#[doc = "Reader of field `AUXIO7`"]
pub type AUXIO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO7`"]
pub struct AUXIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO7_W<'a> {
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
#[doc = "Reader of field `AUXIO6`"]
pub type AUXIO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO6`"]
pub struct AUXIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO6_W<'a> {
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
#[doc = "Reader of field `AUXIO5`"]
pub type AUXIO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO5`"]
pub struct AUXIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO5_W<'a> {
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
#[doc = "Reader of field `AUXIO4`"]
pub type AUXIO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO4`"]
pub struct AUXIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO4_W<'a> {
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
#[doc = "Reader of field `AUXIO3`"]
pub type AUXIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO3`"]
pub struct AUXIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO3_W<'a> {
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
#[doc = "Reader of field `AUXIO2`"]
pub type AUXIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO2`"]
pub struct AUXIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO2_W<'a> {
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
#[doc = "Reader of field `AUXIO1`"]
pub type AUXIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO1`"]
pub struct AUXIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO1_W<'a> {
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
#[doc = "Reader of field `AUXIO0`"]
pub type AUXIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO0`"]
pub struct AUXIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO0_W<'a> {
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
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&self) -> AUXIO15_R {
        AUXIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&self) -> AUXIO14_R {
        AUXIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&self) -> AUXIO13_R {
        AUXIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&self) -> AUXIO12_R {
        AUXIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&self) -> AUXIO11_R {
        AUXIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&self) -> AUXIO10_R {
        AUXIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&self) -> AUXIO9_R {
        AUXIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&self) -> AUXIO8_R {
        AUXIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&self) -> AUXIO7_R {
        AUXIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&self) -> AUXIO6_R {
        AUXIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&self) -> AUXIO5_R {
        AUXIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&self) -> AUXIO4_R {
        AUXIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&self) -> AUXIO3_R {
        AUXIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&self) -> AUXIO2_R {
        AUXIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&self) -> AUXIO1_R {
        AUXIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&self) -> AUXIO0_R {
        AUXIO0_R::new((self.bits & 0x01) != 0)
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
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&mut self) -> AUXIO15_W {
        AUXIO15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&mut self) -> AUXIO14_W {
        AUXIO14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&mut self) -> AUXIO13_W {
        AUXIO13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&mut self) -> AUXIO12_W {
        AUXIO12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&mut self) -> AUXIO11_W {
        AUXIO11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&mut self) -> AUXIO10_W {
        AUXIO10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&mut self) -> AUXIO9_W {
        AUXIO9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&mut self) -> AUXIO8_W {
        AUXIO8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&mut self) -> AUXIO7_W {
        AUXIO7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&mut self) -> AUXIO6_W {
        AUXIO6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&mut self) -> AUXIO5_W {
        AUXIO5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&mut self) -> AUXIO4_W {
        AUXIO4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&mut self) -> AUXIO3_W {
        AUXIO3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&mut self) -> AUXIO2_W {
        AUXIO2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&mut self) -> AUXIO1_W {
        AUXIO1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&mut self) -> AUXIO0_W {
        AUXIO0_W { w: self }
    }
}
