#[doc = "Reader of register OPMODEREQ"]
pub type R = crate::R<u32, super::OPMODEREQ>;
#[doc = "Writer for register OPMODEREQ"]
pub type W = crate::W<u32, super::OPMODEREQ>;
#[doc = "Register OPMODEREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::OPMODEREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:0\\]
AUX operational mode request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REQ_A {
    #[doc = "3: Powerdown operational mode with wakeup to lowpower mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    PDLP = 3,
    #[doc = "2: Powerdown operational mode with wakeup to active mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    PDA = 2,
    #[doc = "1: Lowpower operational mode, characterized by:\n- Powerdown system power supply state (uLDO) request.\n- SCE clock frequency (SCE_RATE) equals SCLK_MF.\n- An active wakeup flag does not change operational mode."]
    LP = 1,
    #[doc = "0: Active operational mode, characterized by:\n- Active system power supply state (GLDO or DCDC) request. \n- AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE).\n- An active wakeup flag does not change operational mode."]
    A = 0,
}
impl From<REQ_A> for u8 {
    #[inline(always)]
    fn from(variant: REQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REQ`"]
pub type REQ_R = crate::R<u8, REQ_A>;
impl REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_A {
        match self.bits {
            3 => REQ_A::PDLP,
            2 => REQ_A::PDA,
            1 => REQ_A::LP,
            0 => REQ_A::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PDLP`"]
    #[inline(always)]
    pub fn is_pdlp(&self) -> bool {
        *self == REQ_A::PDLP
    }
    #[doc = "Checks if the value of the field is `PDA`"]
    #[inline(always)]
    pub fn is_pda(&self) -> bool {
        *self == REQ_A::PDA
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == REQ_A::LP
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == REQ_A::A
    }
}
#[doc = "Write proxy for field `REQ`"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    #[inline(always)]
    pub fn pdlp(self) -> &'a mut W {
        self.variant(REQ_A::PDLP)
    }
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    #[inline(always)]
    pub fn pda(self) -> &'a mut W {
        self.variant(REQ_A::PDA)
    }
    #[doc = "Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(REQ_A::LP)
    }
    #[doc = "Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(REQ_A::A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
AUX operational mode request."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
AUX operational mode request."]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
}
