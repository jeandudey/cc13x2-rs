#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "7:6\\]
Synchronize GPT Timer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC3_A {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    BOTH = 3,
    #[doc = "2: A timeout event for Timer B of GPT3 is triggered"]
    TIMERB = 2,
    #[doc = "1: A timeout event for Timer A of GPT3 is triggered"]
    TIMERA = 1,
    #[doc = "0: No Sync. GPT3 is not affected."]
    NOSYNC = 0,
}
impl From<SYNC3_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC3`"]
pub type SYNC3_R = crate::R<u8, SYNC3_A>;
impl SYNC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC3_A {
        match self.bits {
            3 => SYNC3_A::BOTH,
            2 => SYNC3_A::TIMERB,
            1 => SYNC3_A::TIMERA,
            0 => SYNC3_A::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SYNC3_A::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC3_A::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == SYNC3_A::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC3_A::NOSYNC
    }
}
#[doc = "Write proxy for field `SYNC3`"]
pub struct SYNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC3_A::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT3 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC3_A::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT3 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC3_A::TIMERA)
    }
    #[doc = "No Sync. GPT3 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC3_A::NOSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "5:4\\]
Synchronize GPT Timer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC2_A {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    BOTH = 3,
    #[doc = "2: A timeout event for Timer B of GPT2 is triggered"]
    TIMERB = 2,
    #[doc = "1: A timeout event for Timer A of GPT2 is triggered"]
    TIMERA = 1,
    #[doc = "0: No Sync. GPT2 is not affected."]
    NOSYNC = 0,
}
impl From<SYNC2_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC2`"]
pub type SYNC2_R = crate::R<u8, SYNC2_A>;
impl SYNC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC2_A {
        match self.bits {
            3 => SYNC2_A::BOTH,
            2 => SYNC2_A::TIMERB,
            1 => SYNC2_A::TIMERA,
            0 => SYNC2_A::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SYNC2_A::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC2_A::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == SYNC2_A::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC2_A::NOSYNC
    }
}
#[doc = "Write proxy for field `SYNC2`"]
pub struct SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC2_A::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT2 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC2_A::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT2 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC2_A::TIMERA)
    }
    #[doc = "No Sync. GPT2 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC2_A::NOSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "3:2\\]
Synchronize GPT Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC1_A {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    BOTH = 3,
    #[doc = "2: A timeout event for Timer B of GPT1 is triggered"]
    TIMERB = 2,
    #[doc = "1: A timeout event for Timer A of GPT1 is triggered"]
    TIMERA = 1,
    #[doc = "0: No Sync. GPT1 is not affected."]
    NOSYNC = 0,
}
impl From<SYNC1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC1`"]
pub type SYNC1_R = crate::R<u8, SYNC1_A>;
impl SYNC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC1_A {
        match self.bits {
            3 => SYNC1_A::BOTH,
            2 => SYNC1_A::TIMERB,
            1 => SYNC1_A::TIMERA,
            0 => SYNC1_A::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SYNC1_A::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC1_A::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == SYNC1_A::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC1_A::NOSYNC
    }
}
#[doc = "Write proxy for field `SYNC1`"]
pub struct SYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC1_A::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT1 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC1_A::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT1 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC1_A::TIMERA)
    }
    #[doc = "No Sync. GPT1 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC1_A::NOSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Synchronize GPT Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC0_A {
    #[doc = "3: A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    BOTH = 3,
    #[doc = "2: A timeout event for Timer B of GPT0 is triggered"]
    TIMERB = 2,
    #[doc = "1: A timeout event for Timer A of GPT0 is triggered"]
    TIMERA = 1,
    #[doc = "0: No Sync. GPT0 is not affected."]
    NOSYNC = 0,
}
impl From<SYNC0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC0`"]
pub type SYNC0_R = crate::R<u8, SYNC0_A>;
impl SYNC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC0_A {
        match self.bits {
            3 => SYNC0_A::BOTH,
            2 => SYNC0_A::TIMERB,
            1 => SYNC0_A::TIMERA,
            0 => SYNC0_A::NOSYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SYNC0_A::BOTH
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline(always)]
    pub fn is_timerb(&self) -> bool {
        *self == SYNC0_A::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline(always)]
    pub fn is_timera(&self) -> bool {
        *self == SYNC0_A::TIMERA
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == SYNC0_A::NOSYNC
    }
}
#[doc = "Write proxy for field `SYNC0`"]
pub struct SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SYNC0_A::BOTH)
    }
    #[doc = "A timeout event for Timer B of GPT0 is triggered"]
    #[inline(always)]
    pub fn timerb(self) -> &'a mut W {
        self.variant(SYNC0_A::TIMERB)
    }
    #[doc = "A timeout event for Timer A of GPT0 is triggered"]
    #[inline(always)]
    pub fn timera(self) -> &'a mut W {
        self.variant(SYNC0_A::TIMERA)
    }
    #[doc = "No Sync. GPT0 is not affected."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(SYNC0_A::NOSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Synchronize GPT Timer 3."]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronize GPT Timer 2."]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Synchronize GPT Timer 1"]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Synchronize GPT Timer 0"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Synchronize GPT Timer 3."]
    #[inline(always)]
    pub fn sync3(&mut self) -> SYNC3_W {
        SYNC3_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Synchronize GPT Timer 2."]
    #[inline(always)]
    pub fn sync2(&mut self) -> SYNC2_W {
        SYNC2_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Synchronize GPT Timer 1"]
    #[inline(always)]
    pub fn sync1(&mut self) -> SYNC1_W {
        SYNC1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Synchronize GPT Timer 0"]
    #[inline(always)]
    pub fn sync0(&mut self) -> SYNC0_W {
        SYNC0_W { w: self }
    }
}
