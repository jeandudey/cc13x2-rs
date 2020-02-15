#[doc = "Reader of register CCFG_TI_OPTIONS"]
pub type R = crate::R<u32, super::CCFG_TI_OPTIONS>;
#[doc = "Writer for register CCFG_TI_OPTIONS"]
pub type W = crate::W<u32, super::CCFG_TI_OPTIONS>;
#[doc = "Register CCFG_TI_OPTIONS `reset()`'s with value 0xffff_ffc5"]
impl crate::ResetValue for super::CCFG_TI_OPTIONS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffc5
    }
}
#[doc = "Reader of field `TI_FA_ENABLE`"]
pub type TI_FA_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI_FA_ENABLE`"]
pub struct TI_FA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_FA_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    pub fn ti_fa_enable(&self) -> TI_FA_ENABLE_R {
        TI_FA_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    pub fn ti_fa_enable(&mut self) -> TI_FA_ENABLE_W {
        TI_FA_ENABLE_W { w: self }
    }
}
