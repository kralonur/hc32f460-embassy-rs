#[doc = "Register `PWRC3` reader"]
pub type R = crate::R<Pwrc3Spec>;
#[doc = "Register `PWRC3` writer"]
pub type W = crate::W<Pwrc3Spec>;
#[doc = "Field `PDTS` reader - desc PDTS"]
pub type PdtsR = crate::BitReader;
#[doc = "Field `PDTS` writer - desc PDTS"]
pub type PdtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - desc PDTS"]
    #[inline(always)]
    pub fn pdts(&self) -> PdtsR {
        PdtsR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc PDTS"]
    #[inline(always)]
    pub fn pdts(&mut self) -> PdtsW<'_, Pwrc3Spec> {
        PdtsW::new(self, 2)
    }
}
#[doc = "desc PWRC3\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc3Spec;
impl crate::RegisterSpec for Pwrc3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc3::R`](R) reader structure"]
impl crate::Readable for Pwrc3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc3::W`](W) writer structure"]
impl crate::Writable for Pwrc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC3 to value 0x07"]
impl crate::Resettable for Pwrc3Spec {
    const RESET_VALUE: u8 = 0x07;
}
