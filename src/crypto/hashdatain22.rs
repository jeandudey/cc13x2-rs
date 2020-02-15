#[doc = "Reader of register HASHDATAIN22"]
pub type R = crate::R<u32, super::HASHDATAIN22>;
#[doc = "Writer for register HASHDATAIN22"]
pub type W = crate::W<u32, super::HASHDATAIN22>;
#[doc = "Register HASHDATAIN22 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHDATAIN22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HASH_DATA_IN`"]
pub type HASH_DATA_IN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HASH_DATA_IN`"]
pub struct HASH_DATA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_DATA_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DATA_IN\\[735:704\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    pub fn hash_data_in(&self) -> HASH_DATA_IN_R {
        HASH_DATA_IN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HASH_DATA_IN\\[735:704\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when HASHIOBUFCTRL.RFD_IN is 1. If the HASHIOBUFCTRL.RFD_IN is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASHIOBUFCTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    pub fn hash_data_in(&mut self) -> HASH_DATA_IN_W {
        HASH_DATA_IN_W { w: self }
    }
}
