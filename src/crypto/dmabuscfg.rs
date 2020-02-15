#[doc = "Reader of register DMABUSCFG"]
pub type R = crate::R<u32, super::DMABUSCFG>;
#[doc = "Writer for register DMABUSCFG"]
pub type W = crate::W<u32, super::DMABUSCFG>;
#[doc = "Register DMABUSCFG `reset()`'s with value 0x2400"]
impl crate::ResetValue for super::DMABUSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "15:12\\]
Maximum burst size that can be performed on the AHB bus\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHB_MST1_BURST_SIZE_A {
    #[doc = "6: 64 bytes"]
    _64_BYTE = 6,
    #[doc = "5: 32 bytes"]
    _32_BYTE = 5,
    #[doc = "4: 16 bytes"]
    _16_BYTE = 4,
    #[doc = "3: 8 bytes"]
    _8_BYTE = 3,
    #[doc = "2: 4 bytes"]
    _4_BYTE = 2,
}
impl From<AHB_MST1_BURST_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_MST1_BURST_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AHB_MST1_BURST_SIZE`"]
pub type AHB_MST1_BURST_SIZE_R = crate::R<u8, AHB_MST1_BURST_SIZE_A>;
impl AHB_MST1_BURST_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AHB_MST1_BURST_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            6 => Val(AHB_MST1_BURST_SIZE_A::_64_BYTE),
            5 => Val(AHB_MST1_BURST_SIZE_A::_32_BYTE),
            4 => Val(AHB_MST1_BURST_SIZE_A::_16_BYTE),
            3 => Val(AHB_MST1_BURST_SIZE_A::_8_BYTE),
            2 => Val(AHB_MST1_BURST_SIZE_A::_4_BYTE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZE_A::_4_BYTE
    }
}
#[doc = "Write proxy for field `AHB_MST1_BURST_SIZE`"]
pub struct AHB_MST1_BURST_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BURST_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_BURST_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_64_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_32_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_16_BYTE)
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_8_BYTE)
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_4_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "11:11\\]
Idle insertion between consecutive burst transfers on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_IDLE_EN_A {
    #[doc = "1: Idle transfer insertion enabled"]
    IDLE = 1,
    #[doc = "0: Do not insert idle transfers."]
    NO_IDLE = 0,
}
impl From<AHB_MST1_IDLE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_IDLE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHB_MST1_IDLE_EN`"]
pub type AHB_MST1_IDLE_EN_R = crate::R<bool, AHB_MST1_IDLE_EN_A>;
impl AHB_MST1_IDLE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_IDLE_EN_A {
        match self.bits {
            true => AHB_MST1_IDLE_EN_A::IDLE,
            false => AHB_MST1_IDLE_EN_A::NO_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == AHB_MST1_IDLE_EN_A::IDLE
    }
    #[doc = "Checks if the value of the field is `NO_IDLE`"]
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == AHB_MST1_IDLE_EN_A::NO_IDLE
    }
}
#[doc = "Write proxy for field `AHB_MST1_IDLE_EN`"]
pub struct AHB_MST1_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_IDLE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_IDLE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle transfer insertion enabled"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_EN_A::IDLE)
    }
    #[doc = "Do not insert idle transfers."]
    #[inline(always)]
    pub fn no_idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_EN_A::NO_IDLE)
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
#[doc = "10:10\\]
Burst length type of AHB transfer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_INCR_EN_A {
    #[doc = "1: Fixed length bursts or single transfers"]
    SPECIFIED = 1,
    #[doc = "0: Unspecified length burst transfers"]
    UNSPECIFIED = 0,
}
impl From<AHB_MST1_INCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_INCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHB_MST1_INCR_EN`"]
pub type AHB_MST1_INCR_EN_R = crate::R<bool, AHB_MST1_INCR_EN_A>;
impl AHB_MST1_INCR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_INCR_EN_A {
        match self.bits {
            true => AHB_MST1_INCR_EN_A::SPECIFIED,
            false => AHB_MST1_INCR_EN_A::UNSPECIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `SPECIFIED`"]
    #[inline(always)]
    pub fn is_specified(&self) -> bool {
        *self == AHB_MST1_INCR_EN_A::SPECIFIED
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == AHB_MST1_INCR_EN_A::UNSPECIFIED
    }
}
#[doc = "Write proxy for field `AHB_MST1_INCR_EN`"]
pub struct AHB_MST1_INCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_INCR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_INCR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fixed length bursts or single transfers"]
    #[inline(always)]
    pub fn specified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_EN_A::SPECIFIED)
    }
    #[doc = "Unspecified length burst transfers"]
    #[inline(always)]
    pub fn unspecified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_EN_A::UNSPECIFIED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "9:9\\]
Locked transform on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_LOCK_EN_A {
    #[doc = "1: Transfers are locked"]
    LOCKED = 1,
    #[doc = "0: Transfers are not locked"]
    NOT_LOCKED = 0,
}
impl From<AHB_MST1_LOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_LOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHB_MST1_LOCK_EN`"]
pub type AHB_MST1_LOCK_EN_R = crate::R<bool, AHB_MST1_LOCK_EN_A>;
impl AHB_MST1_LOCK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_LOCK_EN_A {
        match self.bits {
            true => AHB_MST1_LOCK_EN_A::LOCKED,
            false => AHB_MST1_LOCK_EN_A::NOT_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == AHB_MST1_LOCK_EN_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == AHB_MST1_LOCK_EN_A::NOT_LOCKED
    }
}
#[doc = "Write proxy for field `AHB_MST1_LOCK_EN`"]
pub struct AHB_MST1_LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_LOCK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_LOCK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfers are locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_EN_A::LOCKED)
    }
    #[doc = "Transfers are not locked"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_EN_A::NOT_LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "8:8\\]
Endianess for the AHB master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_BIGEND_A {
    #[doc = "1: Big Endian"]
    BIG_ENDIAN = 1,
    #[doc = "0: Little Endian"]
    LITTLE_ENDIAN = 0,
}
impl From<AHB_MST1_BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHB_MST1_BIGEND`"]
pub type AHB_MST1_BIGEND_R = crate::R<bool, AHB_MST1_BIGEND_A>;
impl AHB_MST1_BIGEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_BIGEND_A {
        match self.bits {
            true => AHB_MST1_BIGEND_A::BIG_ENDIAN,
            false => AHB_MST1_BIGEND_A::LITTLE_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN`"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == AHB_MST1_BIGEND_A::BIG_ENDIAN
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN`"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == AHB_MST1_BIGEND_A::LITTLE_ENDIAN
    }
}
#[doc = "Write proxy for field `AHB_MST1_BIGEND`"]
pub struct AHB_MST1_BIGEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BIGEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_BIGEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGEND_A::BIG_ENDIAN)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGEND_A::LITTLE_ENDIAN)
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
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZE_R {
        AHB_MST1_BURST_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_EN_R {
        AHB_MST1_IDLE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_EN_R {
        AHB_MST1_INCR_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_EN_R {
        AHB_MST1_LOCK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGEND_R {
        AHB_MST1_BIGEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&mut self) -> AHB_MST1_BURST_SIZE_W {
        AHB_MST1_BURST_SIZE_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Idle insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&mut self) -> AHB_MST1_IDLE_EN_W {
        AHB_MST1_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&mut self) -> AHB_MST1_INCR_EN_W {
        AHB_MST1_INCR_EN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&mut self) -> AHB_MST1_LOCK_EN_W {
        AHB_MST1_LOCK_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&mut self) -> AHB_MST1_BIGEND_W {
        AHB_MST1_BIGEND_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
