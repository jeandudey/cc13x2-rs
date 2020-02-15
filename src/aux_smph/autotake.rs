#[doc = "Reader of register AUTOTAKE"]
pub type R = crate::R<u32, super::AUTOTAKE>;
#[doc = "Writer for register AUTOTAKE"]
pub type W = crate::W<u32, super::AUTOTAKE>;
#[doc = "Register AUTOTAKE `reset()`'s with value 0"]
impl crate::ResetValue for super::AUTOTAKE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `SMPH_ID`"]
pub type SMPH_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPH_ID`"]
pub struct SMPH_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    pub fn smph_id(&self) -> SMPH_ID_R {
        SMPH_ID_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    pub fn smph_id(&mut self) -> SMPH_ID_W {
        SMPH_ID_W { w: self }
    }
}
