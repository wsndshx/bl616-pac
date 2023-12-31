#[doc = "Register `interrupt_mask` reader"]
pub type R = crate::R<INTERRUPT_MASK_SPEC>;
#[doc = "Register `interrupt_mask` writer"]
pub type W = crate::W<INTERRUPT_MASK_SPEC>;
#[doc = "Field `transmit_transfer` reader - Transmit transfer finish signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as TRANSMIT_TRANSFER_R;
#[doc = "Field `receive_transfer` reader - Receive transfer finish signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_TRANSFER_R;
#[doc = "Field `transmit_fifo_ready` reader - Transmit FIFO ready signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as TRANSMIT_FIFO_READY_R;
#[doc = "Field `receive_fifo_ready` reader - Receive FIFO ready signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_FIFO_READY_R;
#[doc = "Field `receive_timeout` reader - Receive timed-out interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_TIMEOUT_R;
#[doc = "Field `receive_parity` reader - Receive parity check failure interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_PARITY_R;
#[doc = "Field `transmit_fifo_error` reader - Transmit FIFO overflow or underflow interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as TRANSMIT_FIFO_ERROR_R;
#[doc = "Field `receive_fifo_error` reader - Receive FIFO overflow or underflow interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_FIFO_ERROR_R;
#[doc = "Field `receive_sync_error` reader - Receive LIN mode synchronization field error interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_SYNC_ERROR_R;
#[doc = "Field `receive_byte_count` reader - Receive byte count reached interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as RECEIVE_BYTE_COUNT_R;
#[doc = "Field `auto_baudrate_start_bit` reader - Receive auto baudrate detection finished using start bit interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_R as AUTO_BAUDRATE_START_BIT_R;
#[doc = "Field `transmit_transfer` writer - Transmit transfer finish signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_TRANSFER_W;
#[doc = "Field `receive_transfer` writer - Receive transfer finish signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_TRANSFER_W;
#[doc = "Field `transmit_fifo_ready` writer - Transmit FIFO ready signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_FIFO_READY_W;
#[doc = "Field `receive_fifo_ready` writer - Receive FIFO ready signal interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_FIFO_READY_W;
#[doc = "Field `receive_timeout` writer - Receive timed-out interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_TIMEOUT_W;
#[doc = "Field `receive_parity` writer - Receive parity check failure interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_PARITY_W;
#[doc = "Field `transmit_fifo_error` writer - Transmit FIFO overflow or underflow interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as TRANSMIT_FIFO_ERROR_W;
#[doc = "Field `receive_fifo_error` writer - Receive FIFO overflow or underflow interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_FIFO_ERROR_W;
#[doc = "Field `receive_sync_error` writer - Receive LIN mode synchronization field error interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_SYNC_ERROR_W;
#[doc = "Field `receive_byte_count` writer - Receive byte count reached interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as RECEIVE_BYTE_COUNT_W;
#[doc = "Field `auto_baudrate_start_bit` writer - Receive auto baudrate detection finished using start bit interrupt mask"]
pub use AUTO_BAUDRATE_FIVE_FIVE_W as AUTO_BAUDRATE_START_BIT_W;
#[doc = "Field `auto_baudrate_five_five` reader - Receive auto baudrate detection finished using 0x55 occurred"]
pub type AUTO_BAUDRATE_FIVE_FIVE_R = crate::BitReader<INTERRUPT_MASK_A>;
#[doc = "Receive auto baudrate detection finished using 0x55 occurred\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK_A {
    #[doc = "1: Mask interrupt"]
    MASK = 1,
    #[doc = "0: Unmask interrupt"]
    UNMASK = 0,
}
impl From<INTERRUPT_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_BAUDRATE_FIVE_FIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK_A {
        match self.bits {
            true => INTERRUPT_MASK_A::MASK,
            false => INTERRUPT_MASK_A::UNMASK,
        }
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == INTERRUPT_MASK_A::MASK
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == INTERRUPT_MASK_A::UNMASK
    }
}
#[doc = "Field `auto_baudrate_five_five` writer - Receive auto baudrate detection finished using 0x55 occurred"]
pub type AUTO_BAUDRATE_FIVE_FIVE_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK_A>;
impl<'a, REG> AUTO_BAUDRATE_FIVE_FIVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK_A::MASK)
    }
    #[doc = "Unmask interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK_A::UNMASK)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit transfer finish signal interrupt mask"]
    #[inline(always)]
    pub fn transmit_transfer(&self) -> TRANSMIT_TRANSFER_R {
        TRANSMIT_TRANSFER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive transfer finish signal interrupt mask"]
    #[inline(always)]
    pub fn receive_transfer(&self) -> RECEIVE_TRANSFER_R {
        RECEIVE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_ready(&self) -> TRANSMIT_FIFO_READY_R {
        TRANSMIT_FIFO_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO ready signal interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_ready(&self) -> RECEIVE_FIFO_READY_R {
        RECEIVE_FIFO_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive timed-out interrupt mask"]
    #[inline(always)]
    pub fn receive_timeout(&self) -> RECEIVE_TIMEOUT_R {
        RECEIVE_TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive parity check failure interrupt mask"]
    #[inline(always)]
    pub fn receive_parity(&self) -> RECEIVE_PARITY_R {
        RECEIVE_PARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    pub fn transmit_fifo_error(&self) -> TRANSMIT_FIFO_ERROR_R {
        TRANSMIT_FIFO_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    pub fn receive_fifo_error(&self) -> RECEIVE_FIFO_ERROR_R {
        RECEIVE_FIFO_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive LIN mode synchronization field error interrupt mask"]
    #[inline(always)]
    pub fn receive_sync_error(&self) -> RECEIVE_SYNC_ERROR_R {
        RECEIVE_SYNC_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive byte count reached interrupt mask"]
    #[inline(always)]
    pub fn receive_byte_count(&self) -> RECEIVE_BYTE_COUNT_R {
        RECEIVE_BYTE_COUNT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive auto baudrate detection finished using start bit interrupt mask"]
    #[inline(always)]
    pub fn auto_baudrate_start_bit(&self) -> AUTO_BAUDRATE_START_BIT_R {
        AUTO_BAUDRATE_START_BIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive auto baudrate detection finished using 0x55 occurred"]
    #[inline(always)]
    pub fn auto_baudrate_five_five(&self) -> AUTO_BAUDRATE_FIVE_FIVE_R {
        AUTO_BAUDRATE_FIVE_FIVE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit transfer finish signal interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_transfer(&mut self) -> TRANSMIT_TRANSFER_W<INTERRUPT_MASK_SPEC> {
        TRANSMIT_TRANSFER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive transfer finish signal interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_transfer(&mut self) -> RECEIVE_TRANSFER_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_TRANSFER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO ready signal interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_ready(&mut self) -> TRANSMIT_FIFO_READY_W<INTERRUPT_MASK_SPEC> {
        TRANSMIT_FIFO_READY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO ready signal interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_ready(&mut self) -> RECEIVE_FIFO_READY_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_FIFO_READY_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive timed-out interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_timeout(&mut self) -> RECEIVE_TIMEOUT_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_TIMEOUT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive parity check failure interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_parity(&mut self) -> RECEIVE_PARITY_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_PARITY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_error(&mut self) -> TRANSMIT_FIFO_ERROR_W<INTERRUPT_MASK_SPEC> {
        TRANSMIT_FIFO_ERROR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive FIFO overflow or underflow interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_error(&mut self) -> RECEIVE_FIFO_ERROR_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_FIFO_ERROR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive LIN mode synchronization field error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_sync_error(&mut self) -> RECEIVE_SYNC_ERROR_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_SYNC_ERROR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive byte count reached interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn receive_byte_count(&mut self) -> RECEIVE_BYTE_COUNT_W<INTERRUPT_MASK_SPEC> {
        RECEIVE_BYTE_COUNT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive auto baudrate detection finished using start bit interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate_start_bit(&mut self) -> AUTO_BAUDRATE_START_BIT_W<INTERRUPT_MASK_SPEC> {
        AUTO_BAUDRATE_START_BIT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Receive auto baudrate detection finished using 0x55 occurred"]
    #[inline(always)]
    #[must_use]
    pub fn auto_baudrate_five_five(&mut self) -> AUTO_BAUDRATE_FIVE_FIVE_W<INTERRUPT_MASK_SPEC> {
        AUTO_BAUDRATE_FIVE_FIVE_W::new(self, 11)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mask::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mask::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_mask to value 0x0fff"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
