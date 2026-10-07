#[doc = "Register `CCMD` reader"]
pub type R = crate::R<CcmdSpec>;
#[doc = "Register `CCMD` writer"]
pub type W = crate::W<CcmdSpec>;
#[doc = "Field `RIC` reader - desc RIC"]
pub type RicR = crate::FieldReader;
#[doc = "Field `RIC` writer - desc RIC"]
pub type RicW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc RIC"]
    #[inline(always)]
    pub fn ric(&self) -> RicR {
        RicR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc RIC"]
    #[inline(always)]
    pub fn ric(&mut self) -> RicW<'_, CcmdSpec> {
        RicW::new(self, 0)
    }
}
#[doc = "desc CCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcmdSpec;
impl crate::RegisterSpec for CcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmd::R`](R) reader structure"]
impl crate::Readable for CcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmd::W`](W) writer structure"]
impl crate::Writable for CcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMD to value 0"]
impl crate::Resettable for CcmdSpec {}
