#[doc = "Register `CVSPRD` reader"]
pub type R = crate::R<CvsprdSpec>;
#[doc = "Register `CVSPRD` writer"]
pub type W = crate::W<CvsprdSpec>;
#[doc = "Field `PRD` reader - desc PRD"]
pub type PrdR = crate::FieldReader;
#[doc = "Field `PRD` writer - desc PRD"]
pub type PrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc PRD"]
    #[inline(always)]
    pub fn prd(&self) -> PrdR {
        PrdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc PRD"]
    #[inline(always)]
    pub fn prd(&mut self) -> PrdW<'_, CvsprdSpec> {
        PrdW::new(self, 0)
    }
}
#[doc = "desc CVSPRD\n\nYou can [`read`](crate::Reg::read) this register and get [`cvsprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvsprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvsprdSpec;
impl crate::RegisterSpec for CvsprdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cvsprd::R`](R) reader structure"]
impl crate::Readable for CvsprdSpec {}
#[doc = "`write(|w| ..)` method takes [`cvsprd::W`](W) writer structure"]
impl crate::Writable for CvsprdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVSPRD to value 0x0f"]
impl crate::Resettable for CvsprdSpec {
    const RESET_VALUE: u16 = 0x0f;
}
