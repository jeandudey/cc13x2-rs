#[doc = "Reader of register HASHMODE"]
pub type R = crate::R<u32, super::HASHMODE>;
#[doc = "Writer for register HASHMODE"]
pub type W = crate::W<u32, super::HASHMODE>;
#[doc = "Register HASHMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED7`"]
pub type RESERVED7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `SHA384_MODE`"]
pub type SHA384_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA384_MODE`"]
pub struct SHA384_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA384_MODE_W<'a> {
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
#[doc = "Reader of field `SHA512_MODE`"]
pub type SHA512_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA512_MODE`"]
pub struct SHA512_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA512_MODE_W<'a> {
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
#[doc = "Reader of field `SHA224_MODE`"]
pub type SHA224_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA224_MODE`"]
pub struct SHA224_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA224_MODE_W<'a> {
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
#[doc = "Reader of field `SHA256_MODE`"]
pub type SHA256_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA256_MODE`"]
pub struct SHA256_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA256_MODE_W<'a> {
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
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `NEW_HASH`"]
pub type NEW_HASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEW_HASH`"]
pub struct NEW_HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> NEW_HASH_W<'a> {
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
    #[doc = "Bits 7:31 - 31:7\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 6 - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
    #[inline(always)]
    pub fn sha384_mode(&self) -> SHA384_MODE_R {
        SHA384_MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
    #[inline(always)]
    pub fn sha512_mode(&self) -> SHA512_MODE_R {
        SHA512_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
    #[inline(always)]
    pub fn sha224_mode(&self) -> SHA224_MODE_R {
        SHA224_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
    #[inline(always)]
    pub fn sha256_mode(&self) -> SHA256_MODE_R {
        SHA256_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&self) -> NEW_HASH_R {
        NEW_HASH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
    #[inline(always)]
    pub fn sha384_mode(&mut self) -> SHA384_MODE_W {
        SHA384_MODE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
    #[inline(always)]
    pub fn sha512_mode(&mut self) -> SHA512_MODE_W {
        SHA512_MODE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
    #[inline(always)]
    pub fn sha224_mode(&mut self) -> SHA224_MODE_W {
        SHA224_MODE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
    #[inline(always)]
    pub fn sha256_mode(&mut self) -> SHA256_MODE_W {
        SHA256_MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&mut self) -> NEW_HASH_W {
        NEW_HASH_W { w: self }
    }
}
