#[doc = "Reader of register VECCFG5"]
pub type R = crate::R<u32, super::VECCFG5>;
#[doc = "Writer for register VECCFG5"]
pub type W = crate::W<u32, super::VECCFG5>;
#[doc = "Register VECCFG5 `reset()`'s with value 0"]
impl crate::ResetValue for super::VECCFG5 {
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
#[doc = "3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VEC_EV_A {
    #[doc = "9: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AON_RTC_CH2_DLY = 9,
    #[doc = "8: WUFLAGS.SW_WU3"]
    SW_WU3 = 8,
    #[doc = "7: WUFLAGS.SW_WU2"]
    SW_WU2 = 7,
    #[doc = "6: WUFLAGS.SW_WU1"]
    SW_WU1 = 6,
    #[doc = "5: WUFLAGS.SW_WU0"]
    SW_WU0 = 5,
    #[doc = "4: WUFLAGS.PROG_WU3"]
    PROG_WU3 = 4,
    #[doc = "3: WUFLAGS.PROG_WU2"]
    PROG_WU2 = 3,
    #[doc = "2: WUFLAGS.PROG_WU1"]
    PROG_WU1 = 2,
    #[doc = "1: WUFLAGS.PROG_WU0"]
    PROG_WU0 = 1,
    #[doc = "0: Vector is disabled."]
    NONE = 0,
}
impl From<VEC_EV_A> for u8 {
    #[inline(always)]
    fn from(variant: VEC_EV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VEC_EV`"]
pub type VEC_EV_R = crate::R<u8, VEC_EV_A>;
impl VEC_EV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VEC_EV_A> {
        use crate::Variant::*;
        match self.bits {
            9 => Val(VEC_EV_A::AON_RTC_CH2_DLY),
            8 => Val(VEC_EV_A::SW_WU3),
            7 => Val(VEC_EV_A::SW_WU2),
            6 => Val(VEC_EV_A::SW_WU1),
            5 => Val(VEC_EV_A::SW_WU0),
            4 => Val(VEC_EV_A::PROG_WU3),
            3 => Val(VEC_EV_A::PROG_WU2),
            2 => Val(VEC_EV_A::PROG_WU1),
            1 => Val(VEC_EV_A::PROG_WU0),
            0 => Val(VEC_EV_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2_DLY`"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == VEC_EV_A::AON_RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `SW_WU3`"]
    #[inline(always)]
    pub fn is_sw_wu3(&self) -> bool {
        *self == VEC_EV_A::SW_WU3
    }
    #[doc = "Checks if the value of the field is `SW_WU2`"]
    #[inline(always)]
    pub fn is_sw_wu2(&self) -> bool {
        *self == VEC_EV_A::SW_WU2
    }
    #[doc = "Checks if the value of the field is `SW_WU1`"]
    #[inline(always)]
    pub fn is_sw_wu1(&self) -> bool {
        *self == VEC_EV_A::SW_WU1
    }
    #[doc = "Checks if the value of the field is `SW_WU0`"]
    #[inline(always)]
    pub fn is_sw_wu0(&self) -> bool {
        *self == VEC_EV_A::SW_WU0
    }
    #[doc = "Checks if the value of the field is `PROG_WU3`"]
    #[inline(always)]
    pub fn is_prog_wu3(&self) -> bool {
        *self == VEC_EV_A::PROG_WU3
    }
    #[doc = "Checks if the value of the field is `PROG_WU2`"]
    #[inline(always)]
    pub fn is_prog_wu2(&self) -> bool {
        *self == VEC_EV_A::PROG_WU2
    }
    #[doc = "Checks if the value of the field is `PROG_WU1`"]
    #[inline(always)]
    pub fn is_prog_wu1(&self) -> bool {
        *self == VEC_EV_A::PROG_WU1
    }
    #[doc = "Checks if the value of the field is `PROG_WU0`"]
    #[inline(always)]
    pub fn is_prog_wu0(&self) -> bool {
        *self == VEC_EV_A::PROG_WU0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VEC_EV_A::NONE
    }
}
#[doc = "Write proxy for field `VEC_EV`"]
pub struct VEC_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VEC_EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut W {
        self.variant(VEC_EV_A::AON_RTC_CH2_DLY)
    }
    #[doc = "WUFLAGS.SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU3)
    }
    #[doc = "WUFLAGS.SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU2)
    }
    #[doc = "WUFLAGS.SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU1)
    }
    #[doc = "WUFLAGS.SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(self) -> &'a mut W {
        self.variant(VEC_EV_A::SW_WU0)
    }
    #[doc = "WUFLAGS.PROG_WU3"]
    #[inline(always)]
    pub fn prog_wu3(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU3)
    }
    #[doc = "WUFLAGS.PROG_WU2"]
    #[inline(always)]
    pub fn prog_wu2(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU2)
    }
    #[doc = "WUFLAGS.PROG_WU1"]
    #[inline(always)]
    pub fn prog_wu1(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU1)
    }
    #[doc = "WUFLAGS.PROG_WU0"]
    #[inline(always)]
    pub fn prog_wu0(self) -> &'a mut W {
        self.variant(VEC_EV_A::PROG_WU0)
    }
    #[doc = "Vector is disabled."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(VEC_EV_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
    #[doc = "Bits 0:3 - 3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE."]
    #[inline(always)]
    pub fn vec_ev(&self) -> VEC_EV_R {
        VEC_EV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Select trigger event for vector 5. Non-enumerated values are treated as NONE."]
    #[inline(always)]
    pub fn vec_ev(&mut self) -> VEC_EV_W {
        VEC_EV_W { w: self }
    }
}
