#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:0\\]
TDC commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "3: Force TDC state machine back to IDLE state.\n\nNever write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    ABORT = 3,
    #[doc = "2: Asynchronous counter start.\n\nThe counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    RUN = 2,
    #[doc = "1: Synchronous counter start.\n\nThe counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    RUN_SYNC_START = 1,
    #[doc = "0: Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. \n\nThis is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    CLR_RESULT = 0,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, CMD_A>;
impl CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_A {
        match self.bits {
            3 => CMD_A::ABORT,
            2 => CMD_A::RUN,
            1 => CMD_A::RUN_SYNC_START,
            0 => CMD_A::CLR_RESULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMD_A::ABORT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == CMD_A::RUN
    }
    #[doc = "Checks if the value of the field is `RUN_SYNC_START`"]
    #[inline(always)]
    pub fn is_run_sync_start(&self) -> bool {
        *self == CMD_A::RUN_SYNC_START
    }
    #[doc = "Checks if the value of the field is `CLR_RESULT`"]
    #[inline(always)]
    pub fn is_clr_result(&self) -> bool {
        *self == CMD_A::CLR_RESULT
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMD_A::ABORT)
    }
    #[doc = "Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CMD_A::RUN)
    }
    #[doc = "Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    #[inline(always)]
    pub fn run_sync_start(self) -> &'a mut W {
        self.variant(CMD_A::RUN_SYNC_START)
    }
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    #[inline(always)]
    pub fn clr_result(self) -> &'a mut W {
        self.variant(CMD_A::CLR_RESULT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
TDC commands."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
TDC commands."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
