#[doc = "Reader of register SIMR"]
pub type R = crate::R<u32, super::SIMR>;
#[doc = "Writer for register SIMR"]
pub type W = crate::W<u32, super::SIMR>;
#[doc = "Register SIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SIMR {
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
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<STOPIM_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPIM`"]
pub type STOPIM_R = crate::R<bool, STOPIM_A>;
impl STOPIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPIM_A {
        match self.bits {
            true => STOPIM_A::EN,
            false => STOPIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STOPIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STOPIM_A::DIS
    }
}
#[doc = "Write proxy for field `STOPIM`"]
pub struct STOPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOPIM_A::DIS)
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
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<STARTIM_A> for bool {
    #[inline(always)]
    fn from(variant: STARTIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STARTIM`"]
pub type STARTIM_R = crate::R<bool, STARTIM_A>;
impl STARTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTIM_A {
        match self.bits {
            true => STARTIM_A::EN,
            false => STARTIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == STARTIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STARTIM_A::DIS
    }
}
#[doc = "Write proxy for field `STARTIM`"]
pub struct STARTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(STARTIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STARTIM_A::DIS)
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
#[doc = "Reader of field `DATAIM`"]
pub type DATAIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAIM`"]
pub struct DATAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIM_W<'a> {
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
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&self) -> DATAIM_R {
        DATAIM_R::new((self.bits & 0x01) != 0)
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
Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&mut self) -> STOPIM_W {
        STOPIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&mut self) -> STARTIM_W {
        STARTIM_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&mut self) -> DATAIM_W {
        DATAIM_W { w: self }
    }
}
