#[doc = "Register `bmx_cfg6` reader"]
pub type R = crate::R<BMX_CFG6_SPEC>;
#[doc = "Register `bmx_cfg6` writer"]
pub type W = crate::W<BMX_CFG6_SPEC>;
#[doc = "Field `sts_mcu_berr_addr` reader - "]
pub type STS_MCU_BERR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `sts_mcu_berr_addr` writer - "]
pub type STS_MCU_BERR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_mcu_berr_addr(&self) -> STS_MCU_BERR_ADDR_R {
        STS_MCU_BERR_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu_berr_addr(&mut self) -> STS_MCU_BERR_ADDR_W<BMX_CFG6_SPEC> {
        STS_MCU_BERR_ADDR_W::new(self, 0)
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
#[doc = "bmx_cfg6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMX_CFG6_SPEC;
impl crate::RegisterSpec for BMX_CFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg6::R`](R) reader structure"]
impl crate::Readable for BMX_CFG6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg6::W`](W) writer structure"]
impl crate::Writable for BMX_CFG6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg6 to value 0"]
impl crate::Resettable for BMX_CFG6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
