#[doc = "Reader of register EVSTAT1"]
pub type R = crate::R<u32, super::EVSTAT1>;
#[doc = "Writer for register EVSTAT1"]
pub type W = crate::W<u32, super::EVSTAT1>;
#[doc = "Register EVSTAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSTAT1 {
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
#[doc = "Reader of field `AUXIO31`"]
pub type AUXIO31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO31`"]
pub struct AUXIO31_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO31_W<'a> {
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
#[doc = "Reader of field `AUXIO30`"]
pub type AUXIO30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO30`"]
pub struct AUXIO30_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO30_W<'a> {
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
#[doc = "Reader of field `AUXIO29`"]
pub type AUXIO29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO29`"]
pub struct AUXIO29_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO29_W<'a> {
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
#[doc = "Reader of field `AUXIO28`"]
pub type AUXIO28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO28`"]
pub struct AUXIO28_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO28_W<'a> {
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
#[doc = "Reader of field `AUXIO27`"]
pub type AUXIO27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO27`"]
pub struct AUXIO27_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO27_W<'a> {
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
#[doc = "Reader of field `AUXIO26`"]
pub type AUXIO26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO26`"]
pub struct AUXIO26_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO26_W<'a> {
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
#[doc = "Reader of field `AUXIO25`"]
pub type AUXIO25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO25`"]
pub struct AUXIO25_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO25_W<'a> {
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
#[doc = "Reader of field `AUXIO24`"]
pub type AUXIO24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO24`"]
pub struct AUXIO24_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO24_W<'a> {
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
#[doc = "Reader of field `AUXIO23`"]
pub type AUXIO23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO23`"]
pub struct AUXIO23_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO23_W<'a> {
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
#[doc = "Reader of field `AUXIO22`"]
pub type AUXIO22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO22`"]
pub struct AUXIO22_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO22_W<'a> {
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
#[doc = "Reader of field `AUXIO21`"]
pub type AUXIO21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO21`"]
pub struct AUXIO21_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO21_W<'a> {
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
#[doc = "Reader of field `AUXIO20`"]
pub type AUXIO20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO20`"]
pub struct AUXIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO20_W<'a> {
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
#[doc = "Reader of field `AUXIO19`"]
pub type AUXIO19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO19`"]
pub struct AUXIO19_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO19_W<'a> {
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
#[doc = "Reader of field `AUXIO18`"]
pub type AUXIO18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO18`"]
pub struct AUXIO18_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO18_W<'a> {
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
#[doc = "Reader of field `AUXIO17`"]
pub type AUXIO17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO17`"]
pub struct AUXIO17_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO17_W<'a> {
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
#[doc = "Reader of field `AUXIO16`"]
pub type AUXIO16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXIO16`"]
pub struct AUXIO16_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO16_W<'a> {
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
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio31(&self) -> AUXIO31_R {
        AUXIO31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio30(&self) -> AUXIO30_R {
        AUXIO30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio29(&self) -> AUXIO29_R {
        AUXIO29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio28(&self) -> AUXIO28_R {
        AUXIO28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio27(&self) -> AUXIO27_R {
        AUXIO27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio26(&self) -> AUXIO26_R {
        AUXIO26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio25(&self) -> AUXIO25_R {
        AUXIO25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio24(&self) -> AUXIO24_R {
        AUXIO24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio23(&self) -> AUXIO23_R {
        AUXIO23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio22(&self) -> AUXIO22_R {
        AUXIO22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio21(&self) -> AUXIO21_R {
        AUXIO21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio20(&self) -> AUXIO20_R {
        AUXIO20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio19(&self) -> AUXIO19_R {
        AUXIO19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio18(&self) -> AUXIO18_R {
        AUXIO18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio17(&self) -> AUXIO17_R {
        AUXIO17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio16(&self) -> AUXIO16_R {
        AUXIO16_R::new((self.bits & 0x01) != 0)
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
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio31(&mut self) -> AUXIO31_W {
        AUXIO31_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio30(&mut self) -> AUXIO30_W {
        AUXIO30_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio29(&mut self) -> AUXIO29_W {
        AUXIO29_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio28(&mut self) -> AUXIO28_W {
        AUXIO28_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio27(&mut self) -> AUXIO27_W {
        AUXIO27_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio26(&mut self) -> AUXIO26_W {
        AUXIO26_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio25(&mut self) -> AUXIO25_W {
        AUXIO25_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio24(&mut self) -> AUXIO24_W {
        AUXIO24_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio23(&mut self) -> AUXIO23_W {
        AUXIO23_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio22(&mut self) -> AUXIO22_W {
        AUXIO22_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio21(&mut self) -> AUXIO21_W {
        AUXIO21_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio20(&mut self) -> AUXIO20_W {
        AUXIO20_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio19(&mut self) -> AUXIO19_W {
        AUXIO19_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio18(&mut self) -> AUXIO18_W {
        AUXIO18_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio17(&mut self) -> AUXIO17_W {
        AUXIO17_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio16(&mut self) -> AUXIO16_W {
        AUXIO16_W { w: self }
    }
}
