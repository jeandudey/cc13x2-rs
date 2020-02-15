#[doc = "Reader of register AIRCR"]
pub type R = crate::R<u32, super::AIRCR>;
#[doc = "Writer for register AIRCR"]
pub type W = crate::W<u32, super::AIRCR>;
#[doc = "Register AIRCR `reset()`'s with value 0xfa05_0000"]
impl crate::ResetValue for super::AIRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0000
    }
}
#[doc = "Reader of field `VECTKEY`"]
pub type VECTKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTKEY`"]
pub struct VECTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "15:15\\]
Data endianness bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANESS_A {
    #[doc = "1: Big endian"]
    BIG = 1,
    #[doc = "0: Little endian"]
    LITTLE = 0,
}
impl From<ENDIANESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDIANESS`"]
pub type ENDIANESS_R = crate::R<bool, ENDIANESS_A>;
impl ENDIANESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANESS_A {
        match self.bits {
            true => ENDIANESS_A::BIG,
            false => ENDIANESS_A::LITTLE,
        }
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIANESS_A::BIG
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIANESS_A::LITTLE
    }
}
#[doc = "Write proxy for field `ENDIANESS`"]
pub struct ENDIANESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIANESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIANESS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIANESS_A::BIG)
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIANESS_A::LITTLE)
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
#[doc = "Reader of field `RESERVED11`"]
pub type RESERVED11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `PRIGROUP`"]
pub type PRIGROUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIGROUP`"]
pub struct PRIGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `SYSRESETREQ`"]
pub type SYSRESETREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRESETREQ`"]
pub struct SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESETREQ_W<'a> {
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
#[doc = "Reader of field `VECTCLRACTIVE`"]
pub type VECTCLRACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VECTCLRACTIVE`"]
pub struct VECTCLRACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTCLRACTIVE_W<'a> {
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
#[doc = "Reader of field `VECTRESET`"]
pub type VECTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VECTRESET`"]
pub struct VECTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTRESET_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Data endianness bit"]
    #[inline(always)]
    pub fn endianess(&self) -> ENDIANESS_R {
        ENDIANESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline(always)]
    pub fn vectreset(&self) -> VECTRESET_R {
        VECTRESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    pub fn vectkey(&mut self) -> VECTKEY_W {
        VECTKEY_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Data endianness bit"]
    #[inline(always)]
    pub fn endianess(&mut self) -> ENDIANESS_W {
        ENDIANESS_W { w: self }
    }
    #[doc = "Bits 11:14 - 14:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    pub fn prigroup(&mut self) -> PRIGROUP_W {
        PRIGROUP_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W {
        SYSRESETREQ_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W {
        VECTCLRACTIVE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline(always)]
    pub fn vectreset(&mut self) -> VECTRESET_W {
        VECTRESET_W { w: self }
    }
}
