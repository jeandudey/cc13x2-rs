#[doc = "Reader of register SPPR"]
pub type R = crate::R<u32, super::SPPR>;
#[doc = "Writer for register SPPR"]
pub type W = crate::W<u32, super::SPPR>;
#[doc = "Register SPPR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SPPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
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
Trace output protocol\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROTOCOL_A {
    #[doc = "2: SerialWire Output (NRZ)"]
    SWO_NRZ = 2,
    #[doc = "1: SerialWire Output (Manchester). This is the reset value."]
    SWO_MANCHESTER = 1,
    #[doc = "0: TracePort mode"]
    TRACEPORT = 0,
}
impl From<PROTOCOL_A> for u8 {
    #[inline(always)]
    fn from(variant: PROTOCOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PROTOCOL`"]
pub type PROTOCOL_R = crate::R<u8, PROTOCOL_A>;
impl PROTOCOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROTOCOL_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(PROTOCOL_A::SWO_NRZ),
            1 => Val(PROTOCOL_A::SWO_MANCHESTER),
            0 => Val(PROTOCOL_A::TRACEPORT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWO_NRZ`"]
    #[inline(always)]
    pub fn is_swo_nrz(&self) -> bool {
        *self == PROTOCOL_A::SWO_NRZ
    }
    #[doc = "Checks if the value of the field is `SWO_MANCHESTER`"]
    #[inline(always)]
    pub fn is_swo_manchester(&self) -> bool {
        *self == PROTOCOL_A::SWO_MANCHESTER
    }
    #[doc = "Checks if the value of the field is `TRACEPORT`"]
    #[inline(always)]
    pub fn is_traceport(&self) -> bool {
        *self == PROTOCOL_A::TRACEPORT
    }
}
#[doc = "Write proxy for field `PROTOCOL`"]
pub struct PROTOCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTOCOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTOCOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SerialWire Output (NRZ)"]
    #[inline(always)]
    pub fn swo_nrz(self) -> &'a mut W {
        self.variant(PROTOCOL_A::SWO_NRZ)
    }
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    #[inline(always)]
    pub fn swo_manchester(self) -> &'a mut W {
        self.variant(PROTOCOL_A::SWO_MANCHESTER)
    }
    #[doc = "TracePort mode"]
    #[inline(always)]
    pub fn traceport(self) -> &'a mut W {
        self.variant(PROTOCOL_A::TRACEPORT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
Trace output protocol"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new((self.bits & 0x03) as u8)
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
Trace output protocol"]
    #[inline(always)]
    pub fn protocol(&mut self) -> PROTOCOL_W {
        PROTOCOL_W { w: self }
    }
}
