#[doc = "Reader of register OPTIONS"]
pub type R = crate::R<u32, super::OPTIONS>;
#[doc = "Writer for register OPTIONS"]
pub type W = crate::W<u32, super::OPTIONS>;
#[doc = "Register OPTIONS `reset()`'s with value 0x20"]
impl crate::ResetValue for super::OPTIONS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `INT_MASKING`"]
pub type INT_MASKING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_MASKING`"]
pub struct INT_MASKING_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASKING_W<'a> {
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
#[doc = "Reader of field `PROTECTION_OPTION`"]
pub type PROTECTION_OPTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTECTION_OPTION`"]
pub struct PROTECTION_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECTION_OPTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PROGRAM_RAM`"]
pub type PROGRAM_RAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGRAM_RAM`"]
pub struct PROGRAM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGRAM_RAM_W<'a> {
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
#[doc = "Reader of field `SEQUENCER_CONFIGURATION`"]
pub type SEQUENCER_CONFIGURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCER_CONFIGURATION`"]
pub struct SEQUENCER_CONFIGURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCER_CONFIGURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `PKCP_CONFIGURATION`"]
pub type PKCP_CONFIGURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PKCP_CONFIGURATION`"]
pub struct PKCP_CONFIGURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> PKCP_CONFIGURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&self) -> INT_MASKING_R {
        INT_MASKING_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&self) -> PROTECTION_OPTION_R {
        PROTECTION_OPTION_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&self) -> PROGRAM_RAM_R {
        PROGRAM_RAM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SEQUENCER_CONFIGURATION_R {
        SEQUENCER_CONFIGURATION_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PKCP_CONFIGURATION_R {
        PKCP_CONFIGURATION_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Masking 0x0: indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, 0x1 : indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&mut self) -> INT_MASKING_W {
        INT_MASKING_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Protection Option 0x0: indicates no additional protection against side channel attacks, 0x1: indicates the SCAP option 0x2: Reserved 0x3: indicates the PROT option; Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&mut self) -> PROTECTION_OPTION_W {
        PROTECTION_OPTION_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Program RAM 0x1: indicates sequencer program storage in RAM, 0x0: indicates sequencer program storage in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&mut self) -> PROGRAM_RAM_W {
        PROGRAM_RAM_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
Sequencer Configuration 0x0: Reserved 0x1 : Indicates a standard sequencer 0x2: Reserved 0x3: Reserved"]
    #[inline(always)]
    pub fn sequencer_configuration(&mut self) -> SEQUENCER_CONFIGURATION_W {
        SEQUENCER_CONFIGURATION_W { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
PKCP Configuration 0x0 : Reserved 0x1 : Indicates a PKCP with a 16x16 multiplier, 0x2: indicates a PKCP with a 32x32 multiplier, 0x3 : Reserved Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&mut self) -> PKCP_CONFIGURATION_W {
        PKCP_CONFIGURATION_W { w: self }
    }
}
