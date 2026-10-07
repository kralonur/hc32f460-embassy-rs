#[doc = "Register `CNTBR` reader"]
pub type R = crate::R<CntbrSpec>;
#[doc = "Register `CNTBR` writer"]
pub type W = crate::W<CntbrSpec>;
#[doc = "Field `CNTB` reader - desc CNTB"]
pub type CntbR = crate::FieldReader<u16>;
#[doc = "Field `CNTB` writer - desc CNTB"]
pub type CntbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CNTB"]
    #[inline(always)]
    pub fn cntb(&self) -> CntbR {
        CntbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CNTB"]
    #[inline(always)]
    pub fn cntb(&mut self) -> CntbW<'_, CntbrSpec> {
        CntbW::new(self, 0)
    }
}
#[doc = "desc CNTBR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntbrSpec;
impl crate::RegisterSpec for CntbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntbr::R`](R) reader structure"]
impl crate::Readable for CntbrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntbr::W`](W) writer structure"]
impl crate::Writable for CntbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTBR to value 0"]
impl crate::Resettable for CntbrSpec {}
