#[doc = "Reader of register OPMODEACK"]
pub type R = crate::R<u32, super::OPMODEACK>;
#[doc = "Writer for register OPMODEACK"]
pub type W = crate::W<u32, super::OPMODEACK>;
#[doc = "Register OPMODEACK `reset()`'s with value 0"]
impl crate::ResetValue for super::OPMODEACK {
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
AUX operational mode acknowledgement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACK_A {
    #[doc = "3: Powerdown operational mode with wakeup to lowpower mode is acknowledged."]
    PDLP = 3,
    #[doc = "2: Powerdown operational mode with wakeup to active mode is acknowledged."]
    PDA = 2,
    #[doc = "1: Lowpower operational mode is acknowledged."]
    LP = 1,
    #[doc = "0: Active operational mode is acknowledged."]
    A = 0,
}
impl From<ACK_A> for u8 {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<u8, ACK_A>;
impl ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            3 => ACK_A::PDLP,
            2 => ACK_A::PDA,
            1 => ACK_A::LP,
            0 => ACK_A::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PDLP`"]
    #[inline(always)]
    pub fn is_pdlp(&self) -> bool {
        *self == ACK_A::PDLP
    }
    #[doc = "Checks if the value of the field is `PDA`"]
    #[inline(always)]
    pub fn is_pda(&self) -> bool {
        *self == ACK_A::PDA
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == ACK_A::LP
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == ACK_A::A
    }
}
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Powerdown operational mode with wakeup to lowpower mode is acknowledged."]
    #[inline(always)]
    pub fn pdlp(self) -> &'a mut W {
        self.variant(ACK_A::PDLP)
    }
    #[doc = "Powerdown operational mode with wakeup to active mode is acknowledged."]
    #[inline(always)]
    pub fn pda(self) -> &'a mut W {
        self.variant(ACK_A::PDA)
    }
    #[doc = "Lowpower operational mode is acknowledged."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(ACK_A::LP)
    }
    #[doc = "Active operational mode is acknowledged."]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(ACK_A::A)
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
AUX operational mode acknowledgement."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 0x03) as u8)
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
AUX operational mode acknowledgement."]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
}
