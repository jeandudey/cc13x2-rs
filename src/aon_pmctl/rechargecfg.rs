#[doc = "Reader of register RECHARGECFG"]
pub type R = crate::R<u32, super::RECHARGECFG>;
#[doc = "Writer for register RECHARGECFG"]
pub type W = crate::W<u32, super::RECHARGECFG>;
#[doc = "Register RECHARGECFG `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::RECHARGECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "3: External recharge comparator. \nNote that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting  this recharge algorithm."]
    COMPARATOR = 3,
    #[doc = "2: Adaptive timer"]
    ADAPTIVE = 2,
    #[doc = "1: Static timer"]
    STATIC = 1,
    #[doc = "0: Recharge disabled"]
    OFF = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            3 => MODE_A::COMPARATOR,
            2 => MODE_A::ADAPTIVE,
            1 => MODE_A::STATIC,
            0 => MODE_A::OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMPARATOR`"]
    #[inline(always)]
    pub fn is_comparator(&self) -> bool {
        *self == MODE_A::COMPARATOR
    }
    #[doc = "Checks if the value of the field is `ADAPTIVE`"]
    #[inline(always)]
    pub fn is_adaptive(&self) -> bool {
        *self == MODE_A::ADAPTIVE
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static_(&self) -> bool {
        *self == MODE_A::STATIC
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE_A::OFF
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External recharge comparator. Note that the clock to the recharge comparator must be enabled, \\[ANATOP_MMAP:ADI_3_REFSYS:CTL_RECHARGE_CMP0:COMP_CLK_DISABLE\\], before selecting this recharge algorithm."]
    #[inline(always)]
    pub fn comparator(self) -> &'a mut W {
        self.variant(MODE_A::COMPARATOR)
    }
    #[doc = "Adaptive timer"]
    #[inline(always)]
    pub fn adaptive(self) -> &'a mut W {
        self.variant(MODE_A::ADAPTIVE)
    }
    #[doc = "Static timer"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(MODE_A::STATIC)
    }
    #[doc = "Recharge disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `C2`"]
pub type C2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C2`"]
pub struct C2_W<'a> {
    w: &'a mut W,
}
impl<'a> C2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `C1`"]
pub type C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C1`"]
pub struct C1_W<'a> {
    w: &'a mut W,
}
impl<'a> C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_PER_M`"]
pub type MAX_PER_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_PER_M`"]
pub struct MAX_PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MAX_PER_E`"]
pub type MAX_PER_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_PER_E`"]
pub struct MAX_PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PER_M`"]
pub type PER_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_M`"]
pub struct PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PER_E`"]
pub type PER_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_E`"]
pub struct PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_m(&self) -> MAX_PER_M_R {
        MAX_PER_M_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_e(&self) -> MAX_PER_E_R {
        MAX_PER_E_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Selects recharge algorithm for VDDR when the system is running on the uLDO"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W {
        C2_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W {
        C1_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_m(&mut self) -> MAX_PER_M_W {
        MAX_PER_M_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_per_e(&mut self) -> MAX_PER_E_W {
        MAX_PER_E_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&mut self) -> PER_M_W {
        PER_M_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&mut self) -> PER_E_W {
        PER_E_W { w: self }
    }
}
