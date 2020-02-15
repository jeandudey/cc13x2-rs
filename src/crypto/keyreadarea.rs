#[doc = "Reader of register KEYREADAREA"]
pub type R = crate::R<u32, super::KEYREADAREA>;
#[doc = "Writer for register KEYREADAREA"]
pub type W = crate::W<u32, super::KEYREADAREA>;
#[doc = "Register KEYREADAREA `reset()`'s with value 0x08"]
impl crate::ResetValue for super::KEYREADAREA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
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
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 4)) | (((value as u32) & 0x07ff_ffff) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAM_AREA_A {
    #[doc = "8: No RAM"]
    NO_RAM = 8,
    #[doc = "7: RAM Area 7"]
    RAM_AREA7 = 7,
    #[doc = "6: RAM Area 6"]
    RAM_AREA6 = 6,
    #[doc = "5: RAM Area 5"]
    RAM_AREA5 = 5,
    #[doc = "4: RAM Area 4"]
    RAM_AREA4 = 4,
    #[doc = "3: RAM Area 3"]
    RAM_AREA3 = 3,
    #[doc = "2: RAM Area 2"]
    RAM_AREA2 = 2,
    #[doc = "1: RAM Area 1"]
    RAM_AREA1 = 1,
    #[doc = "0: RAM Area 0"]
    RAM_AREA0 = 0,
}
impl From<RAM_AREA_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM_AREA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAM_AREA`"]
pub type RAM_AREA_R = crate::R<u8, RAM_AREA_A>;
impl RAM_AREA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM_AREA_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(RAM_AREA_A::NO_RAM),
            7 => Val(RAM_AREA_A::RAM_AREA7),
            6 => Val(RAM_AREA_A::RAM_AREA6),
            5 => Val(RAM_AREA_A::RAM_AREA5),
            4 => Val(RAM_AREA_A::RAM_AREA4),
            3 => Val(RAM_AREA_A::RAM_AREA3),
            2 => Val(RAM_AREA_A::RAM_AREA2),
            1 => Val(RAM_AREA_A::RAM_AREA1),
            0 => Val(RAM_AREA_A::RAM_AREA0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_RAM`"]
    #[inline(always)]
    pub fn is_no_ram(&self) -> bool {
        *self == RAM_AREA_A::NO_RAM
    }
    #[doc = "Checks if the value of the field is `RAM_AREA7`"]
    #[inline(always)]
    pub fn is_ram_area7(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA7
    }
    #[doc = "Checks if the value of the field is `RAM_AREA6`"]
    #[inline(always)]
    pub fn is_ram_area6(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA6
    }
    #[doc = "Checks if the value of the field is `RAM_AREA5`"]
    #[inline(always)]
    pub fn is_ram_area5(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA5
    }
    #[doc = "Checks if the value of the field is `RAM_AREA4`"]
    #[inline(always)]
    pub fn is_ram_area4(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA4
    }
    #[doc = "Checks if the value of the field is `RAM_AREA3`"]
    #[inline(always)]
    pub fn is_ram_area3(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA3
    }
    #[doc = "Checks if the value of the field is `RAM_AREA2`"]
    #[inline(always)]
    pub fn is_ram_area2(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA2
    }
    #[doc = "Checks if the value of the field is `RAM_AREA1`"]
    #[inline(always)]
    pub fn is_ram_area1(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA1
    }
    #[doc = "Checks if the value of the field is `RAM_AREA0`"]
    #[inline(always)]
    pub fn is_ram_area0(&self) -> bool {
        *self == RAM_AREA_A::RAM_AREA0
    }
}
#[doc = "Write proxy for field `RAM_AREA`"]
pub struct RAM_AREA_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No RAM"]
    #[inline(always)]
    pub fn no_ram(self) -> &'a mut W {
        self.variant(RAM_AREA_A::NO_RAM)
    }
    #[doc = "RAM Area 7"]
    #[inline(always)]
    pub fn ram_area7(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA7)
    }
    #[doc = "RAM Area 6"]
    #[inline(always)]
    pub fn ram_area6(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA6)
    }
    #[doc = "RAM Area 5"]
    #[inline(always)]
    pub fn ram_area5(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA5)
    }
    #[doc = "RAM Area 4"]
    #[inline(always)]
    pub fn ram_area4(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA4)
    }
    #[doc = "RAM Area 3"]
    #[inline(always)]
    pub fn ram_area3(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA3)
    }
    #[doc = "RAM Area 2"]
    #[inline(always)]
    pub fn ram_area2(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA2)
    }
    #[doc = "RAM Area 1"]
    #[inline(always)]
    pub fn ram_area1(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA1)
    }
    #[doc = "RAM Area 0"]
    #[inline(always)]
    pub fn ram_area0(self) -> &'a mut W {
        self.variant(RAM_AREA_A::RAM_AREA0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&self) -> RAM_AREA_R {
        RAM_AREA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&mut self) -> RAM_AREA_W {
        RAM_AREA_W { w: self }
    }
}
