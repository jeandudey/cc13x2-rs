#[doc = "Reader of register FSM_ACC_PP"]
pub type R = crate::R<u32, super::FSM_ACC_PP>;
#[doc = "Writer for register FSM_ACC_PP"]
pub type W = crate::W<u32, super::FSM_ACC_PP>;
#[doc = "Register FSM_ACC_PP `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_ACC_PP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSM_ACC_PP`"]
pub type FSM_ACC_PP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_ACC_PP`"]
pub struct FSM_ACC_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_ACC_PP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_acc_pp(&self) -> FSM_ACC_PP_R {
        FSM_ACC_PP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_acc_pp(&mut self) -> FSM_ACC_PP_W {
        FSM_ACC_PP_W { w: self }
    }
}
