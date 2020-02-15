#[doc = "Reader of register CONFIG_IF_ADC"]
pub type R = crate::R<u32, super::CONFIG_IF_ADC>;
#[doc = "Writer for register CONFIG_IF_ADC"]
pub type W = crate::W<u32, super::CONFIG_IF_ADC>;
#[doc = "Register CONFIG_IF_ADC `reset()`'s with value 0x3460_f400"]
impl crate::ResetValue for super::CONFIG_IF_ADC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3460_f400
    }
}
#[doc = "Reader of field `FF2ADJ`"]
pub type FF2ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FF2ADJ`"]
pub struct FF2ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FF2ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `FF3ADJ`"]
pub type FF3ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FF3ADJ`"]
pub struct FF3ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FF3ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `INT3ADJ`"]
pub type INT3ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT3ADJ`"]
pub struct INT3ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `FF1ADJ`"]
pub type FF1ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FF1ADJ`"]
pub struct FF1ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FF1ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AAFCAP`"]
pub type AAFCAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AAFCAP`"]
pub struct AAFCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> AAFCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `INT2ADJ`"]
pub type INT2ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT2ADJ`"]
pub struct INT2ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `IFDIGLDO_TRIM_OUTPUT`"]
pub type IFDIGLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFDIGLDO_TRIM_OUTPUT`"]
pub struct IFDIGLDO_TRIM_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> IFDIGLDO_TRIM_OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `IFANALDO_TRIM_OUTPUT`"]
pub type IFANALDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFANALDO_TRIM_OUTPUT`"]
pub struct IFANALDO_TRIM_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> IFANALDO_TRIM_OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff2adj(&self) -> FF2ADJ_R {
        FF2ADJ_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff3adj(&self) -> FF3ADJ_R {
        FF3ADJ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int3adj(&self) -> INT3ADJ_R {
        INT3ADJ_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff1adj(&self) -> FF1ADJ_R {
        FF1ADJ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aafcap(&self) -> AAFCAP_R {
        AAFCAP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int2adj(&self) -> INT2ADJ_R {
        INT2ADJ_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifdigldo_trim_output(&self) -> IFDIGLDO_TRIM_OUTPUT_R {
        IFDIGLDO_TRIM_OUTPUT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifanaldo_trim_output(&self) -> IFANALDO_TRIM_OUTPUT_R {
        IFANALDO_TRIM_OUTPUT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff2adj(&mut self) -> FF2ADJ_W {
        FF2ADJ_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff3adj(&mut self) -> FF3ADJ_W {
        FF3ADJ_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int3adj(&mut self) -> INT3ADJ_W {
        INT3ADJ_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ff1adj(&mut self) -> FF1ADJ_W {
        FF1ADJ_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aafcap(&mut self) -> AAFCAP_W {
        AAFCAP_W { w: self }
    }
    #[doc = "Bits 10:13 - 13:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn int2adj(&mut self) -> INT2ADJ_W {
        INT2ADJ_W { w: self }
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifdigldo_trim_output(&mut self) -> IFDIGLDO_TRIM_OUTPUT_W {
        IFDIGLDO_TRIM_OUTPUT_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifanaldo_trim_output(&mut self) -> IFANALDO_TRIM_OUTPUT_W {
        IFANALDO_TRIM_OUTPUT_W { w: self }
    }
}
