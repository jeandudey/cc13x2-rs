#[doc = "Reader of register AESIV"]
pub type R = crate::R<u32, super::AESIV>;
#[doc = "Writer for register AESIV"]
pub type W = crate::W<u32, super::AESIV>;
#[doc = "Register AESIV `reset()`'s with value 0"]
impl crate::ResetValue for super::AESIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_IV`"]
pub type AES_IV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_IV`"]
pub struct AES_IV_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_IV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AES_IV\\[31:0\\]
Initialization vector Used for regular non-ECB modes (CBC/CTR): -\\[127:0\\]
- AES_IV - For regular AES operations (CBC and CTR) these registers must be written with a new 128-bit IV. After an operation, these registers contain the latest 128-bit result IV, generated by the EIP-120t. If CTR mode is selected, this value is incremented with 0x1: After first use - When a new data block is submitted to the engine For GCM: -\\[127:0\\]
- AES_IV - For GCM operations, these registers must be written with a new 128-bit IV. After an operation, these registers contain the updated 128-bit result IV, generated by the EIP-120t. Note that bits \\[127:96\\]
of the IV represent the initial counter value (which is 1 for GCM) and must therefore be initialized to 0x01000000. This value is incremented with 0x1: After first use - When a new data block is submitted to the engine. For CCM: -\\[127:0\\]
- A0: For CCM this field must be written with value A0, this value is the concatenation of: A0-flags (5-bits of 0 and 3-bits 'L'), Nonce and counter value. 'L' must be a copy from the 'L' value of the AES_CTRL register. This 'L' indicates the width of the Nonce and counter. The loaded counter must be initialized to 0. The total width of A0 is 128-bit. For CBC-MAC: -\\[127:0\\]
- Zeroes - For CBC-MAC this register must be written with 0s at the start of each operation. After an operation, these registers contain the 128-bit TAG output, generated by the EIP-120t."]
    #[inline(always)]
    pub fn aes_iv(&self) -> AES_IV_R {
        AES_IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AES_IV\\[31:0\\]
Initialization vector Used for regular non-ECB modes (CBC/CTR): -\\[127:0\\]
- AES_IV - For regular AES operations (CBC and CTR) these registers must be written with a new 128-bit IV. After an operation, these registers contain the latest 128-bit result IV, generated by the EIP-120t. If CTR mode is selected, this value is incremented with 0x1: After first use - When a new data block is submitted to the engine For GCM: -\\[127:0\\]
- AES_IV - For GCM operations, these registers must be written with a new 128-bit IV. After an operation, these registers contain the updated 128-bit result IV, generated by the EIP-120t. Note that bits \\[127:96\\]
of the IV represent the initial counter value (which is 1 for GCM) and must therefore be initialized to 0x01000000. This value is incremented with 0x1: After first use - When a new data block is submitted to the engine. For CCM: -\\[127:0\\]
- A0: For CCM this field must be written with value A0, this value is the concatenation of: A0-flags (5-bits of 0 and 3-bits 'L'), Nonce and counter value. 'L' must be a copy from the 'L' value of the AES_CTRL register. This 'L' indicates the width of the Nonce and counter. The loaded counter must be initialized to 0. The total width of A0 is 128-bit. For CBC-MAC: -\\[127:0\\]
- Zeroes - For CBC-MAC this register must be written with 0s at the start of each operation. After an operation, these registers contain the 128-bit TAG output, generated by the EIP-120t."]
    #[inline(always)]
    pub fn aes_iv(&mut self) -> AES_IV_W {
        AES_IV_W { w: self }
    }
}
