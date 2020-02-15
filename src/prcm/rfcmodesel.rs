#[doc = "Reader of register RFCMODESEL"]
pub type R = crate::R<u32, super::RFCMODESEL>;
#[doc = "Writer for register RFCMODESEL"]
pub type W = crate::W<u32, super::RFCMODESEL>;
#[doc = "Register RFCMODESEL `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCMODESEL {
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
#[doc = "2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CURR_A {
    #[doc = "7: Select Mode 7"]
    MODE7 = 7,
    #[doc = "6: Select Mode 6"]
    MODE6 = 6,
    #[doc = "5: Select Mode 5"]
    MODE5 = 5,
    #[doc = "4: Select Mode 4"]
    MODE4 = 4,
    #[doc = "3: Select Mode 3"]
    MODE3 = 3,
    #[doc = "2: Select Mode 2"]
    MODE2 = 2,
    #[doc = "1: Select Mode 1"]
    MODE1 = 1,
    #[doc = "0: Select Mode 0"]
    MODE0 = 0,
}
impl From<CURR_A> for u8 {
    #[inline(always)]
    fn from(variant: CURR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CURR`"]
pub type CURR_R = crate::R<u8, CURR_A>;
impl CURR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURR_A {
        match self.bits {
            7 => CURR_A::MODE7,
            6 => CURR_A::MODE6,
            5 => CURR_A::MODE5,
            4 => CURR_A::MODE4,
            3 => CURR_A::MODE3,
            2 => CURR_A::MODE2,
            1 => CURR_A::MODE1,
            0 => CURR_A::MODE0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE7`"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == CURR_A::MODE7
    }
    #[doc = "Checks if the value of the field is `MODE6`"]
    #[inline(always)]
    pub fn is_mode6(&self) -> bool {
        *self == CURR_A::MODE6
    }
    #[doc = "Checks if the value of the field is `MODE5`"]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == CURR_A::MODE5
    }
    #[doc = "Checks if the value of the field is `MODE4`"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == CURR_A::MODE4
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == CURR_A::MODE3
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == CURR_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == CURR_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == CURR_A::MODE0
    }
}
#[doc = "Write proxy for field `CURR`"]
pub struct CURR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select Mode 7"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut W {
        self.variant(CURR_A::MODE7)
    }
    #[doc = "Select Mode 6"]
    #[inline(always)]
    pub fn mode6(self) -> &'a mut W {
        self.variant(CURR_A::MODE6)
    }
    #[doc = "Select Mode 5"]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut W {
        self.variant(CURR_A::MODE5)
    }
    #[doc = "Select Mode 4"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut W {
        self.variant(CURR_A::MODE4)
    }
    #[doc = "Select Mode 3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(CURR_A::MODE3)
    }
    #[doc = "Select Mode 2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(CURR_A::MODE2)
    }
    #[doc = "Select Mode 1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(CURR_A::MODE1)
    }
    #[doc = "Select Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(CURR_A::MODE0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
    #[doc = "Bits 0:2 - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline(always)]
    pub fn curr(&self) -> CURR_R {
        CURR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline(always)]
    pub fn curr(&mut self) -> CURR_W {
        CURR_W { w: self }
    }
}
