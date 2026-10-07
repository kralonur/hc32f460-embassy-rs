#[doc = "Register `FAPRT` reader"]
pub type R = crate::R<FaprtSpec>;
#[doc = "Register `FAPRT` writer"]
pub type W = crate::W<FaprtSpec>;
#[doc = "Field `FAPRT` reader - desc FAPRT"]
pub type FaprtR = crate::FieldReader<u16>;
#[doc = "Field `FAPRT` writer - desc FAPRT"]
pub type FaprtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc FAPRT"]
    #[inline(always)]
    pub fn faprt(&self) -> FaprtR {
        FaprtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc FAPRT"]
    #[inline(always)]
    pub fn faprt(&mut self) -> FaprtW<'_, FaprtSpec> {
        FaprtW::new(self, 0)
    }
}
#[doc = "desc FAPRT\n\nYou can [`read`](crate::Reg::read) this register and get [`faprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaprtSpec;
impl crate::RegisterSpec for FaprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faprt::R`](R) reader structure"]
impl crate::Readable for FaprtSpec {}
#[doc = "`write(|w| ..)` method takes [`faprt::W`](W) writer structure"]
impl crate::Writable for FaprtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FAPRT to value 0"]
impl crate::Resettable for FaprtSpec {}
