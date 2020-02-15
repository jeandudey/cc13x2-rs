#[doc = "Reader of register FUNCTION1"]
pub type R = crate::R<u32, super::FUNCTION1>;
#[doc = "Writer for register FUNCTION1"]
pub type W = crate::W<u32, super::FUNCTION1>;
#[doc = "Register FUNCTION1 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::FUNCTION1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `RESERVED25`"]
pub type RESERVED25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `MATCHED`"]
pub type MATCHED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MATCHED`"]
pub struct MATCHED_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED20`"]
pub type RESERVED20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DATAVADDR1`"]
pub type DATAVADDR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVADDR1`"]
pub struct DATAVADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATAVADDR0`"]
pub type DATAVADDR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVADDR0`"]
pub struct DATAVADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DATAVSIZE`"]
pub type DATAVSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVSIZE`"]
pub struct DATAVSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `LNK1ENA`"]
pub type LNK1ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNK1ENA`"]
pub struct LNK1ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LNK1ENA_W<'a> {
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
#[doc = "Reader of field `DATAVMATCH`"]
pub type DATAVMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAVMATCH`"]
pub struct DATAVMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVMATCH_W<'a> {
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
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `EMITRANGE`"]
pub type EMITRANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMITRANGE`"]
pub struct EMITRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMITRANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FUNCTION`"]
pub type FUNCTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNCTION`"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    pub fn datavaddr1(&self) -> DATAVADDR1_R {
        DATAVADDR1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    pub fn datavaddr0(&self) -> DATAVADDR0_R {
        DATAVADDR0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline(always)]
    pub fn lnk1ena(&self) -> LNK1ENA_R {
        LNK1ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    pub fn datavmatch(&self) -> DATAVMATCH_R {
        DATAVMATCH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub fn emitrange(&self) -> EMITRANGE_R {
        EMITRANGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub fn matched(&mut self) -> MATCHED_W {
        MATCHED_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    pub fn datavaddr1(&mut self) -> DATAVADDR1_W {
        DATAVADDR1_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    pub fn datavaddr0(&mut self) -> DATAVADDR0_W {
        DATAVADDR0_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    pub fn datavsize(&mut self) -> DATAVSIZE_W {
        DATAVSIZE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline(always)]
    pub fn lnk1ena(&mut self) -> LNK1ENA_W {
        LNK1ENA_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    pub fn datavmatch(&mut self) -> DATAVMATCH_W {
        DATAVMATCH_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub fn emitrange(&mut self) -> EMITRANGE_W {
        EMITRANGE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
}
