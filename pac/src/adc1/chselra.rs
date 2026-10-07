#[doc = "Register `CHSELRA` reader"]
pub type R = crate::R<ChselraSpec>;
#[doc = "Register `CHSELRA` writer"]
pub type W = crate::W<ChselraSpec>;
#[doc = "Field `CHSELA` reader - desc CHSELA"]
pub type ChselaR = crate::FieldReader<u32>;
#[doc = "Field `CHSELA` writer - desc CHSELA"]
pub type ChselaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - desc CHSELA"]
    #[inline(always)]
    pub fn chsela(&self) -> ChselaR {
        ChselaR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - desc CHSELA"]
    #[inline(always)]
    pub fn chsela(&mut self) -> ChselaW<'_, ChselraSpec> {
        ChselaW::new(self, 0)
    }
}
#[doc = "desc CHSELRA\n\nYou can [`read`](crate::Reg::read) this register and get [`chselra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChselraSpec;
impl crate::RegisterSpec for ChselraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselra::R`](R) reader structure"]
impl crate::Readable for ChselraSpec {}
#[doc = "`write(|w| ..)` method takes [`chselra::W`](W) writer structure"]
impl crate::Writable for ChselraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSELRA to value 0"]
impl crate::Resettable for ChselraSpec {}
