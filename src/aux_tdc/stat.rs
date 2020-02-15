#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0x06"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
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
#[doc = "Reader of field `SAT`"]
pub type SAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAT`"]
pub struct SAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAT_W<'a> {
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
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
#[doc = "5:0\\]
TDC state machine status.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "46: Current state is TDC_FORCESTOP.\nYou wrote ABORT to CTL.CMD to abort the TDC measurement."]
    FORCE_STOP = 46,
    #[doc = "30: Current state is TDC_WAIT_STARTFALL. \nThe fast-counter circuit waits for a falling edge on the start event."]
    START_FALL = 30,
    #[doc = "22: Current state is TDC_STATE_WAIT_CLRCNT_DONE. \nThe state machine waits for fast-counter circuit to finish reset."]
    WAIT_CLR_CNT_DONE = 22,
    #[doc = "15: Current state is TDC_STATE_POR. \nThis is the reset state."]
    POR = 15,
    #[doc = "14: Current state is TDC_STATE_GETRESULTS.\nThe state machine copies the counter value from the fast-counter circuit."]
    GET_RESULT = 14,
    #[doc = "12: Current state is TDC_STATE_WAIT_STOPCNTDOWN.\nThe fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WAIT_STOP_CNTDWN = 12,
    #[doc = "8: Current state is TDC_STATE_WAIT_STOP.\nThe state machine waits for the fast-counter circuit to stop."]
    WAIT_STOP = 8,
    #[doc = "7: Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    CLR_CNT = 7,
    #[doc = "6: Current state is TDC_STATE_IDLE. \nThis is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    IDLE = 6,
    #[doc = "4: Current state is TDC_STATE_WAIT_STARTSTOPCNTEN.\nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START_STOP_CNT_EN = 4,
    #[doc = "0: Current state is TDC_STATE_WAIT_START. \nThe fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START = 0,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            46 => Val(STATE_A::FORCE_STOP),
            30 => Val(STATE_A::START_FALL),
            22 => Val(STATE_A::WAIT_CLR_CNT_DONE),
            15 => Val(STATE_A::POR),
            14 => Val(STATE_A::GET_RESULT),
            12 => Val(STATE_A::WAIT_STOP_CNTDWN),
            8 => Val(STATE_A::WAIT_STOP),
            7 => Val(STATE_A::CLR_CNT),
            6 => Val(STATE_A::IDLE),
            4 => Val(STATE_A::WAIT_START_STOP_CNT_EN),
            0 => Val(STATE_A::WAIT_START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_STOP`"]
    #[inline(always)]
    pub fn is_force_stop(&self) -> bool {
        *self == STATE_A::FORCE_STOP
    }
    #[doc = "Checks if the value of the field is `START_FALL`"]
    #[inline(always)]
    pub fn is_start_fall(&self) -> bool {
        *self == STATE_A::START_FALL
    }
    #[doc = "Checks if the value of the field is `WAIT_CLR_CNT_DONE`"]
    #[inline(always)]
    pub fn is_wait_clr_cnt_done(&self) -> bool {
        *self == STATE_A::WAIT_CLR_CNT_DONE
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == STATE_A::POR
    }
    #[doc = "Checks if the value of the field is `GET_RESULT`"]
    #[inline(always)]
    pub fn is_get_result(&self) -> bool {
        *self == STATE_A::GET_RESULT
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP_CNTDWN`"]
    #[inline(always)]
    pub fn is_wait_stop_cntdwn(&self) -> bool {
        *self == STATE_A::WAIT_STOP_CNTDWN
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP`"]
    #[inline(always)]
    pub fn is_wait_stop(&self) -> bool {
        *self == STATE_A::WAIT_STOP
    }
    #[doc = "Checks if the value of the field is `CLR_CNT`"]
    #[inline(always)]
    pub fn is_clr_cnt(&self) -> bool {
        *self == STATE_A::CLR_CNT
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT_START_STOP_CNT_EN`"]
    #[inline(always)]
    pub fn is_wait_start_stop_cnt_en(&self) -> bool {
        *self == STATE_A::WAIT_START_STOP_CNT_EN
    }
    #[doc = "Checks if the value of the field is `WAIT_START`"]
    #[inline(always)]
    pub fn is_wait_start(&self) -> bool {
        *self == STATE_A::WAIT_START
    }
}
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    #[inline(always)]
    pub fn force_stop(self) -> &'a mut W {
        self.variant(STATE_A::FORCE_STOP)
    }
    #[doc = "Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    #[inline(always)]
    pub fn start_fall(self) -> &'a mut W {
        self.variant(STATE_A::START_FALL)
    }
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    #[inline(always)]
    pub fn wait_clr_cnt_done(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_CLR_CNT_DONE)
    }
    #[doc = "Current state is TDC_STATE_POR. This is the reset state."]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(STATE_A::POR)
    }
    #[doc = "Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    #[inline(always)]
    pub fn get_result(self) -> &'a mut W {
        self.variant(STATE_A::GET_RESULT)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    #[inline(always)]
    pub fn wait_stop_cntdwn(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_STOP_CNTDWN)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    #[inline(always)]
    pub fn wait_stop(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_STOP)
    }
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    #[inline(always)]
    pub fn clr_cnt(self) -> &'a mut W {
        self.variant(STATE_A::CLR_CNT)
    }
    #[doc = "Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(STATE_A::IDLE)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline(always)]
    pub fn wait_start_stop_cnt_en(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_START_STOP_CNT_EN)
    }
    #[doc = "Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline(always)]
    pub fn wait_start(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_START)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
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
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 5:0\\]
TDC state machine status."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x3f) as u8)
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
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline(always)]
    pub fn sat(&mut self) -> SAT_W {
        SAT_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
TDC state machine status."]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
}
