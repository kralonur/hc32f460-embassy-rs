#[doc = "Register `CHSELRB` reader"]
pub type R = crate::R<ChselrbSpec>;
#[doc = "Register `CHSELRB` writer"]
pub type W = crate::W<ChselrbSpec>;
#[doc = "Field `CHSELB` reader - desc CHSELB"]
pub type ChselbR = crate::FieldReader<u32>;
#[doc = "Field `CHSELB` writer - desc CHSELB"]
pub type ChselbW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - desc CHSELB"]
    #[inline(always)]
    pub fn chselb(&self) -> ChselbR {
        ChselbR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - desc CHSELB"]
    #[inline(always)]
    pub fn chselb(&mut self) -> ChselbW<'_, ChselrbSpec> {
        ChselbW::new(self, 0)
    }
}
#[doc = "desc CHSELRB\n\nYou can [`read`](crate::Reg::read) this register and get [`chselrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChselrbSpec;
impl crate::RegisterSpec for ChselrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselrb::R`](R) reader structure"]
impl crate::Readable for ChselrbSpec {}
#[doc = "`write(|w| ..)` method takes [`chselrb::W`](W) writer structure"]
impl crate::Writable for ChselrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSELRB to value 0"]
impl crate::Resettable for ChselrbSpec {}
