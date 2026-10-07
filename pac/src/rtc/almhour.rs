#[doc = "Register `ALMHOUR` reader"]
pub type R = crate::R<AlmhourSpec>;
#[doc = "Register `ALMHOUR` writer"]
pub type W = crate::W<AlmhourSpec>;
#[doc = "Field `ALMHOURU` reader - desc ALMHOURU"]
pub type AlmhouruR = crate::FieldReader;
#[doc = "Field `ALMHOURU` writer - desc ALMHOURU"]
pub type AlmhouruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALMHOURD` reader - desc ALMHOURD"]
pub type AlmhourdR = crate::FieldReader;
#[doc = "Field `ALMHOURD` writer - desc ALMHOURD"]
pub type AlmhourdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - desc ALMHOURU"]
    #[inline(always)]
    pub fn almhouru(&self) -> AlmhouruR {
        AlmhouruR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - desc ALMHOURD"]
    #[inline(always)]
    pub fn almhourd(&self) -> AlmhourdR {
        AlmhourdR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ALMHOURU"]
    #[inline(always)]
    pub fn almhouru(&mut self) -> AlmhouruW<'_, AlmhourSpec> {
        AlmhouruW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc ALMHOURD"]
    #[inline(always)]
    pub fn almhourd(&mut self) -> AlmhourdW<'_, AlmhourSpec> {
        AlmhourdW::new(self, 4)
    }
}
#[doc = "desc ALMHOUR\n\nYou can [`read`](crate::Reg::read) this register and get [`almhour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almhour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlmhourSpec;
impl crate::RegisterSpec for AlmhourSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`almhour::R`](R) reader structure"]
impl crate::Readable for AlmhourSpec {}
#[doc = "`write(|w| ..)` method takes [`almhour::W`](W) writer structure"]
impl crate::Writable for AlmhourSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALMHOUR to value 0"]
impl crate::Resettable for AlmhourSpec {}
