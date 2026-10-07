#[doc = "Register `CNTAR` reader"]
pub type R = crate::R<CntarSpec>;
#[doc = "Register `CNTAR` writer"]
pub type W = crate::W<CntarSpec>;
#[doc = "Field `CNTA` reader - desc CNTA"]
pub type CntaR = crate::FieldReader<u16>;
#[doc = "Field `CNTA` writer - desc CNTA"]
pub type CntaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CNTA"]
    #[inline(always)]
    pub fn cnta(&self) -> CntaR {
        CntaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CNTA"]
    #[inline(always)]
    pub fn cnta(&mut self) -> CntaW<'_, CntarSpec> {
        CntaW::new(self, 0)
    }
}
#[doc = "desc CNTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntarSpec;
impl crate::RegisterSpec for CntarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntar::R`](R) reader structure"]
impl crate::Readable for CntarSpec {}
#[doc = "`write(|w| ..)` method takes [`cntar::W`](W) writer structure"]
impl crate::Writable for CntarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTAR to value 0"]
impl crate::Resettable for CntarSpec {}
