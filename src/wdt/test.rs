#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_A {
    #[doc = "1: Enable STALL"]
    EN = 1,
    #[doc = "0: Disable STALL"]
    DIS = 0,
}
impl From<STALL_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, STALL_A>;
impl STALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALL_A {
        match self.bits {
            true => STALL_A::EN,
            false => STALL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STALL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STALL_A::DIS
    }
}
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable STALL"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STALL_A::EN)
    }
    #[doc = "Disable STALL"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STALL_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_EN_A {
    #[doc = "1: Test mode Enabled"]
    EN = 1,
    #[doc = "0: Test mode Disabled"]
    DIS = 0,
}
impl From<TEST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEST_EN`"]
pub type TEST_EN_R = crate::R<bool, TEST_EN_A>;
impl TEST_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_EN_A {
        match self.bits {
            true => TEST_EN_A::EN,
            false => TEST_EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TEST_EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TEST_EN_A::DIS
    }
}
#[doc = "Write proxy for field `TEST_EN`"]
pub struct TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEST_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Test mode Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TEST_EN_A::EN)
    }
    #[doc = "Test mode Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TEST_EN_A::DIS)
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
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
    #[inline(always)]
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated"]
    #[inline(always)]
    pub fn test_en(&mut self) -> TEST_EN_W {
        TEST_EN_W { w: self }
    }
}
