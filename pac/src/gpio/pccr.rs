#[doc = "Register `PCCR` reader"]
pub type R = crate::R<PccrSpec>;
#[doc = "Register `PCCR` writer"]
pub type W = crate::W<PccrSpec>;
#[doc = "Field `BFSEL` reader - desc BFSEL"]
pub type BfselR = crate::FieldReader;
#[doc = "Field `BFSEL` writer - desc BFSEL"]
pub type BfselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDWT` reader - desc RDWT"]
pub type RdwtR = crate::FieldReader;
#[doc = "Field `RDWT` writer - desc RDWT"]
pub type RdwtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - desc BFSEL"]
    #[inline(always)]
    pub fn bfsel(&self) -> BfselR {
        BfselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - desc RDWT"]
    #[inline(always)]
    pub fn rdwt(&self) -> RdwtR {
        RdwtR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc BFSEL"]
    #[inline(always)]
    pub fn bfsel(&mut self) -> BfselW<'_, PccrSpec> {
        BfselW::new(self, 0)
    }
    #[doc = "Bits 14:15 - desc RDWT"]
    #[inline(always)]
    pub fn rdwt(&mut self) -> RdwtW<'_, PccrSpec> {
        RdwtW::new(self, 14)
    }
}
#[doc = "desc PCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PccrSpec;
impl crate::RegisterSpec for PccrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pccr::R`](R) reader structure"]
impl crate::Readable for PccrSpec {}
#[doc = "`write(|w| ..)` method takes [`pccr::W`](W) writer structure"]
impl crate::Writable for PccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCCR to value 0"]
impl crate::Resettable for PccrSpec {}
