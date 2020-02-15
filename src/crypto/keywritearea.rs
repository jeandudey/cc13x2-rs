#[doc = "Reader of register KEYWRITEAREA"]
pub type R = crate::R<u32, super::KEYWRITEAREA>;
#[doc = "Writer for register KEYWRITEAREA"]
pub type W = crate::W<u32, super::KEYWRITEAREA>;
#[doc = "Register KEYWRITEAREA `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYWRITEAREA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "7:7\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA7_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA7_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA7`"]
pub type RAM_AREA7_R = crate::R<bool, RAM_AREA7_A>;
impl RAM_AREA7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA7_A {
        match self.bits {
            true => RAM_AREA7_A::SEL,
            false => RAM_AREA7_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA7_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA7_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA7`"]
pub struct RAM_AREA7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA7_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA7_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "6:6\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA6_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA6_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA6`"]
pub type RAM_AREA6_R = crate::R<bool, RAM_AREA6_A>;
impl RAM_AREA6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA6_A {
        match self.bits {
            true => RAM_AREA6_A::SEL,
            false => RAM_AREA6_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA6_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA6_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA6`"]
pub struct RAM_AREA6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA6_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA6_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "5:5\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA5_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA5_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA5`"]
pub type RAM_AREA5_R = crate::R<bool, RAM_AREA5_A>;
impl RAM_AREA5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA5_A {
        match self.bits {
            true => RAM_AREA5_A::SEL,
            false => RAM_AREA5_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA5_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA5_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA5`"]
pub struct RAM_AREA5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA5_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA5_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "4:4\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA4_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA4_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA4`"]
pub type RAM_AREA4_R = crate::R<bool, RAM_AREA4_A>;
impl RAM_AREA4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA4_A {
        match self.bits {
            true => RAM_AREA4_A::SEL,
            false => RAM_AREA4_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA4_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA4_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA4`"]
pub struct RAM_AREA4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA4_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA4_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "3:3\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA3_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA3_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA3`"]
pub type RAM_AREA3_R = crate::R<bool, RAM_AREA3_A>;
impl RAM_AREA3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA3_A {
        match self.bits {
            true => RAM_AREA3_A::SEL,
            false => RAM_AREA3_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA3_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA3_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA3`"]
pub struct RAM_AREA3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA3_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA3_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "2:2\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA2_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA2_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA2`"]
pub type RAM_AREA2_R = crate::R<bool, RAM_AREA2_A>;
impl RAM_AREA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA2_A {
        match self.bits {
            true => RAM_AREA2_A::SEL,
            false => RAM_AREA2_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA2_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA2_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA2`"]
pub struct RAM_AREA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA2_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA2_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA1_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA1`"]
pub type RAM_AREA1_R = crate::R<bool, RAM_AREA1_A>;
impl RAM_AREA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA1_A {
        match self.bits {
            true => RAM_AREA1_A::SEL,
            false => RAM_AREA1_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA1_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA1_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA1`"]
pub struct RAM_AREA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA1_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA1_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "0:0\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA0_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AREA0`"]
pub type RAM_AREA0_R = crate::R<bool, RAM_AREA0_A>;
impl RAM_AREA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA0_A {
        match self.bits {
            true => RAM_AREA0_A::SEL,
            false => RAM_AREA0_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA0_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA0_A::NOT_SEL
    }
}
#[doc = "Write proxy for field `RAM_AREA0`"]
pub struct RAM_AREA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA0_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA0_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RAM_AREA7_R {
        RAM_AREA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RAM_AREA6_R {
        RAM_AREA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RAM_AREA5_R {
        RAM_AREA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RAM_AREA4_R {
        RAM_AREA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RAM_AREA3_R {
        RAM_AREA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RAM_AREA2_R {
        RAM_AREA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RAM_AREA1_R {
        RAM_AREA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RAM_AREA0_R {
        RAM_AREA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&mut self) -> RAM_AREA7_W {
        RAM_AREA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&mut self) -> RAM_AREA6_W {
        RAM_AREA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&mut self) -> RAM_AREA5_W {
        RAM_AREA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&mut self) -> RAM_AREA4_W {
        RAM_AREA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&mut self) -> RAM_AREA3_W {
        RAM_AREA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&mut self) -> RAM_AREA2_W {
        RAM_AREA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&mut self) -> RAM_AREA1_W {
        RAM_AREA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&mut self) -> RAM_AREA0_W {
        RAM_AREA0_W { w: self }
    }
}
