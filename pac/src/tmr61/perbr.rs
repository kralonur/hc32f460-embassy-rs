#[doc = "Register `PERBR` reader"]
pub type R = crate::R<PerbrSpec>;
#[doc = "Register `PERBR` writer"]
pub type W = crate::W<PerbrSpec>;
#[doc = "Field `PERB` reader - desc PERB"]
pub type PerbR = crate::FieldReader<u16>;
#[doc = "Field `PERB` writer - desc PERB"]
pub type PerbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc PERB"]
    #[inline(always)]
    pub fn perb(&self) -> PerbR {
        PerbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PERB"]
    #[inline(always)]
    pub fn perb(&mut self) -> PerbW<'_, PerbrSpec> {
        PerbW::new(self, 0)
    }
}
#[doc = "desc PERBR\n\nYou can [`read`](crate::Reg::read) this register and get [`perbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbrSpec;
impl crate::RegisterSpec for PerbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbr::R`](R) reader structure"]
impl crate::Readable for PerbrSpec {}
#[doc = "`write(|w| ..)` method takes [`perbr::W`](W) writer structure"]
impl crate::Writable for PerbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERBR to value 0xffff"]
impl crate::Resettable for PerbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
