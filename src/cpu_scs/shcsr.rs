#[doc = "Reader of register SHCSR"]
pub type R = crate::R<u32, super::SHCSR>;
#[doc = "Writer for register SHCSR"]
pub type W = crate::W<u32, super::SHCSR>;
#[doc = "Register SHCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED19`"]
pub type RESERVED19_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED19`"]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 19)) | (((value as u32) & 0x1fff) << 19);
        self.w
    }
}
#[doc = "18:18\\]
Usage fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<USGFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USGFAULTENA`"]
pub type USGFAULTENA_R = crate::R<bool, USGFAULTENA_A>;
impl USGFAULTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTENA_A {
        match self.bits {
            true => USGFAULTENA_A::EN,
            false => USGFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == USGFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == USGFAULTENA_A::DIS
    }
}
#[doc = "Write proxy for field `USGFAULTENA`"]
pub struct USGFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "17:17\\]
Bus fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<BUSFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSFAULTENA`"]
pub type BUSFAULTENA_R = crate::R<bool, BUSFAULTENA_A>;
impl BUSFAULTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTENA_A {
        match self.bits {
            true => BUSFAULTENA_A::EN,
            false => BUSFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BUSFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BUSFAULTENA_A::DIS
    }
}
#[doc = "Write proxy for field `BUSFAULTENA`"]
pub struct BUSFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "16:16\\]
MemManage fault system handler enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENA_A {
    #[doc = "1: Exception enabled"]
    EN = 1,
    #[doc = "0: Exception disabled"]
    DIS = 0,
}
impl From<MEMFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMFAULTENA`"]
pub type MEMFAULTENA_R = crate::R<bool, MEMFAULTENA_A>;
impl MEMFAULTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTENA_A {
        match self.bits {
            true => MEMFAULTENA_A::EN,
            false => MEMFAULTENA_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEMFAULTENA_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MEMFAULTENA_A::DIS
    }
}
#[doc = "Write proxy for field `MEMFAULTENA`"]
pub struct MEMFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::EN)
    }
    #[doc = "Exception disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "15:15\\]
SVCall pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<SVCALLPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVCALLPENDED`"]
pub type SVCALLPENDED_R = crate::R<bool, SVCALLPENDED_A>;
impl SVCALLPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLPENDED_A {
        match self.bits {
            true => SVCALLPENDED_A::PENDING,
            false => SVCALLPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SVCALLPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == SVCALLPENDED_A::NOTPENDING
    }
}
#[doc = "Write proxy for field `SVCALLPENDED`"]
pub struct SVCALLPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::NOTPENDING)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "14:14\\]
BusFault pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<BUSFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSFAULTPENDED`"]
pub type BUSFAULTPENDED_R = crate::R<bool, BUSFAULTPENDED_A>;
impl BUSFAULTPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTPENDED_A {
        match self.bits {
            true => BUSFAULTPENDED_A::PENDING,
            false => BUSFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BUSFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == BUSFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Write proxy for field `BUSFAULTPENDED`"]
pub struct BUSFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::NOTPENDING)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "13:13\\]
MemManage exception pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<MEMFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMFAULTPENDED`"]
pub type MEMFAULTPENDED_R = crate::R<bool, MEMFAULTPENDED_A>;
impl MEMFAULTPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTPENDED_A {
        match self.bits {
            true => MEMFAULTPENDED_A::PENDING,
            false => MEMFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MEMFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == MEMFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Write proxy for field `MEMFAULTPENDED`"]
pub struct MEMFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::NOTPENDING)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "12:12\\]
Usage fault pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDED_A {
    #[doc = "1: Exception is pending."]
    PENDING = 1,
    #[doc = "0: Exception is not active"]
    NOTPENDING = 0,
}
impl From<USGFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USGFAULTPENDED`"]
pub type USGFAULTPENDED_R = crate::R<bool, USGFAULTPENDED_A>;
impl USGFAULTPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTPENDED_A {
        match self.bits {
            true => USGFAULTPENDED_A::PENDING,
            false => USGFAULTPENDED_A::NOTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == USGFAULTPENDED_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        *self == USGFAULTPENDED_A::NOTPENDING
    }
}
#[doc = "Write proxy for field `USGFAULTPENDED`"]
pub struct USGFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::PENDING)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::NOTPENDING)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<SYSTICKACT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTICKACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSTICKACT`"]
pub type SYSTICKACT_R = crate::R<bool, SYSTICKACT_A>;
impl SYSTICKACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTICKACT_A {
        match self.bits {
            true => SYSTICKACT_A::ACTIVE,
            false => SYSTICKACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SYSTICKACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == SYSTICKACT_A::NOTACTIVE
    }
}
#[doc = "Write proxy for field `SYSTICKACT`"]
pub struct SYSTICKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICKACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTICKACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::NOTACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PENDSVACT`"]
pub type PENDSVACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDSVACT`"]
pub struct PENDSVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVACT_W<'a> {
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
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
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
#[doc = "8:8\\]
Debug monitor active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<MONITORACT_A> for bool {
    #[inline(always)]
    fn from(variant: MONITORACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONITORACT`"]
pub type MONITORACT_R = crate::R<bool, MONITORACT_A>;
impl MONITORACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONITORACT_A {
        match self.bits {
            true => MONITORACT_A::ACTIVE,
            false => MONITORACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MONITORACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == MONITORACT_A::NOTACTIVE
    }
}
#[doc = "Write proxy for field `MONITORACT`"]
pub struct MONITORACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONITORACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONITORACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MONITORACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(MONITORACT_A::NOTACTIVE)
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
#[doc = "7:7\\]
SVCall active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<SVCALLACT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVCALLACT`"]
pub type SVCALLACT_R = crate::R<bool, SVCALLACT_A>;
impl SVCALLACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLACT_A {
        match self.bits {
            true => SVCALLACT_A::ACTIVE,
            false => SVCALLACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SVCALLACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == SVCALLACT_A::NOTACTIVE
    }
}
#[doc = "Write proxy for field `SVCALLACT`"]
pub struct SVCALLACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SVCALLACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(SVCALLACT_A::NOTACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "3:3\\]
UsageFault exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<USGFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USGFAULTACT`"]
pub type USGFAULTACT_R = crate::R<bool, USGFAULTACT_A>;
impl USGFAULTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTACT_A {
        match self.bits {
            true => USGFAULTACT_A::ACTIVE,
            false => USGFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == USGFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == USGFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Write proxy for field `USGFAULTACT`"]
pub struct USGFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::NOTACTIVE)
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
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
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
BusFault exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<BUSFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSFAULTACT`"]
pub type BUSFAULTACT_R = crate::R<bool, BUSFAULTACT_A>;
impl BUSFAULTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTACT_A {
        match self.bits {
            true => BUSFAULTACT_A::ACTIVE,
            false => BUSFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == BUSFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Write proxy for field `BUSFAULTACT`"]
pub struct BUSFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::NOTACTIVE)
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
MemManage exception active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACT_A {
    #[doc = "1: Exception is active"]
    ACTIVE = 1,
    #[doc = "0: Exception is not active"]
    NOTACTIVE = 0,
}
impl From<MEMFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMFAULTACT`"]
pub type MEMFAULTACT_R = crate::R<bool, MEMFAULTACT_A>;
impl MEMFAULTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTACT_A {
        match self.bits {
            true => MEMFAULTACT_A::ACTIVE,
            false => MEMFAULTACT_A::NOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MEMFAULTACT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == MEMFAULTACT_A::NOTACTIVE
    }
}
#[doc = "Write proxy for field `MEMFAULTACT`"]
pub struct MEMFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exception is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::ACTIVE)
    }
    #[doc = "Exception is not active"]
    #[inline(always)]
    pub fn notactive(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::NOTACTIVE)
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
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bit 18 - 18:18\\]
Usage fault system handler enable"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Bus fault system handler enable"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MemManage fault system handler enable"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
SVCall pending"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
BusFault pending"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MemManage exception pending"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Usage fault pending"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Debug monitor active"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
SVCall active"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
UsageFault exception active"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
BusFault exception active"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
MemManage exception active"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Usage fault system handler enable"]
    #[inline(always)]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W {
        USGFAULTENA_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Bus fault system handler enable"]
    #[inline(always)]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W {
        BUSFAULTENA_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
MemManage fault system handler enable"]
    #[inline(always)]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W {
        MEMFAULTENA_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
SVCall pending"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W {
        SVCALLPENDED_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
BusFault pending"]
    #[inline(always)]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W {
        BUSFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
MemManage exception pending"]
    #[inline(always)]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W {
        MEMFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Usage fault pending"]
    #[inline(always)]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W {
        USGFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
SysTick active flag. 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn systickact(&mut self) -> SYSTICKACT_W {
        SYSTICKACT_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
PendSV active 0x0: Not active 0x1: Active"]
    #[inline(always)]
    pub fn pendsvact(&mut self) -> PENDSVACT_W {
        PENDSVACT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Debug monitor active"]
    #[inline(always)]
    pub fn monitoract(&mut self) -> MONITORACT_W {
        MONITORACT_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
SVCall active"]
    #[inline(always)]
    pub fn svcallact(&mut self) -> SVCALLACT_W {
        SVCALLACT_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
UsageFault exception active"]
    #[inline(always)]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W {
        USGFAULTACT_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
BusFault exception active"]
    #[inline(always)]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W {
        BUSFAULTACT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
MemManage exception active"]
    #[inline(always)]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W {
        MEMFAULTACT_W { w: self }
    }
}
